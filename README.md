# Scaff - Architecture in Your Pocket 🏗️

> A powerful CLI tool for extracting, saving, and replicating code architecture patterns across projects.

[![Build Status](https://img.shields.io/badge/build-passing-brightgreen.svg)]()
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)

## 🚀 Quick Start

```bash
# Scan your current codebase
scaff scan --language rust

# Save the pattern as a scaff
scaff save my-api-pattern --language rust

# Generate new code from your scaff
scaff generate my-api-pattern --output new-project

# Validate your codebase against a scaff
scaff validate my-api-pattern
```

## ✨ Main Features

### 🔍 **Intelligent Code Scanning**
- **Multi-language support**: Rust, JavaScript, TypeScript, Python, Java, Go, JSON, HTML, CSS
- **Pattern recognition**: Automatically detects classes, functions, structs, and implementations
- **Tree-sitter powered**: Uses robust parsing for accurate code analysis

### 💾 **Pattern Management**
- **Save patterns as scaffs**: Capture your architecture decisions in reusable templates
- **JSON-based storage**: Human-readable and version-controllable scaff files
- **Metadata tracking**: Timestamps, descriptions, and language information

### 🏗️ **Code Generation**
- **Template-based generation**: Handlebars-powered code templates for rapid scaffolding
- **Project scaffolding**: Generate complete project structures from saved patterns
- **Configuration files**: Auto-generate Cargo.toml, package.json, etc.

### ✅ **Architecture Validation**
- **Compliance checking**: Ensure your codebase follows established patterns
- **Missing component detection**: Identify gaps in your architecture
- **Suggestion system**: Get actionable recommendations for improvements

### 🔧 **Developer Experience**
- **Fast execution**: Optimized for speed and efficiency
- **Clear output**: Beautiful, informative command-line interface
- **Error handling**: Helpful error messages and recovery suggestions

## 🎯 Why Scaff?

**Take your architecture with you** - Instead of rebuilding project structures from scratch, extract proven patterns from existing codebases and replicate them instantly.

**Maintain consistency** - Ensure all team projects follow the same architectural standards with automated validation and suggestions.

**Speed up development** - Generate boilerplate code, project structures, and components based on your established patterns.

**Learn from codebases** - Understand complex project architectures through intelligent scanning and pattern visualization.

## 📦 Installation

### From Source
```bash
git clone https://github.com/yourusername/scaff.git
cd scaff
cargo build --release
./target/release/scaff --help
```

### From Cargo (Future)
```bash
cargo install scaff
```

## 🛠️ Usage

### Basic Commands

#### Scan Codebase
```bash
# Scan for Rust patterns
scaff scan --language rust

# Scan for JavaScript patterns
scaff scan --language javascript

# Scan all supported languages
scaff scan --language all
```

#### Save Patterns
```bash
# Save current codebase as a scaff
scaff save my-pattern --language rust

# The scaff will be saved to scaffs/my-pattern.json
```

#### List Scaffs
```bash
# List all available scaffs
scaff list
```

#### Generate Code
```bash
# Generate from a scaff
scaff generate my-pattern --output new-project

# Generate to current directory
scaff generate api-server --output .
```

#### Validate Architecture
```bash
# Check if codebase follows a scaff pattern
scaff validate my-pattern
```

## 🌍 Supported Languages

| Language   | Extensions  | Features Detected |
|------------|-------------|-------------------|
| Rust       | `.rs`       | structs, functions, implementations, modules |
| JavaScript | `.js`, `.jsx` | classes, functions, methods |
| TypeScript | `.ts`, `.tsx` | classes, functions, interfaces |
| Python     | `.py`, `.pyi` | classes, functions, methods |
| Java       | `.java`     | classes, methods, interfaces |
| Go         | `.go`       | types, functions, methods |
| JSON       | `.json`     | keys, structure |
| HTML       | `.html`, `.htm` | elements, structure |
| CSS        | `.css`      | selectors, rules |

## 📚 Examples

### Example 1: REST API Pattern

```bash
# 1. In your existing API project
scaff scan --language rust
scaff save rest-api --language rust

# 2. Generate new API from pattern
mkdir new-api && cd new-api
scaff generate rest-api --output .

# 3. Validate the generated code
scaff validate rest-api
```

### Example 2: Multi-language Project

```bash
# Scan a full-stack project
scaff scan --language all

# Save specific patterns
scaff save frontend-pattern --language javascript
scaff save backend-pattern --language rust

# Generate new full-stack project
mkdir new-project
cd new-project
scaff generate frontend-pattern --output frontend
scaff generate backend-pattern --output backend
```

### Example 3: Team Architecture Standards

```bash
# Save your team's standard architecture
scaff save company-standard --language rust

# Team members can validate their projects
cd team-project-1
scaff validate company-standard

# Output shows compliance status and suggestions
```

## 📋 Scaff Format

Scaffs are stored as JSON files with the following structure:

```json
{
  "name": "my-api-pattern",
  "description": "REST API with authentication",
  "language": "Rust",
  "created_at": "2024-01-01T00:00:00Z",
  "files": [
    {
      "path": "src/main.rs",
      "extension": "rs",
      "classes": [],
      "functions": ["main"],
      "structs": ["AppConfig"],
      "implementations": ["AppConfig"]
    },
    {
      "path": "src/auth.rs",
      "extension": "rs",
      "classes": [],
      "functions": ["authenticate", "generate_token"],
      "structs": ["User", "AuthRequest"],
      "implementations": ["User"]
    }
  ]
}
```

### Scaff Fields

- **name**: Unique identifier for the scaff
- **description**: Human-readable description of the pattern
- **language**: Primary programming language
- **created_at**: Timestamp of creation
- **files**: Array of file patterns with detected code elements

## 🎨 Templates

Scaff uses Handlebars templates for code generation. Templates support:

### Built-in Helpers
- `{{uppercase}}`: Convert to UPPERCASE
- `{{lowercase}}`: Convert to lowercase
- `{{pascal_case}}`: Convert to PascalCase
- `{{snake_case}}`: Convert to snake_case

### Template Variables
- `{{pattern.name}}`: Scaff name
- `{{pattern.description}}`: Scaff description
- `{{file.path}}`: File path
- `{{file.functions}}`: Array of functions
- `{{file.structs}}`: Array of structs

### Example Template (templates/rust_file.hbs)
```handlebars
{{#each file.structs}}
#[derive(Debug, Clone)]
pub struct {{pascal_case this}} {
    // TODO: Add fields
}
{{/each}}

{{#each file.functions}}
pub fn {{snake_case this}}() {
    // TODO: Implement {{this}}
}
{{/each}}
```

## 🤝 Contributing

We welcome contributions! Here's how to get started:

### Development Setup

1. **Clone the repository**
   ```bash
   git clone https://github.com/yourusername/scaff.git
   cd scaff
   ```

2. **Install Rust**
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

3. **Build and test**
   ```bash
   cargo build
   cargo test
   ```

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test module
cargo test scanner::tests

# Run integration tests
cargo test --test integration_tests

# Run with verbose output
cargo test -- --nocapture
```

### Code Style

- Follow Rust standard formatting: `cargo fmt`
- Lint with Clippy: `cargo clippy`
- Document public APIs with doc comments
- Write tests for new functionality

### Contribution Guidelines

1. **Fork the repository**
2. **Create a feature branch** (`git checkout -b feature/amazing-feature`)
3. **Write tests** for your changes
4. **Ensure all tests pass** (`cargo test`)
5. **Commit your changes** (`git commit -m 'Add amazing feature'`)
6. **Push to the branch** (`git push origin feature/amazing-feature`)
7. **Open a Pull Request**

### Types of Contributions

- 🐛 **Bug fixes**: Help us squash bugs
- ✨ **New features**: Add new language support, improve patterns
- 📚 **Documentation**: Improve guides, examples, and API docs
- 🧪 **Tests**: Add test coverage for edge cases
- 🎨 **Templates**: Contribute reusable code templates
- 🌍 **Language support**: Add parsers for new programming languages

### Development Guidelines

- **Small, focused PRs**: Easier to review and merge
- **Clear commit messages**: Describe what and why, not just what
- **Test your changes**: Include unit and integration tests
- **Update documentation**: Keep README and docs current
- **Follow existing patterns**: Maintain code consistency

## 🏗️ Project Architecture

### Folder Structure

```
scaff/
├── src/
│   ├── main.rs         # CLI entry point
│   ├── cli.rs          # Command definitions (clap)
│   ├── scanner.rs      # Multi-language code scanning
│   ├── pattern.rs      # Pattern extraction and storage
│   ├── generator.rs    # Code generation (Handlebars)
│   └── validator.rs    # Architecture validation
├── templates/          # Code generation templates
│   ├── rust_file.hbs   # Rust file template
│   └── js_file.hbs     # JavaScript file template
├── scaffs/             # Saved pattern files (JSON)
├── tests/              # Integration tests
└── Cargo.toml          # Dependencies and metadata
```

### Key Dependencies

- **[clap](https://crates.io/crates/clap)**: CLI argument parsing and commands
- **[tree-sitter](https://crates.io/crates/tree-sitter)**: Multi-language code parsing
- **[serde](https://crates.io/crates/serde)**: JSON serialization/deserialization
- **[handlebars](https://crates.io/crates/handlebars)**: Template engine for code generation
- **[tempfile](https://crates.io/crates/tempfile)**: Temporary directories for testing
- **[log](https://crates.io/crates/log)**: Structured logging

### Design Principles

1. **Modularity**: Each component has a single responsibility
2. **Extensibility**: Easy to add new languages and features
3. **Performance**: Optimized for large codebases
4. **Reliability**: Comprehensive error handling and testing
5. **Usability**: Intuitive CLI with helpful output

## 🎯 Development Roadmap

### ✅ Completed Phases
- **Phase 1**: CLI foundation with clap
- **Phase 2**: Tree-sitter integration for code scanning
- **Phase 3**: Pattern extraction and JSON serialization
- **Phase 4**: Handlebars-based code generation
- **Phase 5**: Architecture validation system
- **Phase 6**: Multi-language support (9 languages)
- **Phase 7**: Comprehensive test suite (54 tests)
- **Phase 8**: Enhanced documentation and community guidelines

### 🚧 Future Phases
- **Phase 9**: Community scaff registry
- **Phase 10**: Plugin system for custom language parsers
- **Phase 11**: Web interface for scaff management
- **Phase 12**: AI-powered pattern suggestions

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- **Tree-sitter**: For robust multi-language parsing
- **Handlebars**: For flexible template engine
- **Clap**: For excellent CLI experience
- **Rust Community**: For amazing ecosystem and tools

## 💬 Support

- 📖 **Documentation**: Check this README and inline docs
- 🐛 **Issues**: Report bugs on GitHub Issues
- 💬 **Discussions**: Join GitHub Discussions
- 📧 **Contact**: Open an issue for questions

---

**Happy Scaffolding!** 🏗️✨
