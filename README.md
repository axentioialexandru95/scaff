# Scaff - architecture in your pocket

## Main features

1. Scan the codebase and understand the patterns within the code
2. After finding the patterns within the code, be able to save the "scaff"
3. Saved scaffs can build their own "code generators" for example if you want to scaffold a resource table that uses a **CRUD**, you only have to build it once in order for the flow to be replicable
4. Take the architecture with you, when building a new project instead of just relying on decisions to start a project, just take the scaff with you, init a scaff and then either choose from community scaffs or use your own private scaffs
5. Architecture scanner, check if you're following the chosen pattern or not, so you know if you're properly working with the selected scaff
6. Profit.

## Why

I want to start other projects and I already have a very nice architecture in place in another one, I just want to take that with me into the next few projects since it's quite easy to use.

Also sometimes I'm also quite unsure about how a project's architecture is like so I'd like for a very profound scanner that understands the patterns well so that when I want to build a project, instead of building all the parts on their own, I just want to use the scaff -g + "module" or whatever that's similar in the codebase. That would make the project easier to work with and understand components better.

Automated scaffolder tests, it should create testable components.


## Tech

RUST

## Project Architecture & Implementation Plan (Rust)

### Folder Structure

```
scaff/
  ├── src/
  │   ├── main.rs         # CLI entry point
  │   ├── cli.rs          # CLI command definitions (using clap)
  │   ├── scanner.rs      # Codebase scanning logic (Tree-sitter integration)
  │   ├── pattern.rs      # Pattern extraction and serialization
  │   ├── generator.rs    # Code generation logic (Handlebars/Mustache)
  │   ├── validator.rs    # Architecture validation logic
  │   └── plugins.rs      # Plugin system for extensibility
  ├── templates/          # Example templates for code generation
  ├── scaffs/             # Saved scaffs (JSON/YAML)
  ├── Cargo.toml          # Rust dependencies and metadata
  └── README.md
```

### Key Dependencies
- **clap**: CLI argument parsing
- **tree-sitter**: Multi-language code parsing
- **serde / serde_json / serde_yaml**: (De)serialization of scaffs
- **handlebars**: Code generation templates
- **anyhow / thiserror**: Error handling
- **directories**: User config/data directories

### Initial Development Plan
1. **Bootstrap the CLI** with `clap` and basic commands: `scan`, `save`, `list`, `generate`, `validate`.
2. **Integrate Tree-sitter** for codebase scanning (start with JS/TS, expand to more languages).
3. **Implement pattern extraction and saving** (as JSON/YAML scaffs).
4. **Add code generation** using `handlebars` templates.
5. **Implement architecture validation** against selected scaffs.
6. **Design plugin system** for extensibility (future).

This plan ensures a modular, extensible, and high-performance CLI tool that can be integrated into any project, regardless of tech stack.
