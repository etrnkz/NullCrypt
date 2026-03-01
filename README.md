# 🔐 Secure Vault - Encrypted Container for Removable Media

[![CI](https://github.com/etrnkz/secure-vault/workflows/CI/badge.svg)](https://github.com/etrnkz/secure-vault/actions)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)
[![Rust Version](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org)

A production-grade, cross-platform encrypted vault system for USB drives and SD cards built with Rust.

###  Features

-  **Military-grade encryption**: AES-256-GCM authenticated encryption
-  **Secure key derivation**: Argon2id with unique salts per vault
-  **Memory safety**: Automatic zeroization of secrets
-  **Side-channel protection**: Constant-time operations
-  **Cross-platform**: Linux, macOS, Windows support
-  **Audited primitives**: Uses well-vetted cryptographic libraries
-  **Zero dependencies**: No OpenSSL or system crypto libraries required
-  **Fast**: Optimized release builds with LTO

### Security Architecture

- No plaintext ever written to disk
- Versioned binary container format with integrity validation
- Brute-force mitigation through Argon2id parameters
- Structured logging without secret leakage
- Comprehensive threat modeling

### Installation

#### From Source

```bash
git clone https://github.com/etrnkz/NullCrypt.git
cd NullCrypt
cargo install --path crates/cli
```

#### From Crates.io (Coming Soon)

```bash
cargo install nullcrypt
```

#### Pre-built Binaries

Download from [GitHub Releases](https://github.com/etrnkz/NullCrypt/releases)

### Quick Start

```bash
# Create a new vault
nullcrypt create /media/usb/my-vault.vault

# Add files to vault
nullcrypt pack /media/usb/my-vault.vault file1.txt file2.pdf

# Extract files from vault
nullcrypt unpack /media/usb/my-vault.vault --output ./extracted/

# List vault contents
nullcrypt list /media/usb/my-vault.vault

# Change vault password
nullcrypt change-password /media/usb/my-vault.vault
```

## 🛠️ Development

```bash
# Run all checks
make ci

# Individual commands
cargo test --workspace          # Run tests
cargo clippy --workspace -- -D warnings  # Lint
cargo audit                     # Security audit
cargo fmt --all                 # Format code
cargo build --release           # Release build
```

See [Building from Source](docs/BUILDING.md) for more details.

### Documentation

- [Building from Source](docs/BUILDING.md) - Build instructions
- [Contributing](CONTRIBUTING.md) - How to contribute

### Contributing

Contributions are welcome! Please read the [Contributing Guide](CONTRIBUTING.md).

### Security

This project uses audited cryptographic libraries (aes-gcm, argon2) for all cryptographic operations. Always use strong passwords and keep backups of important data.

### License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Acknowledgments

Built with:
- [aes-gcm](https://github.com/RustCrypto/AEADs) - AES-256-GCM implementation
- [argon2](https://github.com/RustCrypto/password-hashes) - Argon2id key derivation
- [zeroize](https://github.com/RustCrypto/utils) - Secure memory zeroization

### Disclaimer

This software is provided "as is" without warranty. Always keep backups of important data.
