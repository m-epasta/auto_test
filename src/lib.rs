//! # AutoTest
//!
//! A Rust library and CLI tool for automatically generating test stubs for Rust projects.
//!
//! This crate provides both a command-line interface and a programmatic API for
//! analyzing Rust source code and generating meaningful test templates. It uses
//! AST-based analysis to understand function signatures, parameters, and return
//! types, then generates integration tests with appropriate assertions.
//!
//! ## Quick Start
//!
//! ```no_run
//! use auto_test::generate_tests_for_project;
//!
//! // Generate tests for the current directory
//! generate_tests_for_project(".")?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! ## Architecture
//!
//! The library is organized into several modules:
//!
//! - `core`: Core analysis and generation logic (see [`core`] module)
//! - `utils`: Utility functions for file I/O and filesystem operations
//! - `config`: Configuration management (see [`config`] module)
//! - `error`: Error types and handling (see [`error`] module)
//!
//! ## Features
//!
//! - **AST-based Analysis**: Uses `syn` to parse Rust source code accurately
//! - **Type-aware Generation**: Different parameter generation and assertions based on types
//! - **Integration Tests**: Generates tests that call your public API
//! - **Modular Organization**: Creates separate test files per module
//! - **CLI Tool**: Includes a command-line interface for easy usage
//! - **Configuration System**: User-configurable behavior via config files
//! - **Error Handling**: Comprehensive error propagation with helpful messages
//! - **Progress Indicators**: Provides feedback during long operations
//! - **Parallel Processing**: Fast generation on large codebases
//! - **Git Integration**: Respects .gitignore and skips irrelevant files
//!
//! ## Limitations
//!
//! - Currently only analyzes public functions (by design)
//! - TypeScript support is planned but not yet implemented
//! - Complex custom types fall back to `Default::default()`
//!
//! See the [README](https://github.com/yourusername/auto_test) for more information.

pub mod cli;
pub mod config;
pub mod core;
pub mod error;
pub mod utils;

/// Generate test files for a Rust project with default configuration.
///
/// This is the main entry point for generating integration tests.
/// Tests will be created in the `tests/` directory with default settings.
///
/// # Example
/// ```no_run
/// use auto_test::generate_tests_for_project;
///
/// generate_tests_for_project("./my_rust_project")?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn generate_tests_for_project(project_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let project_path = std::path::Path::new(project_path);
    let config = config::Config::load(project_path)?;
    generate_tests_for_project_with_config(project_path, &config)
}

/// Generate test files for a Rust project with custom configuration.
///
/// This is the enhanced entry point that supports all configuration options.
///
/// # Arguments
///
/// * `project_path` - Path to the project root directory
/// * `config` - Configuration for test generation behavior
///
/// # Returns
///
/// Success or an error if generation fails
///
/// # Example
/// ```no_run
/// use auto_test::{generate_tests_for_project_with_config, config::Config};
/// use std::path::Path;
///
/// let config = Config::default();
/// let project_path = Path::new("./my_project");
/// generate_tests_for_project_with_config(project_path, &config)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn generate_tests_for_project_with_config(
    project_path: &std::path::Path,
    config: &config::Config,
) -> Result<(), Box<dyn std::error::Error>> {
    let test_files =
        core::generator::rust_gen::RustGenerator::generate_with_config(project_path, config)?;

    for test_file in &test_files {
        eprintln!("Writing test file: {}", test_file.path);
        utils::fs::FsUtils::write_test_file_atomic(test_file)?;
    }

    // V Language Support
    use std::fs;
    use walkdir::WalkDir;

    for entry in WalkDir::new(project_path)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("v") {
            // Skip test files
            if path
                .file_name()
                .and_then(|s| s.to_str())
                .map(|s| s.ends_with("_test.v"))
                .unwrap_or(false)
            {
                continue;
            }

            let content = fs::read_to_string(path)?;
            let functions = core::v_lang::VParser::parse_function_signatures(&content);

            if !functions.is_empty() {
                let mut test_content = String::from("module main\n\n");
                for func in functions {
                    test_content.push_str(&core::v_lang::VParser::generate_test(&func));
                    test_content.push('\n');
                }

                let file_stem = path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("unknown");
                let test_file_name = format!("{}_test.v", file_stem);
                let test_file_path = path.parent().unwrap().join(&test_file_name);

                eprintln!("Writing V test file: {:?}", test_file_path);
                fs::write(test_file_path, test_content)?;
            }
        }
    }

    Ok(())
}
