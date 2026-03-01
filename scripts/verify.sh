#!/bin/bash
# Verification script for Secure Vault project

set -e

echo "🔍 Verifying Secure Vault Project..."
echo ""

# Check Rust installation
echo "✓ Checking Rust installation..."
rustc --version
cargo --version
echo ""

# Format check
echo "✓ Checking code formatting..."
cargo fmt --all -- --check
echo ""

# Clippy lints
echo "✓ Running clippy lints..."
cargo clippy --workspace --all-targets -- -D warnings
echo ""

# Run tests
echo "✓ Running test suite..."
cargo test --workspace
echo ""

# Security audit
echo "✓ Running security audit..."
if command -v cargo-audit &> /dev/null; then
    cargo audit
else
    echo "⚠️  cargo-audit not installed, skipping..."
    echo "   Install with: cargo install cargo-audit"
fi
echo ""

# Build release
echo "✓ Building release binary..."
cargo build --release --quiet
echo ""

# Check binary
echo "✓ Verifying binary..."
./target/release/vault-cli --version || ./target/release/vault-cli.exe --version
echo ""

echo "✅ All checks passed!"
echo ""
echo "📦 Release binary location:"
if [ -f "./target/release/vault-cli" ]; then
    echo "   ./target/release/vault-cli"
elif [ -f "./target/release/vault-cli.exe" ]; then
    echo "   ./target/release/vault-cli.exe"
fi
