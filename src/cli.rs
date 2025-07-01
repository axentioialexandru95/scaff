use crate::generator::CodeGenerator;
use crate::pattern::{ScaffDirectory, ScaffConfig, create_pattern_from_scan, display_pattern_summary};
use crate::scanner;
use crate::validator::ArchitectureValidator;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "scaff")]
#[command(about = "Architecture in your pocket", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Scan the codebase for patterns
    Scan {
        /// Language to scan for (js, rust, or all)
        #[arg(short, long, default_value = "all")]
        language: String,
    },
    /// Save a detected pattern as a scaff
    Save {
        name: String,
        /// Language to scan for (js, rust, or all)
        #[arg(short, long, default_value = "all")]
        language: String,
    },
    /// List available scaffs
    List {},
    /// Generate code from a scaff
    Generate {
        /// Scaff name (optional if default scaff is set)
        scaff: Option<String>,
        /// Output directory for generated code
        #[arg(short, long, default_value = "generated")]
        output: String,
    },
    /// Validate codebase against a scaff
    Validate { 
        /// Scaff name (optional if default scaff is set)
        scaff: Option<String> 
    },
    /// Manage default scaff
    Default {
        #[command(subcommand)]
        action: DefaultActions,
    },
}

#[derive(Subcommand)]
enum DefaultActions {
    /// Set the default scaff
    Set { scaff: String },
    /// Get the current default scaff
    Get {},
    /// Clear the default scaff
    Clear {},
}

fn resolve_scaff_name(scaff: Option<String>) -> Result<String, String> {
    match scaff {
        Some(name) => Ok(name),
        None => {
            let config = ScaffConfig::load().map_err(|e| format!("Failed to load config: {}", e))?;
            match config.get_default_scaff() {
                Some(default_scaff) => {
                    println!("💡 Using default scaff: {}", default_scaff);
                    Ok(default_scaff.clone())
                }
                None => Err("No scaff specified and no default scaff set. Use 'scaff default set <scaff-name>' to set a default, or specify a scaff name explicitly.".to_string()),
            }
        }
    }
}

