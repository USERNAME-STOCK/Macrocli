# Contributing to Macrocli

Thank you for your interest in contributing to Macrocli. This document provides guidelines and instructions for contributing.

---

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [How to Contribute](#how-to-contribute)
- [Coding Standards](#coding-standards)
- [Testing Guidelines](#testing-guidelines)
- [Pull Request Process](#pull-request-process)
- [Reporting Bugs](#reporting-bugs)
- [Suggesting Features](#suggesting-features)

---

## Code of Conduct

This project adheres to a Code of Conduct. By participating, you are expected to uphold this code. Report unacceptable behavior to project maintainers.

See [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md) for details.

---

## Getting Started

### Prerequisites

Required tools:

- **Rust**: Version 1.70 or higher
  ```bash
  rustup update stable
  ```

- **Node.js**: Version 18 or higher
  ```bash
  node --version
  npm --version
  ```

- **Git**: For version control
  ```bash
  git --version
  ```

- **USB Macropad Device**: For testing (optional but recommended)

### Fork and Clone

1. Fork the repository on GitHub
2. Clone your fork locally:
   ```bash
   git clone https://github.com/YOUR_USERNAME/Macrocli.git
   cd Macrocli
   ```

3. Add upstream remote:
   ```bash
   git remote add upstream https://github.com/USERNAME-STOCK/Macrocli.git
   ```

---

## Development Setup

### Backend Setup (Rust)

```bash
cd Macrocli/
cargo build
cargo test
```

### Frontend Setup (React/TypeScript)

```bash
cd Macrocli/Webapp/
npm install
npm run dev  # Development server
```

### Full Integration Build

```bash
# Build backend
cd Macrocli/
cargo build --release

# Build frontend
cd Webapp/
npm run build
cd ..

# Start integrated server
./target/release/macrocli serve --port 8080
```

---

## How to Contribute

### Types of Contributions

We welcome:

- Bug fixes
- New features
- Documentation improvements
- Test coverage
- UI/UX enhancements
- Code refactoring
- Translations (future)

### Contribution Workflow

1. **Check existing issues** - Search for existing issues or create new one
2. **Discuss first** - For large changes, discuss in issue first
3. **Create branch** - Use descriptive branch names
4. **Make changes** - Follow coding standards
5. **Test thoroughly** - Ensure all tests pass
6. **Submit PR** - Create pull request with clear description

---

## Coding Standards

### Rust Code Style

Follow standard Rust conventions:

```bash
# Format code
cargo fmt

# Lint code
cargo clippy -- -W clippy::all

# Check for issues
cargo check
```

#### Rust Guidelines

- Use descriptive variable names
- Add documentation comments (`///`) for public APIs
- Implement proper error handling (no unwrap in production code)
- Use `Result<T, E>` for fallible operations
- Avoid unsafe code unless absolutely necessary
- Run `cargo clippy` before committing

**Example:**

```rust
/// Validates a macropad configuration against device capabilities.
///
/// # Arguments
/// * `config` - The configuration to validate
/// * `device` - The target device information
///
/// # Returns
/// * `Ok(())` if validation passes
/// * `Err(ValidationError)` if validation fails
pub fn validate_config(config: &Config, device: &Device) -> Result<(), ValidationError> {
    // Implementation
}
```

### TypeScript/React Code Style

```bash
# Lint and format
npm run lint
npm run format
```

#### TypeScript Guidelines

- Use TypeScript strict mode
- Define proper interfaces/types
- Use functional components with hooks
- Follow React best practices
- Add JSDoc comments for complex functions

**Example:**

```typescript
/**
 * Validates configuration data before sending to API
 * @param config - Configuration object to validate
 * @returns True if valid, false otherwise
 */
function validateConfiguration(config: MacropadConfig): boolean {
  // Implementation
}
```

### Commit Message Format

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>(<scope>): <subject>

<body>

<footer>
```

**Types:**
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting)
- `refactor`: Code refactoring
- `test`: Adding or updating tests
- `chore`: Maintenance tasks

**Examples:**

```bash
feat(api): add endpoint for batch device programming

fix(validation): correct layer count validation logic

docs(readme): update installation instructions

refactor(config): simplify configuration parser
```

---

## Testing Guidelines

### Running Tests

**Backend Tests:**
```bash
cd Macrocli/
cargo test
cargo test -- --nocapture  # Show output
```

**Frontend Tests:**
```bash
cd Macrocli/Webapp/
npm test
```

### Writing Tests

#### Rust Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_validation() {
        let config = create_test_config();
        assert!(validate_config(&config).is_ok());
    }

    #[test]
    fn test_invalid_config_rejected() {
        let invalid_config = create_invalid_config();
        assert!(validate_config(&invalid_config).is_err());
    }
}
```

#### TypeScript Tests

```typescript
describe('ConfigValidator', () => {
  it('should accept valid configuration', () => {
    const config = createValidConfig();
    expect(validateConfig(config)).toBe(true);
  });

  it('should reject invalid configuration', () => {
    const config = createInvalidConfig();
    expect(validateConfig(config)).toBe(false);
  });
});
```

### Test Coverage Goals

- Core validation logic: 90%+
- API endpoints: 80%+
- UI components: 70%+
- Overall: 75%+

---

## Pull Request Process

### Before Submitting

- [ ] Code follows style guidelines
- [ ] All tests pass (`cargo test` and `npm test`)
- [ ] No compiler warnings
- [ ] Documentation updated (if applicable)
- [ ] CHANGELOG.md updated (for notable changes)
- [ ] Commits follow conventional commit format

### PR Checklist

1. **Title**: Clear, descriptive title
2. **Description**:
   - What does this PR do?
   - Why is this change needed?
   - How was it tested?
   - Screenshots (for UI changes)
3. **Link to Issue**: Reference related issues
4. **Breaking Changes**: Clearly marked if applicable

### PR Template

```markdown
## Description
Brief description of changes

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update

## Testing
Describe how you tested these changes

## Checklist
- [ ] Code follows project style
- [ ] Tests added/updated
- [ ] Documentation updated
- [ ] No new warnings

## Screenshots (if applicable)
```

### Review Process

1. **Automated Checks**: CI/CD must pass
2. **Code Review**: At least one maintainer approval required
3. **Testing**: Changes tested on actual hardware (if applicable)
4. **Merge**: Squash and merge or rebase

---

## Reporting Bugs

### Before Reporting

1. Check existing issues
2. Try latest version
3. Reproduce with minimal example

### Bug Report Template

**Title**: Clear, specific title

**Description:**
- What happened?
- What did you expect?
- Steps to reproduce
- Environment details:
  - OS and version
  - Rust version: `rustc --version`
  - Node version: `node --version`
  - Device model

**Error Messages:**
```
Include full error messages or logs
```

**Additional Context:**
Screenshots, configuration files, etc.

---

## Suggesting Features

### Feature Request Template

**Title**: Concise feature description

**Problem Statement:**
What problem does this solve?

**Proposed Solution:**
How should it work?

**Alternatives Considered:**
What other approaches did you consider?

**Additional Context:**
Mockups, examples, use cases

---

## Documentation

### Documentation Standards

- Clear, concise language
- Include code examples
- Update relevant docs when changing features
- Use proper markdown formatting

### Documentation Locations

- **README.md**: Overview, quick start, basic usage
- **SECURITY.md**: Security features and policies
- **CONTRIBUTING.md**: This file
- **Code Comments**: Inline documentation
- **API Documentation**: Generated from code comments

---

## Recognition

Contributors will be:

- Added to contributors list
- Credited in CHANGELOG
- Mentioned in release notes (for significant contributions)

---

## Getting Help

Need help contributing?

- **Discussions**: Use GitHub Discussions for questions
- **Issues**: Report bugs via GitHub Issues
- **Email**: Contact maintainers directly for sensitive matters

---

## License

By contributing to Macrocli, you agree that your contributions will be licensed under the Creative Commons Attribution-ShareAlike 3.0 Unported License.

---

Thank you for contributing to Macrocli. Every contribution, no matter how small, makes a difference.
