use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use indicatif::{ProgressBar, ProgressStyle};
use std::fs;
use std::path::PathBuf;
use tracing::info;
use tracing_subscriber::EnvFilter;
use vault_engine::Vault;

#[derive(Parser)]
#[command(name = "nullcrypt")]
#[command(about = "Secure encrypted vault for removable media", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new vault
    Create {
        /// Path to vault file
        #[arg(value_name = "VAULT_PATH")]
        path: PathBuf,
    },
    /// Add files to vault
    Pack {
        /// Path to vault file
        #[arg(value_name = "VAULT_PATH")]
        vault_path: PathBuf,
        /// Files to add
        #[arg(value_name = "FILES")]
        files: Vec<PathBuf>,
    },
    /// Extract files from vault
    Unpack {
        /// Path to vault file
        #[arg(value_name = "VAULT_PATH")]
        vault_path: PathBuf,
        /// Output directory
        #[arg(short, long, value_name = "DIR")]
        output: PathBuf,
    },
    /// List vault contents
    List {
        /// Path to vault file
        #[arg(value_name = "VAULT_PATH")]
        vault_path: PathBuf,
    },
    /// Change vault password
    ChangePassword {
        /// Path to vault file
        #[arg(value_name = "VAULT_PATH")]
        vault_path: PathBuf,
    },
    /// Verify vault integrity
    Verify {
        /// Path to vault file
        #[arg(value_name = "VAULT_PATH")]
        vault_path: PathBuf,
    },
}

fn read_password(prompt: &str) -> Result<Vec<u8>> {
    let password = rpassword::prompt_password(prompt).context("Failed to read password")?;
    Ok(password.into_bytes())
}

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive(tracing::Level::INFO.into()))
        .init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Create { path } => {
            info!("Creating new vault at {:?}", path);
            let password = read_password("Enter password: ")?;
            let confirm = read_password("Confirm password: ")?;

            if password != confirm {
                anyhow::bail!("Passwords do not match");
            }

            let vault = Vault::create(password);
            vault.save(&path)?;
            println!("✓ Vault created successfully");
        }

        Commands::Pack { vault_path, files } => {
            info!("Adding files to vault");
            let password = read_password("Enter vault password: ")?;
            let mut vault = Vault::open(&vault_path, password)?;

            let pb = ProgressBar::new(files.len() as u64);
            pb.set_style(
                ProgressStyle::default_bar()
                    .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} {msg}")
                    .unwrap()
                    .progress_chars("#>-"),
            );

            for file_path in files {
                let filename = file_path
                    .file_name()
                    .context("Invalid filename")?
                    .to_string_lossy()
                    .to_string();

                pb.set_message(format!("Adding {}", filename));

                let data = fs::read(&file_path)
                    .with_context(|| format!("Failed to read {:?}", file_path))?;

                vault.add_file(filename.clone(), data);
                pb.inc(1);
            }

            pb.finish_with_message("Saving vault...");
            vault.save(&vault_path)?;
            println!("✓ Vault updated successfully");
        }

        Commands::Unpack { vault_path, output } => {
            info!("Extracting files from vault");
            let password = read_password("Enter vault password: ")?;
            let vault = Vault::open(&vault_path, password)?;

            fs::create_dir_all(&output)?;

            let files = vault.list_files();
            let pb = ProgressBar::new(files.len() as u64);
            pb.set_style(
                ProgressStyle::default_bar()
                    .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} {msg}")
                    .unwrap()
                    .progress_chars("#>-"),
            );

            for file in files {
                pb.set_message(format!("Extracting {}", file.name));
                let out_path = output.join(&file.name);
                fs::write(&out_path, &file.data)
                    .with_context(|| format!("Failed to write {:?}", out_path))?;
                pb.inc(1);
            }

            pb.finish_with_message("Complete!");
            println!("✓ All files extracted to {:?}", output);
        }

        Commands::List { vault_path } => {
            let password = read_password("Enter vault password: ")?;
            let vault = Vault::open(&vault_path, password)?;

            println!("\nVault contents:");
            println!("{:<40} {:>12}", "Name", "Size");
            println!("{}", "-".repeat(54));

            for file in vault.list_files() {
                println!("{:<40} {:>12}", file.name, format_size(file.size));
            }
        }

        Commands::ChangePassword { vault_path } => {
            info!("Changing vault password");
            let old_password = read_password("Enter current password: ")?;
            let mut vault = Vault::open(&vault_path, old_password)?;

            let new_password = read_password("Enter new password: ")?;
            let confirm = read_password("Confirm new password: ")?;

            if new_password != confirm {
                anyhow::bail!("Passwords do not match");
            }

            if new_password.is_empty() {
                anyhow::bail!("Password cannot be empty");
            }

            vault.change_password(new_password);
            vault.save(&vault_path)?;
            println!("✓ Password changed successfully");
        }

        Commands::Verify { vault_path } => {
            info!("Verifying vault integrity");

            match Vault::verify(&vault_path) {
                Ok(info) => {
                    println!("✓ Vault integrity check passed");
                    println!("\nVault Information:");
                    println!("  Format version: {}", info.version);
                    println!("  Created: {}", format_timestamp(info.created_at));
                    println!("  KDF parameters:");
                    println!("    Memory: {} MB", info.kdf_params.memory_cost_kb / 1024);
                    println!("    Iterations: {}", info.kdf_params.time_cost);
                    println!("    Parallelism: {}", info.kdf_params.parallelism);
                    println!("  Encrypted data size: {} bytes", info.ciphertext_size);
                }
                Err(e) => {
                    anyhow::bail!("Vault verification failed: {}", e);
                }
            }
        }
    }

    Ok(())
}

fn format_size(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB"];
    let mut size = bytes as f64;
    let mut unit_idx = 0;

    while size >= 1024.0 && unit_idx < UNITS.len() - 1 {
        size /= 1024.0;
        unit_idx += 1;
    }

    format!("{:.2} {}", size, UNITS[unit_idx])
}

fn format_timestamp(timestamp: u64) -> String {
    use std::time::{Duration, SystemTime, UNIX_EPOCH};

    let datetime = UNIX_EPOCH + Duration::from_secs(timestamp);
    match datetime.duration_since(SystemTime::now()) {
        Ok(_) => format!("{} seconds from now", timestamp),
        Err(e) => {
            let ago = e.duration().as_secs();
            if ago < 60 {
                format!("{} seconds ago", ago)
            } else if ago < 3600 {
                format!("{} minutes ago", ago / 60)
            } else if ago < 86400 {
                format!("{} hours ago", ago / 3600)
            } else {
                format!("{} days ago", ago / 86400)
            }
        }
    }
}
