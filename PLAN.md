# Scaff Implementation Plan (Rust)

## Phase 1: Project Setup & CLI Skeleton
- [x] Initialize Rust project with Cargo
- [x] Set up folder structure and module files
- [x] Add dependencies to Cargo.toml (clap, tree-sitter, serde, handlebars, etc.)
- [x] Implement CLI skeleton with basic commands: `scan`, `save`, `list`, `generate`, `validate`

## Phase 2: Codebase Scanning
- [x] Integrate Tree-sitter for multi-language parsing (start with JS/TS)
- [x] Implement scanner module to extract code structure (files, classes, functions)
- [x] Output scan results in a structured format

## Phase 3: Pattern Extraction & Saving
- [x] Allow user to select and save detected patterns as "scaffs"
- [x] Serialize scaffs as JSON/YAML in the `scaffs/` directory
- [x] Implement listing and management of saved scaffs

## Phase 4: Code Generation
- [x] Set up templating system using Handlebars
- [x] Implement generator module to create code from scaffs and templates
- [x] Provide example templates in `templates/`

## Phase 5: Architecture Validation
- [x] Implement validator module to compare codebase structure to a selected scaff
- [x] Report deviations and suggest fixes

## Phase 6: Extend languages
- [x] add as many as possible:
tree-sitter-python = "0.23.1"
tree-sitter-java = "0.23.1"
tree-sitter-go = "0.23.1"
tree-sitter-php = "0.23.1"
tree-sitter-ruby = "0.23.1"
tree-sitter-typescript = "0.23.1"
tree-sitter-json = "0.24.1"
tree-sitter-html = "0.23.1"
tree-sitter-css = "0.23.1"


## Phase 7: TEST
- [ ] Add tests for our functionality 

## Phase 8: Documentation & Community
- [ ] Expand README with usage examples and contribution guidelines
- [ ] Document scaff format and template usage
- [ ] Plan for community scaff registry (future)

---

This plan is iterative—each phase builds on the previous. Tasks can be checked off as progress is made.
