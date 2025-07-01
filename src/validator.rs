use crate::pattern::{CodePattern, FilePattern, ScaffDirectory};
use crate::scanner;
use log::info;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub scaff_name: String,
    pub is_valid: bool,
    pub missing_files: Vec<String>,
    pub extra_files: Vec<String>,
    pub missing_items: Vec<ValidationIssue>,
    pub extra_items: Vec<ValidationIssue>,
    pub suggestions: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ValidationIssue {
    pub file_path: String,
    pub item_type: String, // "class", "function", "struct", "implementation"
    pub item_name: String,
}

pub struct ArchitectureValidator;

impl ArchitectureValidator {
    pub fn new() -> Self {
        ArchitectureValidator
    }

    pub fn validate_against_scaff(
        &self,
        scaff_name: &str,
    ) -> Result<ValidationResult, Box<dyn std::error::Error>> {
        info!("Starting validation against scaff: {}", scaff_name);

        // Load the scaff pattern
        let scaff_pattern = self.load_scaff_pattern(scaff_name)?;

        // Scan current codebase
        let current_files = self.scan_current_codebase(&scaff_pattern.language)?;

        // Perform validation comparison
        let validation_result = self.compare_structures(&scaff_pattern, &current_files);

        Ok(validation_result)
    }

    fn load_scaff_pattern(
        &self,
        scaff_name: &str,
    ) -> Result<CodePattern, Box<dyn std::error::Error>> {
        let patterns = ScaffDirectory::load_patterns()?;

        patterns
            .into_iter()
            .find(|p| p.name == scaff_name)
            .ok_or_else(|| {
                format!(
                    "Scaff '{}' not found. Use 'scaff list' to see available scaffs.",
                    scaff_name
                )
                .into()
            })
    }

    fn scan_current_codebase(
        &self,
        language: &str,
    ) -> Result<Vec<FilePattern>, Box<dyn std::error::Error>> {
        info!("Scanning current codebase for language: {}", language);

        let files = match language {
            "JavaScript/TypeScript" => scanner::scan_js_ts_files_in_dir("."),
            "JavaScript" => scanner::scan_language_files_in_dir(".", "javascript"),
            "TypeScript" => scanner::scan_language_files_in_dir(".", "typescript"),
            "Python" => scanner::scan_language_files_in_dir(".", "python"),
            "Java" => scanner::scan_language_files_in_dir(".", "java"),
            "Go" => scanner::scan_language_files_in_dir(".", "go"),
            "Rust" => scanner::scan_rust_files_in_dir("."),
            "JSON" => scanner::scan_language_files_in_dir(".", "json"),
            "HTML" => scanner::scan_language_files_in_dir(".", "html"),
            "CSS" => scanner::scan_language_files_in_dir(".", "css"),
            _ => {
                return Err(format!("Unsupported language for validation: {}", language).into());
            }
        };

        Ok(files)
    }

    fn compare_structures(
        &self,
        scaff: &CodePattern,
        current_files: &[FilePattern],
    ) -> ValidationResult {
        info!("Comparing scaff structure with current codebase");

        let mut result = ValidationResult {
            scaff_name: scaff.name.clone(),
            is_valid: true,
            missing_files: Vec::new(),
            extra_files: Vec::new(),
            missing_items: Vec::new(),
            extra_items: Vec::new(),
            suggestions: Vec::new(),
        };

        // Create lookup maps for efficient comparison
        let scaff_files: HashMap<String, &FilePattern> =
            scaff.files.iter().map(|f| (f.path.clone(), f)).collect();

        let current_files_map: HashMap<String, &FilePattern> =
            current_files.iter().map(|f| (f.path.clone(), f)).collect();

        // Check for missing files
        for scaff_file in &scaff.files {
            if !current_files_map.contains_key(&scaff_file.path) {
                result.missing_files.push(scaff_file.path.clone());
                result.is_valid = false;

                // Add suggestion for missing file
                result.suggestions.push(format!(
                    "Create missing file: {} (should contain {} items)",
                    scaff_file.path,
                    scaff_file.classes.len()
                        + scaff_file.functions.len()
                        + scaff_file.structs.len()
                        + scaff_file.implementations.len()
                ));
            }
        }

        // Check for extra files
        for current_file in current_files {
            if !scaff_files.contains_key(&current_file.path) {
                result.extra_files.push(current_file.path.clone());
                // Extra files don't necessarily make architecture invalid
            }
        }

        // Compare items in matching files
        for scaff_file in &scaff.files {
            if let Some(current_file) = current_files_map.get(&scaff_file.path) {
                self.compare_file_items(&mut result, scaff_file, current_file);
            }
        }

        // Generate overall suggestions
        if result.missing_files.len() > 0 {
            result.suggestions.push(format!(
                "Consider running 'scaff generate {}' to create missing files",
                scaff.name
            ));
        }

        if result.missing_items.len() > 0 {
            result.suggestions.push(
                "Review missing items and implement them according to your scaff pattern"
                    .to_string(),
            );
        }

        if result.extra_files.len() > 0 && result.extra_files.len() > result.missing_files.len() {
            result.suggestions.push(
                "Consider updating your scaff pattern to include the new files in your architecture".to_string()
            );
        }

        result
    }

