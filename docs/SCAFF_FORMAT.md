# Scaff Format Documentation

This document provides a comprehensive guide to the Scaff JSON format, which is used to store and share code architecture patterns.

## Table of Contents
- [Overview](#overview)
- [File Structure](#file-structure)
- [Field Definitions](#field-definitions)
- [Examples](#examples)
- [Best Practices](#best-practices)
- [Validation](#validation)

## Overview

A Scaff is a JSON file that captures the structure and patterns of a codebase. It contains metadata about the pattern and detailed information about each file's code elements.

Scaff files are stored in the `scaffs/` directory with the naming convention `{name}.json`.

## File Structure

```json
{
  "name": "string",
  "description": "string", 
  "language": "string",
  "created_at": "ISO 8601 timestamp",
  "files": [
    {
      "path": "string",
      "extension": "string",
      "classes": ["string"],
      "functions": ["string"],
      "structs": ["string"],
      "implementations": ["string"]
    }
  ]
}
```

## Field Definitions

### Root Fields

#### `name` (required)
- **Type**: String
- **Description**: Unique identifier for the scaff
- **Constraints**: 
  - Must be unique within your scaffs directory
  - Should follow kebab-case convention (e.g., `my-api-pattern`)
  - No spaces or special characters except hyphens
- **Example**: `"rest-api-with-auth"`

#### `description` (required)
- **Type**: String
- **Description**: Human-readable description of the architectural pattern
- **Constraints**: 
  - Should be descriptive and helpful for future users
  - Recommended length: 10-100 characters
- **Example**: `"REST API with JWT authentication and database integration"`

#### `language` (required)
- **Type**: String
- **Description**: Primary programming language of the pattern
- **Constraints**: Must be one of the supported languages
- **Supported Values**:
  - `"Rust"`
  - `"JavaScript"`
  - `"TypeScript"`
  - `"Python"`
  - `"Java"`
  - `"Go"`
  - `"JSON"`
  - `"HTML"`
  - `"CSS"`
- **Example**: `"Rust"`

#### `created_at` (required)
- **Type**: String (ISO 8601 timestamp)
- **Description**: When the scaff was created
- **Format**: `YYYY-MM-DDTHH:MM:SSZ`
- **Example**: `"2024-01-15T14:30:00Z"`

#### `files` (required)
- **Type**: Array of File objects
- **Description**: List of files and their extracted code patterns
- **Constraints**: Cannot be empty
- **Example**: See File Object section below

### File Object Fields

#### `path` (required)
- **Type**: String
- **Description**: Relative path to the file from project root
- **Constraints**: 
  - Should use forward slashes as path separators
  - Should be relative paths (no leading slash)
- **Examples**: 
  - `"src/main.rs"`
  - `"components/Button.jsx"`
  - `"api/auth.py"`

#### `extension` (required)
- **Type**: String
- **Description**: File extension without the dot
- **Examples**: 
  - `"rs"`
  - `"js"`
  - `"py"`
  - `"java"`

#### `classes` (required)
- **Type**: Array of strings
- **Description**: Class names found in the file
- **Notes**: 
  - Empty array if no classes found
  - Includes interfaces for TypeScript/Java
  - Includes HTML elements for HTML files
- **Examples**: 
  - `["UserService", "AuthController"]`
  - `["interface UserInterface", "MyComponent"]`
  - `[]`

#### `functions` (required)
- **Type**: Array of strings
- **Description**: Function/method names found in the file
- **Notes**: 
  - Empty array if no functions found
  - Includes methods within classes
  - Includes constructor functions
- **Examples**: 
  - `["main", "authenticate", "validate_token"]`
  - `["getUserById", "createUser", "deleteUser"]`
  - `[]`

#### `structs` (required)
- **Type**: Array of strings
- **Description**: Struct/type definitions found in the file
- **Notes**: 
  - Empty array if no structs found
  - Includes Go types, Rust structs, TypeScript types
  - Includes JSON keys for JSON files
- **Examples**: 
  - `["User", "AuthRequest", "DatabaseConfig"]`
  - `["\"name\"", "\"version\"", "\"dependencies\""]` (JSON)
  - `[]`

#### `implementations` (required)
- **Type**: Array of strings  
- **Description**: Implementation blocks found in the file
- **Notes**: 
  - Empty array if no implementations found
  - Primarily used for Rust `impl` blocks
  - May include interface implementations in other languages
- **Examples**: 
  - `["User", "DatabaseConnection"]`
  - `[]`

## Examples

### Simple Rust CLI Application

```json
{
  "name": "simple-cli",
  "description": "Basic CLI application with argument parsing",
  "language": "Rust",
  "created_at": "2024-01-15T10:00:00Z",
  "files": [
    {
      "path": "src/main.rs",
      "extension": "rs",
      "classes": [],
      "functions": ["main", "parse_args"],
      "structs": ["Config", "Args"],
      "implementations": ["Config"]
    },
    {
      "path": "Cargo.toml",
      "extension": "toml",
      "classes": [],
      "functions": [],
      "structs": ["package", "dependencies"],
      "implementations": []
    }
  ]
}
```

### React Component Library

```json
{
  "name": "react-component-lib",
  "description": "Reusable React component library with TypeScript",
  "language": "TypeScript",
  "created_at": "2024-01-15T11:30:00Z",
  "files": [
    {
      "path": "src/components/Button.tsx",
      "extension": "tsx",
      "classes": ["Button"],
      "functions": ["Button", "handleClick"],
      "structs": ["ButtonProps"],
      "implementations": []
    },
    {
      "path": "src/components/Modal.tsx", 
      "extension": "tsx",
      "classes": ["Modal"],
      "functions": ["Modal", "openModal", "closeModal"],
      "structs": ["ModalProps"],
      "implementations": []
    },
    {
      "path": "package.json",
      "extension": "json",
      "classes": [],
      "functions": [],
      "structs": ["\"name\"", "\"version\"", "\"scripts\"", "\"dependencies\""],
      "implementations": []
    }
  ]
}
```

### Python FastAPI Service

```json
{
  "name": "fastapi-service",
  "description": "FastAPI web service with database integration",
  "language": "Python",
  "created_at": "2024-01-15T16:45:00Z",
  "files": [
    {
      "path": "main.py",
      "extension": "py",
      "classes": [],
      "functions": ["create_app", "startup_event", "shutdown_event"],
      "structs": [],
      "implementations": []
    },
    {
      "path": "models/user.py",
      "extension": "py", 
      "classes": ["User", "UserCreate", "UserResponse"],
      "functions": ["__init__", "create_user", "get_user"],
      "structs": [],
      "implementations": []
    },
    {
      "path": "routers/auth.py",
      "extension": "py",
      "classes": [],
      "functions": ["login", "register", "verify_token"],
      "structs": [],
      "implementations": []
    }
  ]
}
```

## Best Practices

### Naming Conventions

1. **Scaff Names**: Use kebab-case (e.g., `rest-api-auth`, `react-component-lib`)
2. **Descriptions**: Be specific and include key technologies (e.g., "FastAPI service with PostgreSQL and Redis")
3. **File Paths**: Use forward slashes and relative paths from project root

### Organization

1. **Group Related Files**: Include all files that define the pattern
2. **Include Configuration**: Add build files, configs, and dependencies
3. **Meaningful Structure**: Ensure the file list represents the complete pattern

### Metadata

1. **Accurate Language**: Use the primary language even for multi-language projects
2. **Descriptive Descriptions**: Help future users understand the pattern's purpose
3. **Consistent Timestamps**: Use UTC timezone in ISO 8601 format

### Content Quality

1. **Complete Patterns**: Include all essential files for the pattern
2. **Clean Data**: Remove temporary or generated files from the pattern
3. **Documented Elements**: Ensure important code elements are captured

## Validation

### Required Fields Validation

All scaffs must include:
- `name`: Non-empty string
- `description`: Non-empty string  
- `language`: Supported language value
- `created_at`: Valid ISO 8601 timestamp
- `files`: Non-empty array

### File Validation

Each file object must include:
- `path`: Valid relative path
- `extension`: File extension matching the path
- `classes`: Array (can be empty)
- `functions`: Array (can be empty)
- `structs`: Array (can be empty)
- `implementations`: Array (can be empty)

### Consistency Validation

- File extensions should match the scaff language
- Paths should be consistent with project structure
- Code elements should be realistic for the language

### Size Limitations

- Scaff names: Maximum 50 characters
- Descriptions: Maximum 200 characters
- File arrays: Maximum 100 files per scaff
- Code element arrays: Maximum 50 items per array

## Error Handling

Common validation errors and solutions:

### Invalid JSON Format
```
Error: Failed to parse scaff file
Solution: Validate JSON syntax using a JSON validator
```

### Missing Required Fields
```
Error: Missing required field 'name'
Solution: Ensure all required fields are present
```

### Invalid Language
```
Error: Unsupported language 'PHP'
Solution: Use one of the supported languages listed above
```

### Duplicate Names
```
Error: Scaff 'my-pattern' already exists
Solution: Choose a unique name or delete existing scaff
```

## Version Compatibility

This format specification is for Scaff v1.0+. Future versions will maintain backward compatibility where possible, with migration tools provided for breaking changes.

### Version History

- **v1.0**: Initial format specification
- **Future**: Additional fields may be added (optional)

---

For more information, see the [main README](../README.md) or [template documentation](TEMPLATES.md). 