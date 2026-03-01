//! Vault engine for managing encrypted containers

use crypto_core::{
    decrypt, derive_key, encrypt, generate_nonce, generate_salt, CryptoError, KdfParams,
    NONCE_SIZE, SALT_SIZE,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::Path;
use thiserror::Error;
use tracing::{debug, info};
use zeroize::Zeroize;

pub const MAGIC_BYTES: &[u8; 8] = b"SECVAULT";
pub const VERSION: u32 = 1;

#[derive(Error, Debug)]
pub enum VaultError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    #[error("Crypto error: {0}")]
    Crypto(#[from] CryptoError),
    #[error("Serialization error: {0}")]
    Serialization(String),
    #[error("Invalid vault format")]
    InvalidFormat,
    #[error("Unsupported vault version: {0}")]
    UnsupportedVersion(u32),
    #[error("File not found in vault: {0}")]
    FileNotFound(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VaultMetadata {
    pub version: u32,
    pub kdf_params: KdfParams,
    pub created_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileEntry {
    pub name: String,
    pub size: u64,
    pub data: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct VaultData {
    metadata: VaultMetadata,
    files: HashMap<String, FileEntry>,
}

/// Binary container format:
/// - Magic bytes (8 bytes): "SECVAULT"
/// - Version (4 bytes): u32
/// - Salt (32 bytes): for Argon2id
/// - Nonce (12 bytes): for AES-GCM
/// - Ciphertext length (8 bytes): u64
/// - Ciphertext (variable): encrypted VaultData + auth tag
#[derive(Debug)]
pub struct VaultContainer {
    magic: [u8; 8],
    version: u32,
    salt: [u8; SALT_SIZE],
    nonce: [u8; NONCE_SIZE],
    ciphertext: Vec<u8>,
}

impl VaultContainer {
    pub(crate) fn new(data: &VaultData, password: &[u8]) -> Result<Self, VaultError> {
        let salt = generate_salt();
        let nonce = generate_nonce();

        let key = derive_key(password, &salt, &data.metadata.kdf_params)?;

        let plaintext =
            bincode::serialize(data).map_err(|e| VaultError::Serialization(e.to_string()))?;

        let ciphertext = encrypt(&key, &nonce, &plaintext)?;

        Ok(Self {
            magic: *MAGIC_BYTES,
            version: VERSION,
            salt,
            nonce,
            ciphertext,
        })
    }

    pub fn write_to_file(&self, path: &Path) -> Result<(), VaultError> {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)?;

        file.write_all(&self.magic)?;
        file.write_all(&self.version.to_le_bytes())?;
        file.write_all(&self.salt)?;
        file.write_all(&self.nonce)?;
        file.write_all(&(self.ciphertext.len() as u64).to_le_bytes())?;
        file.write_all(&self.ciphertext)?;
        file.sync_all()?;

        info!("Vault written to {:?}", path);
        Ok(())
    }

    pub fn read_from_file(path: &Path) -> Result<Self, VaultError> {
        let mut file = File::open(path)?;

        let mut magic = [0u8; 8];
        file.read_exact(&mut magic)?;
        if &magic != MAGIC_BYTES {
            return Err(VaultError::InvalidFormat);
        }

        let mut version_bytes = [0u8; 4];
        file.read_exact(&mut version_bytes)?;
        let version = u32::from_le_bytes(version_bytes);

        if version != VERSION {
            return Err(VaultError::UnsupportedVersion(version));
        }

        let mut salt = [0u8; SALT_SIZE];
        file.read_exact(&mut salt)?;

        let mut nonce = [0u8; NONCE_SIZE];
        file.read_exact(&mut nonce)?;

        let mut len_bytes = [0u8; 8];
        file.read_exact(&mut len_bytes)?;
        let ciphertext_len = u64::from_le_bytes(len_bytes) as usize;

        let mut ciphertext = vec![0u8; ciphertext_len];
        file.read_exact(&mut ciphertext)?;

        debug!("Vault read from {:?}", path);
        Ok(Self {
            magic,
            version,
            salt,
            nonce,
            ciphertext,
        })
    }

    pub(crate) fn decrypt(&self, password: &[u8]) -> Result<VaultData, VaultError> {
        // First, we need to extract KDF params from a test decryption
        // In practice, we store KDF params in plaintext header or use fixed params
        let kdf_params = KdfParams::default();

        let key = derive_key(password, &self.salt, &kdf_params)?;
        let plaintext = decrypt(&key, &self.nonce, &self.ciphertext)?;

        let data: VaultData = bincode::deserialize(&plaintext)
            .map_err(|e| VaultError::Serialization(e.to_string()))?;

        Ok(data)
    }
}

pub struct Vault {
    data: VaultData,
    password: Vec<u8>,
}

impl Vault {
    pub fn create(password: Vec<u8>) -> Self {
        let metadata = VaultMetadata {
            version: VERSION,
            kdf_params: KdfParams::default(),
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        };

        Self {
            data: VaultData {
                metadata,
                files: HashMap::new(),
            },
            password,
        }
    }

    pub fn open(path: &Path, password: Vec<u8>) -> Result<Self, VaultError> {
        let container = VaultContainer::read_from_file(path)?;
        let data = container.decrypt(&password)?;

        Ok(Self { data, password })
    }

    pub fn save(&self, path: &Path) -> Result<(), VaultError> {
        let container = VaultContainer::new(&self.data, &self.password)?;
        container.write_to_file(path)?;
        Ok(())
    }

    pub fn add_file(&mut self, name: String, data: Vec<u8>) {
        let size = data.len() as u64;
        let entry = FileEntry {
            name: name.clone(),
            size,
            data,
        };
        self.data.files.insert(name, entry);
        info!("Added file to vault");
    }

    pub fn get_file(&self, name: &str) -> Result<&FileEntry, VaultError> {
        self.data
            .files
            .get(name)
            .ok_or_else(|| VaultError::FileNotFound(name.to_string()))
    }

    pub fn list_files(&self) -> Vec<&FileEntry> {
        self.data.files.values().collect()
    }

    pub fn remove_file(&mut self, name: &str) -> Result<(), VaultError> {
        self.data
            .files
            .remove(name)
            .ok_or_else(|| VaultError::FileNotFound(name.to_string()))?;
        info!("Removed file from vault");
        Ok(())
    }
}

impl Drop for Vault {
    fn drop(&mut self) {
        self.password.zeroize();
        debug!("Vault password zeroized");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_vault_create_and_save() {
        let temp_file = NamedTempFile::new().unwrap();
        let password = b"test_password".to_vec();

        let mut vault = Vault::create(password.clone());
        vault.add_file("test.txt".to_string(), b"Hello, World!".to_vec());

        vault.save(temp_file.path()).unwrap();

        let loaded_vault = Vault::open(temp_file.path(), password).unwrap();
        let file = loaded_vault.get_file("test.txt").unwrap();

        assert_eq!(file.data, b"Hello, World!");
    }

    #[test]
    fn test_wrong_password() {
        let temp_file = NamedTempFile::new().unwrap();
        let password = b"correct_password".to_vec();

        let vault = Vault::create(password);
        vault.save(temp_file.path()).unwrap();

        let result = Vault::open(temp_file.path(), b"wrong_password".to_vec());
        assert!(result.is_err());
    }
}