pub fn run() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Scan { language } => {
            println!("🔍 Scanning the codebase for patterns...");

            match language.as_str() {
                "js" | "javascript" => {
                    let files = scanner::scan_language_files_in_dir(".", "javascript");
                    scanner::display_scan_results(&files, "JavaScript");

                    if !files.is_empty() {
                        println!(
                            "\n💡 To save this pattern, run: scaff save <pattern-name> --language javascript"
                        );
                    }
                }
                "ts" | "typescript" => {
                    let files = scanner::scan_language_files_in_dir(".", "typescript");
                    scanner::display_scan_results(&files, "TypeScript");

                    if !files.is_empty() {
                        println!(
                            "\n💡 To save this pattern, run: scaff save <pattern-name> --language typescript"
                        );
                    }
                }
                "python" | "py" => {
                    let files = scanner::scan_language_files_in_dir(".", "python");
                    scanner::display_scan_results(&files, "Python");

                    if !files.is_empty() {
                        println!(
                            "\n💡 To save this pattern, run: scaff save <pattern-name> --language python"
                        );
                    }
                }
                "java" => {
                    let files = scanner::scan_language_files_in_dir(".", "java");
                    scanner::display_scan_results(&files, "Java");

                    if !files.is_empty() {
                        println!(
                            "\n💡 To save this pattern, run: scaff save <pattern-name> --language java"
                        );
                    }
                }
                "go" => {
                    let files = scanner::scan_language_files_in_dir(".", "go");
                    scanner::display_scan_results(&files, "Go");

                    if !files.is_empty() {
                        println!(
                            "\n💡 To save this pattern, run: scaff save <pattern-name> --language go"
                        );
                    }
                }
                "rust" => {
                    let files = scanner::scan_rust_files_in_dir(".");
                    scanner::display_scan_results(&files, "Rust");

                    if !files.is_empty() {
                        println!(
                            "\n💡 To save this pattern, run: scaff save <pattern-name> --language rust"
                        );
                    }
                }
                "json" => {
                    let files = scanner::scan_language_files_in_dir(".", "json");
                    scanner::display_scan_results(&files, "JSON");

                    if !files.is_empty() {
                        println!(
                            "\n💡 To save this pattern, run: scaff save <pattern-name> --language json"
                        );
                    }
                }
                "html" => {
                    let files = scanner::scan_language_files_in_dir(".", "html");
                    scanner::display_scan_results(&files, "HTML");

                    if !files.is_empty() {
                        println!(
                            "\n💡 To save this pattern, run: scaff save <pattern-name> --language html"
                        );
                    }
                }
                "css" => {
                    let files = scanner::scan_language_files_in_dir(".", "css");
                    scanner::display_scan_results(&files, "CSS");

                    if !files.is_empty() {
                        println!(
                            "\n💡 To save this pattern, run: scaff save <pattern-name> --language css"
                        );
                    }
                }
                "all" => {
                    let results = scanner::scan_all_languages_in_dir(".");

                    if results.is_empty() {
                        println!("No supported files found.");
                        println!(
                            "Supported languages: rust, javascript, typescript, python, java, go, json, html, css"
                        );
                        return;
                    }

                    scanner::display_all_scan_results(&results);

                    println!("\n💡 To save a specific language pattern:");
                    let supported_langs = scanner::get_supported_languages();
                    for (lang_display, _) in &results {
                        // Convert display name back to language identifier
                        let lang_name = supported_langs
                            .iter()
                            .find(|&lang| scanner::get_language_display_name(lang) == *lang_display)
                            .unwrap_or(&"unknown");
                        println!("   scaff save <pattern-name> --language {}", lang_name);
                    }
                }
                _ => {
                    println!("❌ Unsupported language: {}", language);
                    let supported = scanner::get_supported_languages();
                    println!("Supported languages: {}, all", supported.join(", "));
                    return;
                }
            }
        }
        Commands::Save { name, language } => {
            println!("💾 Saving pattern as scaff: {}", name);

            let (files, lang_type) = match language.as_str() {
                "javascript" => (
                    scanner::scan_language_files_in_dir(".", "javascript"),
                    "JavaScript",
                ),
                "typescript" => (
                    scanner::scan_language_files_in_dir(".", "typescript"),
                    "TypeScript",
                ),
                "python" => (scanner::scan_language_files_in_dir(".", "python"), "Python"),
                "java" => (scanner::scan_language_files_in_dir(".", "java"), "Java"),
                "go" => (scanner::scan_language_files_in_dir(".", "go"), "Go"),
                "rust" => (scanner::scan_rust_files_in_dir("."), "Rust"),
                "json" => (scanner::scan_language_files_in_dir(".", "json"), "JSON"),
                "html" => (scanner::scan_language_files_in_dir(".", "html"), "HTML"),
                "css" => (scanner::scan_language_files_in_dir(".", "css"), "CSS"),
                _ => {
                    println!("❌ Unsupported language: {}", language);
                    let supported = scanner::get_supported_languages();
                    println!("Supported languages: {}", supported.join(", "));
                    return;
                }
            };

            if files.is_empty() {
                println!("❌ No files found to save as pattern");
                return;
            }

            let pattern = create_pattern_from_scan(files, name, lang_type.to_string());
            display_pattern_summary(&pattern);

            let scaff_dir = ScaffDirectory::new();
            match scaff_dir.save_pattern(&pattern) {
                Ok(_) => {
                    println!("✅ Successfully saved pattern '{}'", pattern.name);
                    println!(
                        "💡 To generate code from this pattern, run: scaff generate {} --output <directory>",
                        pattern.name
                    );
                }
                Err(e) => println!("❌ Failed to save pattern: {}", e),
            }
        }
        Commands::List {} => match ScaffDirectory::list_patterns() {
            Ok(_) => {}
            Err(e) => println!("❌ Failed to list patterns: {}", e),
        },
        Commands::Generate { scaff, output } => {
            let scaff_name = match resolve_scaff_name(scaff) {
                Ok(name) => name,
                Err(e) => {
                    println!("❌ {}", e);
                    return;
                }
            };

            println!(
                "🏗️ Generating code from scaff: {} to directory: {}",
                scaff_name, output
            );

            match CodeGenerator::new() {
                Ok(generator) => match generator.generate_from_scaff(&scaff_name, &output) {
                    Ok(_) => {
                        println!(
                            "💡 You can now explore the generated code in the '{}' directory",
                            output
                        );
                        println!(
                            "💡 For Rust projects, run 'cd {} && cargo check' to verify the generated code",
                            output
                        );
                    }
                    Err(e) => {
                        println!("❌ Failed to generate code: {}", e);
                        if e.to_string().contains("No such file") {
                            println!(
                                "💡 Make sure the scaff '{}' exists. Run 'scaff list' to see available scaffs.",
                                scaff_name
                            );
                        }
                    }
                },
                Err(e) => {
                    println!("❌ Failed to initialize code generator: {}", e);
                }
            }
        }
        Commands::Validate { scaff } => {
            let scaff_name = match resolve_scaff_name(scaff) {
                Ok(name) => name,
                Err(e) => {
                    println!("❌ {}", e);
                    return;
                }
            };

            println!("🔍 Validating codebase against scaff: {}", scaff_name);

            let validator = ArchitectureValidator::new();
            match validator.validate_against_scaff(&scaff_name) {
                Ok(result) => {
                    validator.display_validation_results(&result);
                }
                Err(e) => {
                    println!("❌ Validation failed: {}", e);
                    if e.to_string().contains("not found") {
                        println!("💡 Run 'scaff list' to see available scaffs.");
                    }
                }
            }
        }
        Commands::Default { action } => {
            match action {
                DefaultActions::Set { scaff } => {
                    println!("🔧 Setting default scaff: {}", scaff);
                    let mut config = match ScaffConfig::load() {
                        Ok(config) => config,
                        Err(e) => {
                            println!("❌ Failed to load config: {}", e);
                            return;
                        }
                    };

                    match config.set_default_scaff(&scaff) {
                        Ok(_) => {
                            println!("✅ Successfully set default scaff to '{}'", scaff);
                            println!("💡 You can now use 'scaff generate' and 'scaff validate' without specifying a scaff name.");
                        }
                        Err(e) => {
                            println!("❌ Failed to set default scaff: {}", e);
                            println!("💡 Run 'scaff list' to see available scaffs.");
                        }
                    }
                }
                DefaultActions::Get {} => {
                    match ScaffConfig::load() {
                        Ok(config) => {
                            if let Some(default_scaff) = config.get_default_scaff() {
                                println!("� Current default scaff: {}", default_scaff);
                            } else {
                                println!("💡 No default scaff is currently set.");
                                println!("💡 Use 'scaff default set <scaff-name>' to set one.");
                            }
                        }
                        Err(e) => println!("❌ Failed to load config: {}", e),
                    }
                }
                DefaultActions::Clear {} => {
                    let mut config = match ScaffConfig::load() {
                        Ok(config) => config,
                        Err(e) => {
                            println!("❌ Failed to load config: {}", e);
                            return;
                        }
                    };

                    match config.clear_default_scaff() {
                        Ok(_) => {
                            println!("✅ Successfully cleared default scaff");
                            println!("💡 You'll need to specify scaff names explicitly for generate and validate commands.");
                        }
                        Err(e) => println!("❌ Failed to clear default scaff: {}", e),
                    }
                }
            }
        }
    }
}
