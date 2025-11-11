# Contributing to Shroud Protocol

Thank you for your interest in contributing to Shroud! This document provides guidelines for contributing to the Rust program implementation.

## Development Setup

### Prerequisites

- Rust 1.75.0 or later
- Solana CLI 1.17.0 or later
- Anchor 0.29.0 or later
- Git

### Environment Setup

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Solana
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"

# Install Anchor
cargo install --git https://github.com/coral-xyz/anchor avm --locked --force
avm install latest
avm use latest

# Clone the repository
git clone https://github.com/ShroudDotFun/Shroud-Protocol-Rust
cd Shroud-Protocol-Rust

# Build the program
anchor build

# Run tests
anchor test
```

## Code Guidelines

### Rust Style

- Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `rustfmt` for code formatting
- Use `clippy` for linting
- Write comprehensive documentation comments

### Security

- All cryptographic operations must be reviewed
- Add tests for edge cases
- Validate all inputs
- Handle errors explicitly
- Avoid panics in production code

### Testing

- Write unit tests for all functions
- Add integration tests for instructions
- Include property-based tests for cryptographic functions
- Maintain >80% code coverage

## Pull Request Process

1. **Fork & Branch**: Create a feature branch from `main`
2. **Develop**: Make your changes with clear, atomic commits
3. **Test**: Ensure all tests pass (`anchor test`)
4. **Document**: Update README and docs as needed
5. **Submit**: Open a PR with a clear description

### PR Requirements

- [ ] Tests pass locally
- [ ] Code follows style guidelines
- [ ] Documentation updated
- [ ] No compiler warnings
- [ ] Clippy checks pass
- [ ] Security implications reviewed

## Areas for Contribution

### High Priority

- Zero-knowledge proof system optimization
- Merkle tree implementation for nullifier sets
- Gas optimization and compute unit reduction
- Cross-program invocation (CPI) interfaces

### Medium Priority

- Additional privacy features
- Performance benchmarking
- Documentation improvements
- Example integrations

### Always Welcome

- Bug fixes
- Test coverage improvements
- Documentation typos
- Code cleanup and refactoring

## Cryptographic Contributions

Contributions to privacy/cryptographic components require:

1. **Academic References**: Cite papers for algorithms
2. **Security Analysis**: Explain threat model and guarantees
3. **Test Vectors**: Include known-answer tests
4. **Peer Review**: Get review from cryptography experts

## Questions?

- Open a GitHub Discussion for questions
- Join our community channels
- Check existing issues and PRs

## Code of Conduct

Be respectful, inclusive, and professional. We're building privacy technology for everyone.

---

Thank you for contributing to privacy on Solana! 🔒

