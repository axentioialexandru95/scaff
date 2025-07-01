# Contributing to Scaff

Thank you for your interest in contributing to Scaff! This guide will help you get started with contributing to the project.

## Table of Contents
- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Making Changes](#making-changes)
- [Testing](#testing)
- [Submitting Changes](#submitting-changes)
- [Contribution Types](#contribution-types)
- [Development Guidelines](#development-guidelines)
- [Community](#community)

## Code of Conduct

By participating in this project, you agree to abide by our Code of Conduct:

- **Be respectful**: Treat everyone with respect and kindness
- **Be inclusive**: Welcome people of all backgrounds and experience levels
- **Be collaborative**: Work together constructively
- **Be helpful**: Support others in learning and growing
- **Be professional**: Maintain appropriate behavior in all interactions

## Getting Started

### Prerequisites

Before contributing, make sure you have:

- **Rust 1.70+**: Install from [rustup.rs](https://rustup.rs/)
- **Git**: For version control
- **Code editor**: VS Code, Vim, or your preferred editor
- **Basic Rust knowledge**: Understanding of Rust syntax and concepts

### First Steps

1. **Read the documentation**
   - [README.md](README.md) - Project overview and usage
   - [docs/SCAFF_FORMAT.md](docs/SCAFF_FORMAT.md) - Scaff file format
   - [docs/TEMPLATES.md](docs/TEMPLATES.md) - Template system

2. **Explore the codebase**
   - Browse the source code in `src/`
   - Run the existing tests: `cargo test`
   - Try the CLI commands: `cargo run -- --help`

3. **Check existing issues**
   - Look for "good first issue" labels
   - Read through open issues and discussions
   - Ask questions if anything is unclear

## Development Setup

### 1. Fork and Clone

```bash
# Fork the repository on GitHub, then clone your fork
git clone https://github.com/YOUR_USERNAME/scaff.git
cd scaff

# Add the original repository as upstream
git remote add upstream https://github.com/original/scaff.git
```

### 2. Build and Test

```bash
# Build the project
cargo build

# Run tests
cargo test

# Run integration tests
cargo test --test integration_tests

# Check code formatting
cargo fmt -- --check

# Run linting
cargo clippy -- -D warnings
```

### 3. Set Up Development Environment

```bash
# Install useful development tools
cargo install cargo-watch    # Auto-rebuild on file changes
cargo install cargo-expand   # Expand macros
cargo install cargo-outdated # Check for outdated dependencies

# Set up pre-commit hooks (optional but recommended)
echo "cargo fmt && cargo clippy -- -D warnings && cargo test" > .git/hooks/pre-commit
chmod +x .git/hooks/pre-commit
```

## Making Changes

### 1. Create a Branch

```bash
# Create and switch to a new branch
git checkout -b feature/your-feature-name

# or for bug fixes
git checkout -b fix/bug-description
```

### 2. Make Your Changes

- Write clean, well-documented code
- Follow Rust conventions and best practices
- Add tests for new functionality
- Update documentation as needed

### 3. Test Your Changes

```bash
# Run all tests
cargo test

# Test specific modules
cargo test scanner::tests
cargo test generator::tests

# Run with verbose output
cargo test -- --nocapture

# Test the CLI manually
cargo run -- scan --language rust
cargo run -- list
```

## Testing

### Running Tests

```bash
# All tests
cargo test

# Unit tests only
cargo test --lib

# Integration tests only
cargo test --test integration_tests

# Specific test
cargo test test_scan_rust_files

# With output
cargo test -- --nocapture
```

### Writing Tests

#### Unit Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_your_function() -> Result<(), Box<dyn std::error::Error>> {
        // Arrange
        let temp_dir = TempDir::new()?;
        
        // Act
        let result = your_function(&temp_dir.path());
        
        // Assert
        assert_eq!(result.len(), 1);
        assert!(result[0].contains("expected"));
        
        Ok(())
    }
}
```

#### Integration Tests
```rust
use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

#[test]
fn test_cli_command() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("scaff")?;
    
    cmd.arg("scan")
       .arg("--language")
       .arg("rust")
       .assert()
       .success()
       .stdout(predicate::str::contains("Scan Results"));
    
    Ok(())
}
```

### Test Guidelines

1. **Write tests for new features**: All new functionality should have tests
2. **Test edge cases**: Include tests for error conditions and boundary cases
3. **Use meaningful names**: Test names should describe what they're testing
4. **Keep tests independent**: Tests should not depend on each other
5. **Use temporary directories**: Use `tempfile::TempDir` for file system tests

## Submitting Changes

### 1. Ensure Quality

```bash
# Format code
cargo fmt

# Fix linting issues
cargo clippy --fix --allow-dirty

# Run all tests
cargo test

# Check for outdated dependencies
cargo outdated
```

### 2. Commit Changes

```bash
# Stage your changes
git add .

# Commit with a descriptive message
git commit -m "feat: add support for Python class detection

- Implement Python AST parsing for class definitions
- Add tests for Python scanning functionality
- Update documentation with Python examples"
```

#### Commit Message Format

Use the [Conventional Commits](https://www.conventionalcommits.org/) format:

- `feat:` - New features
- `fix:` - Bug fixes
- `docs:` - Documentation changes
- `test:` - Adding or updating tests
- `refactor:` - Code refactoring
- `perf:` - Performance improvements
- `chore:` - Maintenance tasks

### 3. Push and Create PR

```bash
# Push to your fork
git push origin feature/your-feature-name

# Create a pull request on GitHub
# Include a clear description of your changes
```

### Pull Request Guidelines

**Title**: Use a clear, descriptive title

**Description**: Include:
- What changes you made and why
- How to test the changes
- Any breaking changes
- Related issues (use "Fixes #123" to auto-close issues)

**Checklist**:
- [ ] Tests pass (`cargo test`)
- [ ] Code is formatted (`cargo fmt`)
- [ ] No linting errors (`cargo clippy`)
- [ ] Documentation updated
- [ ] Commit messages follow convention

## Contribution Types

### 🐛 Bug Fixes

1. **Reproduce the bug**: Create a minimal test case
2. **Write a failing test**: Capture the bug in a test
3. **Fix the issue**: Make the minimal change to fix the bug
4. **Verify the fix**: Ensure the test passes and no regressions

### ✨ New Features

1. **Discuss first**: Open an issue to discuss the feature
2. **Design the API**: Plan the interface and behavior
3. **Implement incrementally**: Break into smaller, reviewable chunks
4. **Add comprehensive tests**: Test the happy path and edge cases
5. **Update documentation**: Include examples and usage instructions

### 🌍 Language Support

Adding support for a new programming language:

1. **Add tree-sitter dependency**: Update `Cargo.toml`
2. **Extend language config**: Add to `SUPPORTED_LANGUAGES`
3. **Implement parsing logic**: Add language-specific extraction
4. **Create templates**: Add code generation templates
5. **Write tests**: Test scanning and generation
6. **Update documentation**: Add language to supported list

### 📚 Documentation

1. **Keep it current**: Update docs when making changes
2. **Add examples**: Include practical, working examples
3. **Be clear and concise**: Use simple language and good structure
4. **Test examples**: Ensure code examples actually work

### 🧪 Tests

1. **Improve coverage**: Add tests for untested code
2. **Test edge cases**: Cover error conditions and boundary cases
3. **Performance tests**: Add benchmarks for critical paths
4. **Integration tests**: Test end-to-end workflows

## Development Guidelines

### Code Style

- **Follow Rust conventions**: Use `cargo fmt` and `cargo clippy`
- **Use meaningful names**: Variables, functions, and types should be descriptive
- **Write documentation**: Document public APIs with doc comments
- **Handle errors properly**: Use `Result` types and proper error handling
- **Keep functions small**: Aim for single responsibility

### Architecture Principles

1. **Modularity**: Keep components loosely coupled
2. **Extensibility**: Make it easy to add new languages and features
3. **Performance**: Optimize for speed and memory usage
4. **Reliability**: Handle errors gracefully and provide good error messages
5. **Usability**: Design intuitive CLI interfaces

### Adding Dependencies

Before adding new dependencies:

1. **Check if necessary**: Can you implement it yourself simply?
2. **Evaluate alternatives**: Choose well-maintained, popular crates
3. **Consider size**: Avoid heavy dependencies for simple needs
4. **Check licensing**: Ensure compatible with MIT license

### Performance Considerations

- **Profile before optimizing**: Use `cargo bench` and profiling tools
- **Optimize hot paths**: Focus on frequently executed code
- **Memory usage**: Be mindful of allocations and data structures
- **I/O efficiency**: Batch file operations when possible

## Community

### Getting Help

- **GitHub Issues**: Ask questions and report bugs
- **Discussions**: Join conversations about features and ideas
- **Documentation**: Check existing docs first
- **Code Review**: Learn from feedback on your PRs

### Helping Others

- **Review PRs**: Help review other contributors' code
- **Answer questions**: Help newcomers in issues and discussions
- **Improve documentation**: Fix typos and add examples
- **Share knowledge**: Write blog posts or tutorials

### Communication

- **Be respectful**: Treat everyone with kindness
- **Be patient**: Remember that people have different experience levels
- **Be constructive**: Provide helpful feedback and suggestions
- **Be responsive**: Reply to questions and feedback promptly

## Recognition

We appreciate all contributions! Contributors will be:

- Listed in the project's contributors
- Acknowledged in release notes for significant contributions
- Invited to become maintainers for sustained contributions

## Questions?

If you have questions about contributing:

1. **Check existing issues**: Someone may have asked the same question
2. **Open a new issue**: Use the "question" template
3. **Join discussions**: Participate in GitHub Discussions
4. **Read the docs**: Check README and docs/ directory

Thank you for contributing to Scaff! 🚀 