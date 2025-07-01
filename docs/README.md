# Scaff Documentation

Welcome to the Scaff documentation! This directory contains comprehensive guides and references for using and contributing to Scaff.

## 📚 Documentation Overview

### User Guides
- **[Main README](../README.md)** - Getting started, installation, and basic usage
- **[Scaff Format Guide](SCAFF_FORMAT.md)** - Complete reference for the JSON scaff format
- **[Template Guide](TEMPLATES.md)** - Handlebars template system and custom templates

### Development Guides  
- **[Contributing Guide](../CONTRIBUTING.md)** - How to contribute to the project
- **[API Documentation](API.md)** - Code documentation (generated with `cargo doc`)

## 🚀 Quick Navigation

### For New Users
1. Start with the [main README](../README.md) for installation and basic usage
2. Try the quick start examples
3. Learn about [scaff format](SCAFF_FORMAT.md) to understand the file structure
4. Explore [templates](TEMPLATES.md) for code generation

### For Contributors
1. Read the [contributing guide](../CONTRIBUTING.md)
2. Set up your development environment
3. Understand the project architecture
4. Make your first contribution

### For Advanced Users
1. Create custom templates using the [template guide](TEMPLATES.md)
2. Understand the full [scaff format specification](SCAFF_FORMAT.md)
3. Integrate Scaff into your workflow
4. Share scaffs with your team

## 📖 Topics by Use Case

### Code Pattern Management
- **Extracting patterns**: Use `scaff scan` to analyze codebases
- **Saving patterns**: Create reusable scaffs with `scaff save`
- **Organizing scaffs**: Best practices for scaff management

### Code Generation
- **Basic generation**: Generate code from patterns
- **Custom templates**: Create templates for your specific needs
- **Multi-language projects**: Handle complex project structures

### Architecture Validation
- **Pattern compliance**: Validate codebases against scaffs
- **Team standards**: Enforce architectural consistency
- **Continuous integration**: Integrate validation into CI/CD

### Team Collaboration
- **Sharing scaffs**: Version control and distribution
- **Documentation**: Document your architectural decisions
- **Onboarding**: Help new team members understand patterns

## 🔧 Reference Materials

### Command Reference
```bash
# Core commands
scaff scan --language <lang>        # Analyze codebase
scaff save <name> --language <lang> # Save pattern
scaff list                          # List available scaffs
scaff generate <name> --output <dir># Generate code
scaff validate <name>               # Validate architecture
```

### Supported Languages
- Rust (`.rs`)
- JavaScript (`.js`, `.jsx`)
- TypeScript (`.ts`, `.tsx`)
- Python (`.py`, `.pyi`)
- Java (`.java`)
- Go (`.go`)
- JSON (`.json`)
- HTML (`.html`, `.htm`)
- CSS (`.css`)

### File Locations
- **Scaffs**: `scaffs/` directory (JSON files)
- **Templates**: `templates/` directory (Handlebars files)
- **Generated code**: Specified output directory

## 🌟 Examples and Tutorials

### Basic Examples
- [Simple CLI Application](../README.md#example-1-rest-api-pattern)
- [React Component Library](../README.md#example-2-multi-language-project)
- [Team Architecture Standards](../README.md#example-3-team-architecture-standards)

### Template Examples
- [Rust service template](TEMPLATES.md#rust-templates-rust_filehbs)
- [JavaScript component template](TEMPLATES.md#javascript-templates-js_filehbs)
- [Configuration file templates](TEMPLATES.md#configuration-templates)

### Scaff Format Examples
- [Simple Rust CLI](SCAFF_FORMAT.md#simple-rust-cli-application)
- [React Component Library](SCAFF_FORMAT.md#react-component-library)
- [Python FastAPI Service](SCAFF_FORMAT.md#python-fastapi-service)

## 🤝 Getting Help

### Common Issues
- **Tree-sitter parsing errors**: Check language support and file syntax
- **Template compilation errors**: Verify Handlebars syntax
- **Missing scaffs**: Ensure scaff files exist in `scaffs/` directory
- **Generation failures**: Check output directory permissions

### Support Channels
- **GitHub Issues**: Report bugs and request features
- **GitHub Discussions**: Ask questions and share ideas
- **Documentation**: Check these docs for answers
- **Contributing**: Help improve the project

## 📝 Contributing to Documentation

We welcome contributions to improve the documentation:

1. **Fix typos and errors**: Small improvements are always appreciated
2. **Add examples**: More examples help users understand concepts
3. **Improve clarity**: Suggest clearer explanations or better organization
4. **Add missing information**: Fill gaps in the documentation

See the [contributing guide](../CONTRIBUTING.md) for details on how to contribute.

## 📋 Documentation Standards

When contributing to documentation:

- **Use clear language**: Write for users of all experience levels
- **Include examples**: Show practical usage wherever possible
- **Keep it current**: Update docs when making code changes
- **Test examples**: Ensure code examples actually work
- **Link between sections**: Help users navigate related information

---

**Need help?** Check the [main README](../README.md) or open an issue on GitHub. 