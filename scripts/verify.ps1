# Verification script for Secure Vault project (PowerShell)

$ErrorActionPreference = "Stop"

Write-Host "🔍 Verifying Secure Vault Project..." -ForegroundColor Cyan
Write-Host ""

# Check Rust installation
Write-Host "✓ Checking Rust installation..." -ForegroundColor Green
rustc --version
cargo --version
Write-Host ""

# Format check
Write-Host "✓ Checking code formatting..." -ForegroundColor Green
cargo fmt --all -- --check
Write-Host ""

# Clippy lints
Write-Host "✓ Running clippy lints..." -ForegroundColor Green
cargo clippy --workspace --all-targets -- -D warnings
Write-Host ""

# Run tests
Write-Host "✓ Running test suite..." -ForegroundColor Green
cargo test --workspace
Write-Host ""

# Security audit
Write-Host "✓ Running security audit..." -ForegroundColor Green
if (Get-Command cargo-audit -ErrorAction SilentlyContinue) {
    cargo audit
} else {
    Write-Host "⚠️  cargo-audit not installed, skipping..." -ForegroundColor Yellow
    Write-Host "   Install with: cargo install cargo-audit"
}
Write-Host ""

# Build release
Write-Host "✓ Building release binary..." -ForegroundColor Green
cargo build --release --quiet
Write-Host ""

# Check binary
Write-Host "✓ Verifying binary..." -ForegroundColor Green
if (Test-Path "./target/release/nullcrypt.exe") {
    & "./target/release/nullcrypt.exe" --help | Select-Object -First 1
}
Write-Host ""

Write-Host "✅ All checks passed!" -ForegroundColor Green
Write-Host ""
Write-Host "📦 Release binary location:" -ForegroundColor Cyan
if (Test-Path "./target/release/nullcrypt.exe") {
    Write-Host "   ./target/release/nullcrypt.exe"
}
