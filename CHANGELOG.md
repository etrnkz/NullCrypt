# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Progress bars for pack and unpack operations
- Visual feedback for large file operations with indicatif

### Changed
- Replaced postcard with ciborium (CBOR) for serialization
- Ciborium has no unmaintained dependencies

## [0.1.0] - 2024-03-02

### Added
- Initial release of NullCrypt
- AES-256-GCM authenticated encryption
- Argon2id key derivation (64MB memory, 3 iterations)
- Password-protected vault creation
- Pack files into encrypted vaults
- Unpack files from vaults
- List vault contents
- Password change command
- Cross-platform support (Linux, macOS, Windows)
- Memory zeroization for all secrets
- Constant-time password verification
- CLI interface with secure password prompts

### Security
- Uses audited cryptographic libraries (aes-gcm, argon2)
- No plaintext ever written to disk
- Automatic secret cleanup on drop
- Side-channel attack mitigations

### Added
- Initial release
- AES-256-GCM authenticated encryption
- Argon2id key derivation
- Binary container format with versioning
- CLI for create, pack, unpack, list operations
- Memory zeroization for secrets
- Constant-time password verification
- Cross-platform support (Linux, macOS, Windows)
- Comprehensive test suite
- Security documentation and threat model
- CI/CD with GitHub Actions

### Security
- All cryptographic operations use audited libraries
- No plaintext written to disk
- Automatic secret zeroization on drop
- Side-channel attack mitigations

## [0.1.0] - TBD

Initial release.