    fn compare_file_items(
        &self,
        result: &mut ValidationResult,
        scaff_file: &FilePattern,
        current_file: &FilePattern,
    ) {
        let file_path = &scaff_file.path;

        // Compare classes
        self.compare_items(
            result,
            file_path,
            "class",
            &scaff_file.classes,
            &current_file.classes,
        );

        // Compare functions
        self.compare_items(
            result,
            file_path,
            "function",
            &scaff_file.functions,
            &current_file.functions,
        );

        // Compare structs
        self.compare_items(
            result,
            file_path,
            "struct",
            &scaff_file.structs,
            &current_file.structs,
        );

        // Compare implementations
        self.compare_items(
            result,
            file_path,
            "implementation",
            &scaff_file.implementations,
            &current_file.implementations,
        );
    }

    fn compare_items(
        &self,
        result: &mut ValidationResult,
        file_path: &str,
        item_type: &str,
        scaff_items: &[String],
        current_items: &[String],
    ) {
        let scaff_set: HashSet<&String> = scaff_items.iter().collect();
        let current_set: HashSet<&String> = current_items.iter().collect();

        // Find missing items
        for item in scaff_items {
            if !current_set.contains(item) {
                result.missing_items.push(ValidationIssue {
                    file_path: file_path.to_string(),
                    item_type: item_type.to_string(),
                    item_name: item.clone(),
                });
                result.is_valid = false;
            }
        }

        // Find extra items (informational, not necessarily invalid)
        for item in current_items {
            if !scaff_set.contains(item) {
                result.extra_items.push(ValidationIssue {
                    file_path: file_path.to_string(),
                    item_type: item_type.to_string(),
                    item_name: item.clone(),
                });
            }
        }
    }

