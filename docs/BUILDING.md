# Building from Source

## Prerequisites

- Rust 1.70 or later
- Cargo (comes with Rust)
- Git

## Quick Start

```bash
# Clone repository
git clone https://github.com/etrnkz/secure-vault.git
cd secure-vault

# Build all crates
cargo build --release --workspace

# Run tests
cargo test --workspace

# Install CLI
cargo install --path crates/cli
```

## Development Build

```bash
# Debug build (faster compilation)
cargo build --workspace

# Run with logging
RUST_LOG=debug cargo run --bin vault-cli -- --help
```

## Release Build

```bash
# Optimized release build
cargo build --release --workspace

# Binary location
./target/release/vault-cli
```

## Cross-Compilation

### Linux to Windows

```bash
rustup target add x86_64-pc-windows-gnu
cargo build --release --target x86_64-pc-windows-gnu
```

### Linux to macOS

Requires osxcross toolchain.

```bash
rustup target add x86_64-apple-darwin
cargo build --release --target x86_64-apple-darwin
```

## Reproducible Builds

For reproducible builds, use Docker:

```bash
docker build -t secure-vault-builder .
docker run --rm -v $(pwd):/workspace secure-vault-builder
```

## Platform-Specific Notes

### Linux

No additional dependencies required.

### macOS

Xcode Command Line Tools required:
```bash
xcode-select --install
```

### Windows

MSVC or GNU toolchain required. Install via rustup:
```bash
rustup toolchain install stable-x86_64-pc-windows-msvc
```

## Verification

```bash
# Run all checks
make ci

# Individual checks
cargo fmt --check
cargo clippy --workspace -- -D warnings
cargo test --workspace
cargo audit
```

## Troubleshooting

### Linker Errors

Install required system libraries:
```bash
# Ubuntu/Debian
sudo apt-get install build-essential

# Fedora
sudo dnf install gcc

# macOS
xcode-select --install
```

### OpenSSL Errors

Not applicable - this project uses pure Rust crypto.

### Out of Memory

Reduce parallelism:
```bash
cargo build -j 2
```
