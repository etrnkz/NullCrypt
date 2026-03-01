.PHONY: all build test lint fmt clean install audit

all: fmt lint test build

build:
	cargo build --release --workspace

test:
	cargo test --workspace --verbose

lint:
	cargo clippy --workspace --all-targets -- -D warnings

fmt:
	cargo fmt --all

fmt-check:
	cargo fmt --all -- --check

audit:
	cargo audit

clean:
	cargo clean

install:
	cargo install --path crates/cli

ci: fmt-check lint audit test

help:
	@echo "Available targets:"
	@echo "  all        - Format, lint, test, and build"
	@echo "  build      - Build release binaries"
	@echo "  test       - Run all tests"
	@echo "  lint       - Run clippy lints"
	@echo "  fmt        - Format code"
	@echo "  fmt-check  - Check code formatting"
	@echo "  audit      - Run security audit"
	@echo "  clean      - Clean build artifacts"
	@echo "  install    - Install CLI binary"
	@echo "  ci         - Run CI checks"
