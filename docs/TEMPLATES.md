# Template Documentation

This document covers the template system used by Scaff for code generation. Scaff uses Handlebars as its templating engine to generate code from saved patterns.

## Table of Contents
- [Overview](#overview)
- [Template Location](#template-location)
- [Template Syntax](#template-syntax)
- [Built-in Helpers](#built-in-helpers)
- [Template Variables](#template-variables)
- [Language-Specific Templates](#language-specific-templates)
- [Examples](#examples)
- [Custom Templates](#custom-templates)
- [Best Practices](#best-practices)

## Overview

Templates in Scaff are Handlebars files (`.hbs`) that define how code should be generated from a scaff pattern. When you run `scaff generate`, the tool:

1. Loads the specified scaff pattern (JSON file)
2. Finds appropriate templates for the language
3. Processes each template with the pattern data
4. Generates the final code files

## Template Location

Templates are located in the `templates/` directory:

```
templates/
├── rust_file.hbs       # Rust code generation
├── js_file.hbs         # JavaScript code generation
├── cargo.hbs           # Cargo.toml generation
├── package.hbs         # package.json generation
└── custom_template.hbs # Your custom templates
```

### Template Naming Convention

- `{language}_file.hbs`: Main code file template for a language
- `{config}_file.hbs`: Configuration file templates
- `default_{language}_file.hbs`: Fallback template for languages

## Template Syntax

Scaff uses standard Handlebars syntax with additional helper functions.

### Basic Interpolation
```handlebars
{{variable_name}}
```

### Conditionals
```handlebars
{{#if condition}}
  Content when true
{{else}}
  Content when false
{{/if}}
```

### Loops
```handlebars
{{#each array}}
  {{this}} {{@index}}
{{/each}}
```

### Comments
```handlebars
{{!-- This is a comment --}}
```

## Built-in Helpers

Scaff provides several built-in Handlebars helpers for common transformations:

### `uppercase`
Converts text to UPPERCASE.

```handlebars
{{uppercase "hello world"}}
<!-- Output: HELLO WORLD -->

{{uppercase pattern.name}}
<!-- If pattern.name is "my-api", output: MY-API -->
```

### `lowercase`
Converts text to lowercase.

```handlebars
{{lowercase "Hello World"}}
<!-- Output: hello world -->

{{lowercase file.path}}
<!-- If file.path is "SRC/MAIN.RS", output: src/main.rs -->
```

### `pascal_case`
Converts text to PascalCase (UpperCamelCase).

```handlebars
{{pascal_case "my_function_name"}}
<!-- Output: MyFunctionName -->

{{pascal_case "user-service"}}
<!-- Output: UserService -->
```

### `snake_case`
Converts text to snake_case.

```handlebars
{{snake_case "MyClassName"}}
<!-- Output: my_class_name -->

{{snake_case "user-service"}}
<!-- Output: user_service -->
```

## Template Variables

Templates have access to the following data structure:

### Root Variables
- `pattern`: The entire scaff pattern object
- `file`: Current file being processed (in file context)

### Pattern Object
```handlebars
pattern.name         // Scaff name
pattern.description  // Scaff description
pattern.language     // Primary language
pattern.created_at   // Creation timestamp
pattern.files        // Array of file objects
```

### File Object (when processing individual files)
```handlebars
file.path            // File path
file.extension       // File extension
file.classes         // Array of class names
file.functions       // Array of function names
file.structs         // Array of struct names
file.implementations // Array of implementation names
```

## Language-Specific Templates

### Rust Templates (`rust_file.hbs`)

```handlebars
{{!-- File header --}}
// Generated from scaff: {{pattern.name}}
// {{pattern.description}}

{{!-- Generate structs --}}
{{#each file.structs}}
#[derive(Debug, Clone)]
pub struct {{pascal_case this}} {
    // TODO: Add fields for {{this}}
}

{{/each}}

{{!-- Generate implementations --}}
{{#each file.implementations}}
impl {{pascal_case this}} {
    pub fn new() -> Self {
        Self {
            // TODO: Initialize {{this}}
        }
    }
}

{{/each}}

{{!-- Generate functions --}}
{{#each file.functions}}
pub fn {{snake_case this}}() {
    // TODO: Implement {{this}}
    todo!("Implement {{this}}")
}

{{/each}}
```

### JavaScript Templates (`js_file.hbs`)

```handlebars
{{!-- File header --}}
/**
 * Generated from scaff: {{pattern.name}}
 * {{pattern.description}}
 */

{{!-- Generate classes --}}
{{#each file.classes}}
export class {{pascal_case this}} {
  constructor() {
    // TODO: Initialize {{this}}
  }
}

{{/each}}

{{!-- Generate functions --}}
{{#each file.functions}}
export function {{this}}() {
  // TODO: Implement {{this}}
  throw new Error('Not implemented: {{this}}');
}

{{/each}}
```

### Configuration Templates

#### Cargo.toml Template (`cargo.hbs`)
```handlebars
[package]
name = "{{snake_case pattern.name}}"
version = "0.1.0"
edition = "2021"
description = "{{pattern.description}}"

[dependencies]
# Add your dependencies here
clap = { version = "4.0", features = ["derive"] }
serde = { version = "1.0", features = ["derive"] }
tokio = { version = "1.0", features = ["full"] }
```

#### package.json Template (`package.hbs`)
```handlebars
{
  "name": "{{snake_case pattern.name}}",
  "version": "1.0.0",
  "description": "{{pattern.description}}",
  "main": "index.js",
  "scripts": {
    "start": "node index.js",
    "test": "jest",
    "build": "webpack --mode production"
  },
  "dependencies": {
    {{#if (eq pattern.language "TypeScript")}}
    "typescript": "^5.0.0",
    "@types/node": "^20.0.0"
    {{else}}
    "express": "^4.18.0"
    {{/if}}
  },
  "devDependencies": {
    "jest": "^29.0.0"
  }
}
```

## Examples

### Complete Rust Service Template

```handlebars
{{!-- src/main.rs --}}
use std::error::Error;

{{#each file.structs}}
#[derive(Debug, Clone)]
pub struct {{pascal_case this}} {
    // TODO: Define fields for {{this}}
}

{{/each}}

{{#each file.implementations}}
impl {{pascal_case this}} {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        // TODO: Implement constructor for {{this}}
        Ok(Self {})
    }
    
    pub async fn run(&self) -> Result<(), Box<dyn Error>> {
        // TODO: Implement main logic for {{this}}
        Ok(())
    }
}

{{/each}}

{{#if (contains file.functions "main")}}
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Generated main function for {{pattern.name}}
    println!("Starting {{pattern.name}}");
    
    {{#each file.structs}}
    let {{snake_case this}} = {{pascal_case this}}::new()?;
    {{snake_case this}}.run().await?;
    {{/each}}
    
    Ok(())
}
{{/if}}

{{#each file.functions}}
{{#unless (eq this "main")}}
pub async fn {{snake_case this}}() -> Result<(), Box<dyn Error>> {
    // TODO: Implement {{this}}
    println!("Executing {{this}}");
    Ok(())
}

{{/unless}}
{{/each}}
```

### React Component Template

```handlebars
{{!-- Generated React component --}}
import React from 'react';

{{#each file.structs}}
interface {{pascal_case this}} {
  // TODO: Define props for {{this}}
}

{{/each}}

{{#each file.classes}}
export const {{pascal_case this}}: React.FC<{{pascal_case this}}Props> = (props) => {
  {{#each ../file.functions}}
  const {{this}} = () => {
    // TODO: Implement {{this}}
  };
  
  {{/each}}
  
  return (
    <div className="{{kebab_case this}}">
      <h1>{{pascal_case this}} Component</h1>
      {/* TODO: Add component content */}
    </div>
  );
};

{{/each}}

export default {{pascal_case (first file.classes)}};
```

## Custom Templates

You can create custom templates for specific use cases:

### API Route Template (`api_route.hbs`)

```handlebars
{{!-- Express.js API route --}}
const express = require('express');
const router = express.Router();

{{#each file.functions}}
// {{this}} endpoint
router.{{#if (contains this "get")}}get{{else if (contains this "post")}}post{{else if (contains this "put")}}put{{else if (contains this "delete")}}delete{{else}}use{{/if}}('/{{kebab_case this}}', async (req, res) => {
  try {
    // TODO: Implement {{this}}
    res.json({ message: '{{this}} not implemented' });
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
});

{{/each}}

module.exports = router;
```

### Database Model Template (`model.hbs`)

```handlebars
{{!-- Database model using Sequelize --}}
const { DataTypes } = require('sequelize');
const sequelize = require('../config/database');

{{#each file.structs}}
const {{pascal_case this}} = sequelize.define('{{pascal_case this}}', {
  id: {
    type: DataTypes.INTEGER,
    primaryKey: true,
    autoIncrement: true
  },
  // TODO: Add fields for {{this}}
  {{#each ../file.functions}}
  {{#if (contains this "created")}}
  createdAt: {
    type: DataTypes.DATE,
    defaultValue: DataTypes.NOW
  },
  {{/if}}
  {{#if (contains this "updated")}}
  updatedAt: {
    type: DataTypes.DATE,
    defaultValue: DataTypes.NOW
  }
  {{/if}}
  {{/each}}
}, {
  tableName: '{{snake_case this}}s',
  timestamps: true
});

{{/each}}

module.exports = {
  {{#each file.structs}}
  {{pascal_case this}}{{#unless @last}},{{/unless}}
  {{/each}}
};
```

## Best Practices

### Template Organization

1. **One Template Per File Type**: Create separate templates for different file types
2. **Language-Specific Templates**: Use different templates for each programming language
3. **Modular Templates**: Break complex templates into smaller, reusable parts
4. **Clear Naming**: Use descriptive names that indicate the template's purpose

### Code Quality

1. **Include TODOs**: Add TODO comments for parts that need manual implementation
2. **Generate Valid Code**: Ensure generated code compiles/runs without errors
3. **Use Proper Formatting**: Include appropriate indentation and spacing
4. **Add Comments**: Include helpful comments in generated code

### Maintainability

1. **Version Templates**: Keep templates updated with language best practices
2. **Test Templates**: Verify templates generate correct code
3. **Document Custom Templates**: Explain what custom templates do and when to use them
4. **Use Helpers**: Leverage built-in helpers for consistent transformations

### Error Handling

1. **Handle Missing Data**: Use conditionals to handle optional fields
2. **Provide Defaults**: Set sensible defaults for missing values
3. **Validate Input**: Check for required fields before processing
4. **Graceful Degradation**: Ensure templates work with minimal data

### Performance

1. **Avoid Deep Nesting**: Keep template logic simple and flat
2. **Cache Templates**: Reuse compiled templates when possible
3. **Minimize Helpers**: Use built-in helpers efficiently
4. **Optimize Loops**: Be careful with nested iterations

## Advanced Features

### Conditional Generation

```handlebars
{{#if (gt (length file.classes) 0)}}
// This file contains classes
{{#each file.classes}}
export class {{pascal_case this}} {}
{{/each}}
{{else}}
// This file contains only functions
{{#each file.functions}}
export function {{this}}() {}
{{/each}}
{{/if}}
```

### Template Partials

```handlebars
{{!-- _header.hbs partial --}}
/**
 * Generated by Scaff
 * Pattern: {{pattern.name}}
 * Created: {{pattern.created_at}}
 */

{{!-- main template --}}
{{> _header}}

// Your code here
```

### Language Detection

```handlebars
{{#if (eq pattern.language "Rust")}}
// Rust-specific code
use std::collections::HashMap;
{{else if (eq pattern.language "JavaScript")}}
// JavaScript-specific code
const fs = require('fs');
{{/if}}
```

---

For more information, see the [main README](../README.md) or [scaff format documentation](SCAFF_FORMAT.md). 