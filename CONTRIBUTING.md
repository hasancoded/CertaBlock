# Contributing to CertaBlock

Thank you for your interest in contributing to CertaBlock! This document provides guidelines for contributing to the project.

## Code of Conduct

By participating in this project, you agree to maintain a respectful and collaborative environment.

## How to Contribute

### Reporting Bugs

If you find a bug, please open an issue with:

- A clear, descriptive title
- Steps to reproduce the issue
- Expected vs. actual behavior
- Your environment (OS, Rust version, Node version)
- Any relevant logs or screenshots

### Suggesting Enhancements

Enhancement suggestions are welcome! Please open an issue with:

- A clear description of the proposed feature
- Use cases and benefits
- Any implementation ideas you may have

### Pull Requests

1. **Fork the repository** and create your branch from `main`
2. **Make your changes** following our coding standards
3. **Test your changes** thoroughly
4. **Update documentation** if needed
5. **Submit a pull request** with a clear description

## Development Setup

### Backend (Rust)

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build and test
cargo build
cargo test
cargo run
```

### Frontend (React)

```bash
cd dashboard
npm install
npm run dev
npm test
```

## Coding Standards

### Rust

- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Run `cargo fmt` before committing
- Ensure `cargo clippy` passes without warnings
- Add tests for new functionality

### JavaScript/React

- Use ES6+ syntax
- Follow React best practices
- Use functional components with hooks
- Add PropTypes or TypeScript types
- Write meaningful component and variable names

### General

- Write clear, concise commit messages
- Keep commits focused and atomic
- Comment complex logic
- Update documentation for API changes

## Project Structure

```
CertaBlock/
├── src/              # Rust backend
├── dashboard/        # React frontend
├── docs/             # Documentation
└── tests/            # Integration tests
```

## Testing

- Write unit tests for new features
- Ensure all tests pass before submitting PR
- Add integration tests for API endpoints
- Test UI changes in multiple browsers

## Documentation

- Update README.md for user-facing changes
- Update API.md for API changes
- Add inline code comments for complex logic
- Update Whitepaper.md for architectural changes

## Questions?

Feel free to open an issue for any questions or clarifications.

Thank you for contributing to CertaBlock!
