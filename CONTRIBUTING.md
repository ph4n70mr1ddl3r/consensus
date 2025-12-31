# Contributing to Poker Consensus Engine

Thank you for your interest in contributing to the Poker Consensus Engine! This document provides guidelines and instructions for contributing.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Workflow](#development-workflow)
- [Coding Standards](#coding-standards)
- [Testing Requirements](#testing-requirements)
- [Pull Request Process](#pull-request-process)
- [Documentation](#documentation)

## Code of Conduct

This project follows our [Code of Conduct](CODE_OF_CONDUCT.md). By participating, you are expected to uphold this code.

## Getting Started

### Prerequisites

- Rust 1.75+ with WASM target
- Git
- CMake, Clang, build essentials
- 4GB+ RAM (8GB+ recommended)

### Setting Up Development Environment

```bash
# Clone the repository
git clone https://github.com/poker-consensus/engine.git
cd engine

# Install dependencies
make install-deps

# Verify setup
make setup
```

### Finding Issues to Work On

- Look for issues tagged with `good first issue` or `help wanted`
- Check the [Story Backlog](_bmad-output/planning-artifacts/stories/poker-consensus-stories.md)
- Review open issues on GitHub

## Development Workflow

### 1. Create a Branch

```bash
# Create and switch to a new branch
git checkout -b feature/your-feature-name

# Or for bug fixes
git checkout -b fix/issue-description
```

### 2. Make Changes

Follow the [coding standards](#coding-standards) and make your changes.

### 3. Test Your Changes

```bash
# Run unit tests
make test-unit

# Run linter
make lint

# Run specific tests
cargo test --package pallet-poker
```

### 4. Commit Changes

We follow [Conventional Commits](https://www.conventionalcommits.org/):

```bash
# Stage changes
git add .

# Commit with conventional message
git commit -m "feat(poker): add hand evaluation logic"

# Or for fixes
git commit -m "fix(timestamp): resolve HLC clock drift issue"
```

### 5. Push and Create PR

```bash
# Push changes
git push origin feature/your-feature-name

# Create Pull Request on GitHub
```

## Coding Standards

### Rust Guidelines

1. **Follow rustfmt**: All code must pass `cargo fmt --check`
2. **Pass clippy**: All code must pass `cargo clippy -- -D warnings`
3. **Documentation**: Document public APIs with doc comments
4. **Error Handling**: Use proper error types (`Result`, `Option`)
5. **Testing**: Write tests for new functionality

### Code Style

```rust
/// Creates a new game with the specified parameters.
///
/// # Errors
///
/// Returns an error if the stake amount is below the minimum.
#[pallet::call]
impl<T: Config> Pallet<T> {
    pub fn create_game(
        origin: OriginFor<T>,
        min_stake: BalanceOf<T>,
    ) -> DispatchResult {
        // Implementation
    }
}
```

### Commit Message Format

```
<type>(<scope>): <description>

[optional body]

[optional footer]
```

Types:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Formatting changes
- `refactor`: Code restructuring
- `test`: Adding tests
- `chore`: Maintenance

## Testing Requirements

### Unit Tests

All new functionality must include unit tests:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_creation() {
        // Test implementation
    }
}
```

### Integration Tests

For complex interactions, add integration tests in `node/src/tests.rs`.

### Benchmarks

Performance-critical code should include benchmarks:

```rust
#[bench]
fn bench_game_creation(b: &mut Bencher) {
    b.iter(|| {
        // Benchmark implementation
    });
}
```

## Pull Request Process

### Before Submitting

1. Ensure all tests pass: `make test`
2. Ensure linting passes: `make lint`
3. Update documentation as needed
4. Squash commits into logical units
5. Write clear PR description

### PR Description Template

```markdown
## Description
Brief description of changes

## Related Issue
Fixes #[issue number]

## Type of Change
- [ ] Bug fix (non-breaking change)
- [ ] New feature (non-breaking change)
- [ ] Breaking change (fix or feature)
- [ ] Documentation update

## Testing
- [ ] Tests pass locally
- [ ] Added new tests
- [ ] Benchmarks included

## Checklist
- [ ] Code follows style guidelines
- [ ] Self-reviewed
- [ ] Documentation updated
- [ ] No new warnings
```

### Review Process

1. Automated checks run (CI/CD)
2. At least one maintainer reviews
3. Address feedback
4. Maintainer approves and merges

## Documentation

### Code Documentation

- Document all public types and functions
- Explain parameters and return values
- Include examples where helpful

### README Updates

Update README.md for:
- New features
- Changed configurations
- Updated prerequisites

### Changelog

Update [CHANGELOG.md](CHANGELOG.md) following [Keep a Changelog](https://keepachangelog.com/).

## Questions?

- Open an issue for questions
- Join our Discord community
- Email: dev@poker-consensus.engine

Thank you for contributing!
