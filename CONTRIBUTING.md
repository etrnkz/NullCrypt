# Contributing to Secure Vault

Thank you for your interest in contributing!

## Development Setup

1. Install Rust: https://rustup.rs/
2. Clone the repository
3. Run tests: `cargo test --workspace`
4. Run lints: `cargo clippy --workspace -- -D warnings`

## Code Standards

- All code must pass `cargo fmt` and `cargo clippy`
- Write tests for new functionality
- Update documentation for API changes
- Follow Rust API guidelines
- No unsafe code without thorough justification

## Security Guidelines

- Never log or print sensitive data (passwords, keys)
- Use zeroization for all secret material
- Prefer constant-time operations for crypto
- Document security assumptions
- Report vulnerabilities privately (see SECURITY.md)

## Pull Request Process

1. Create a feature branch from `develop`
2. Write clear commit messages
3. Add tests and documentation
4. Ensure CI passes
5. Request review from maintainers

## Commit Message Format

```
type(scope): brief description

Longer explanation if needed.

Fixes #123
```

Types: feat, fix, docs, style, refactor, test, chore

## Testing

```bash
# Unit tests
cargo test --workspace

# Integration tests
cargo test --workspace --test '*'

# With coverage
cargo tarpaulin --workspace
```

## License

By contributing, you agree that your contributions will be licensed under MIT OR Apache-2.0.
