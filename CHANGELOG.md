# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Password change command (`change-password`)
- Test coverage for password change functionality

## [0.1.0] - 2024

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