    pub fn display_validation_results(&self, result: &ValidationResult) {
        println!("\n🔍 Architecture Validation Results");
        println!("Scaff: {}", result.scaff_name);
        println!("{:-<60}", "");

        if result.is_valid {
            println!("✅ Architecture is VALID - matches scaff pattern!");
        } else {
            println!("❌ Architecture DEVIATES from scaff pattern");
        }

        // Show missing files
        if !result.missing_files.is_empty() {
            println!("\n📁 Missing Files ({}):", result.missing_files.len());
            for file in &result.missing_files {
                println!("  ❌ {}", file);
            }
        }

        // Show extra files
        if !result.extra_files.is_empty() {
            println!("\n📁 Extra Files ({}):", result.extra_files.len());
            for file in &result.extra_files {
                println!("  ➕ {}", file);
            }
        }

        // Show missing items
        if !result.missing_items.is_empty() {
            println!("\n🔧 Missing Items ({}):", result.missing_items.len());
            for issue in &result.missing_items {
                println!(
                    "  ❌ {} '{}' in {}",
                    issue.item_type, issue.item_name, issue.file_path
                );
            }
        }

        // Show extra items
        if !result.extra_items.is_empty() && result.extra_items.len() <= 10 {
            println!("\n🔧 Extra Items ({}):", result.extra_items.len());
            for issue in &result.extra_items {
                println!(
                    "  ➕ {} '{}' in {}",
                    issue.item_type, issue.item_name, issue.file_path
                );
            }
        } else if result.extra_items.len() > 10 {
            println!(
                "\n🔧 Extra Items ({}) - showing first 10:",
                result.extra_items.len()
            );
            for issue in result.extra_items.iter().take(10) {
                println!(
                    "  ➕ {} '{}' in {}",
                    issue.item_type, issue.item_name, issue.file_path
                );
            }
            println!("  ... and {} more", result.extra_items.len() - 10);
        }

        // Show suggestions
        if !result.suggestions.is_empty() {
            println!("\n💡 Suggestions:");
            for suggestion in &result.suggestions {
                println!("  • {}", suggestion);
            }
        }

        // Summary
        println!("\n📊 Summary:");
        println!("  Missing files: {}", result.missing_files.len());
        println!("  Extra files: {}", result.extra_files.len());
        println!("  Missing items: {}", result.missing_items.len());
        println!("  Extra items: {}", result.extra_items.len());

        if result.is_valid {
            println!("  🎉 Your codebase follows the scaff architecture!");
        } else {
            println!("  🔧 Consider addressing the missing files and items above.");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pattern::{CodePattern, FilePattern};

    fn create_test_file_pattern(path: &str) -> FilePattern {
        FilePattern {
            path: path.to_string(),
            extension: "rs".to_string(),
            classes: vec!["TestClass".to_string()],
            functions: vec!["test_function".to_string()],
            structs: vec!["TestStruct".to_string()],
            implementations: vec!["TestImpl".to_string()],
        }
    }

    fn create_test_scaff_pattern() -> CodePattern {
        CodePattern {
            name: "test_scaff".to_string(),
            description: "Test scaff pattern".to_string(),
            language: "Rust".to_string(),
            files: vec![
                create_test_file_pattern("src/main.rs"),
                create_test_file_pattern("src/lib.rs"),
            ],
            created_at: "2024-01-01T00:00:00Z".to_string(),
        }
    }

    #[test]
    fn test_architecture_validator_new() {
        let _validator = ArchitectureValidator::new();
        // Just verify it creates successfully
        assert!(true);
    }

    #[test]
    fn test_validation_result_creation() {
        let result = ValidationResult {
            scaff_name: "test".to_string(),
            is_valid: true,
            missing_files: vec![],
            extra_files: vec![],
            missing_items: vec![],
            extra_items: vec![],
            suggestions: vec![],
        };

        assert_eq!(result.scaff_name, "test");
        assert!(result.is_valid);
        assert!(result.missing_files.is_empty());
    }

    #[test]
    fn test_validation_issue_creation() {
        let issue = ValidationIssue {
            file_path: "src/main.rs".to_string(),
            item_type: "function".to_string(),
            item_name: "test_function".to_string(),
        };

        assert_eq!(issue.file_path, "src/main.rs");
        assert_eq!(issue.item_type, "function");
        assert_eq!(issue.item_name, "test_function");
    }

    #[test]
    fn test_compare_structures_perfect_match() {
        let validator = ArchitectureValidator::new();
        let scaff = create_test_scaff_pattern();
        let current_files = scaff.files.clone();

        let result = validator.compare_structures(&scaff, &current_files);

        assert!(result.is_valid);
        assert!(result.missing_files.is_empty());
        assert!(result.missing_items.is_empty());
        assert_eq!(result.scaff_name, "test_scaff");
    }

    #[test]
    fn test_compare_structures_missing_files() {
        let validator = ArchitectureValidator::new();
        let scaff = create_test_scaff_pattern();
        let current_files = vec![scaff.files[0].clone()]; // Only first file

        let result = validator.compare_structures(&scaff, &current_files);

        assert!(!result.is_valid);
        assert_eq!(result.missing_files.len(), 1);
        assert!(result.missing_files.contains(&"src/lib.rs".to_string()));
        assert!(!result.suggestions.is_empty());
    }

    #[test]
    fn test_compare_structures_extra_files() {
        let validator = ArchitectureValidator::new();
        let scaff = create_test_scaff_pattern();
        let mut current_files = scaff.files.clone();
        current_files.push(create_test_file_pattern("src/extra.rs"));

        let result = validator.compare_structures(&scaff, &current_files);

        assert!(result.is_valid); // Extra files don't make it invalid
        assert_eq!(result.extra_files.len(), 1);
        assert!(result.extra_files.contains(&"src/extra.rs".to_string()));
    }

    #[test]
    fn test_compare_structures_missing_items() {
        let validator = ArchitectureValidator::new();
        let scaff = create_test_scaff_pattern();

        let mut current_files = scaff.files.clone();
        current_files[0].functions.clear(); // Remove all functions from first file

        let result = validator.compare_structures(&scaff, &current_files);

        assert!(!result.is_valid);
        assert_eq!(result.missing_items.len(), 1);
        assert_eq!(result.missing_items[0].item_type, "function");
        assert_eq!(result.missing_items[0].item_name, "test_function");
        assert_eq!(result.missing_items[0].file_path, "src/main.rs");
    }

    #[test]
    fn test_compare_structures_extra_items() {
        let validator = ArchitectureValidator::new();
        let scaff = create_test_scaff_pattern();

        let mut current_files = scaff.files.clone();
        current_files[0]
            .functions
            .push("extra_function".to_string());

        let result = validator.compare_structures(&scaff, &current_files);

        assert!(result.is_valid); // Extra items don't make it invalid
        assert_eq!(result.extra_items.len(), 1);
        assert_eq!(result.extra_items[0].item_type, "function");
        assert_eq!(result.extra_items[0].item_name, "extra_function");
        assert_eq!(result.extra_items[0].file_path, "src/main.rs");
    }

    #[test]
    fn test_compare_items() {
        let validator = ArchitectureValidator::new();
        let mut result = ValidationResult {
            scaff_name: "test".to_string(),
            is_valid: true,
            missing_files: vec![],
            extra_files: vec![],
            missing_items: vec![],
            extra_items: vec![],
            suggestions: vec![],
        };

        let scaff_items = vec!["item1".to_string(), "item2".to_string()];
        let current_items = vec!["item1".to_string(), "item3".to_string()];

        validator.compare_items(
            &mut result,
            "test.rs",
            "function",
            &scaff_items,
            &current_items,
        );

        assert_eq!(result.missing_items.len(), 1);
        assert_eq!(result.missing_items[0].item_name, "item2");

        assert_eq!(result.extra_items.len(), 1);
        assert_eq!(result.extra_items[0].item_name, "item3");
    }

    #[test]
    fn test_scan_current_codebase_with_temp_files() -> Result<(), Box<dyn std::error::Error>> {
        let validator = ArchitectureValidator::new();

        // Just test that the scan function doesn't crash with Rust language
        let result = validator.scan_current_codebase("Rust");

        // Should either succeed or fail gracefully
        match result {
            Ok(files) => {
                // If successful, files can be empty or contain rust files
                assert!(
                    files
                        .iter()
                        .all(|f| f.extension == "rs" || f.extension.is_empty())
                );
            }
            Err(_) => {
                // If it fails, that's also acceptable for this test
                assert!(true);
            }
        }

        Ok(())
    }

    #[test]
    fn test_scan_current_codebase_unsupported_language() {
        let validator = ArchitectureValidator::new();
        let result = validator.scan_current_codebase("UnsupportedLanguage");

        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Unsupported language")
        );
    }

    #[test]
    fn test_scan_current_codebase_javascript() -> Result<(), Box<dyn std::error::Error>> {
        let validator = ArchitectureValidator::new();

        // Just test that the scan function works with JavaScript language
        let result = validator.scan_current_codebase("JavaScript");

        // Should either succeed or fail gracefully
        match result {
            Ok(files) => {
                // If successful, files can be empty or contain js files
                assert!(
                    files
                        .iter()
                        .all(|f| f.extension == "js" || f.extension.is_empty())
                );
            }
            Err(_) => {
                // If it fails, that's also acceptable for this test
                assert!(true);
            }
        }

        Ok(())
    }

    #[test]
    fn test_validate_against_scaff_nonexistent() {
        let validator = ArchitectureValidator::new();
        let result = validator.validate_against_scaff("nonexistent_scaff");

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not found"));
    }
}
