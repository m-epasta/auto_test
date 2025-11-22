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
//!
//! ## Features
//!
//! - **AST-based Analysis**: Uses `syn` to parse Rust source code accurately
//! - **Type-aware Generation**: Different parameter generation and assertions based on types
//! - **Integration Tests**: Generates tests that call your public API
//! - **Modular Organization**: Creates separate test files per module
//! - **CLI Tool**: Includes a command-line interface for easy usage
//! - **Error Handling**: Comprehensive error propagation with helpful messages
//!
//! ## Limitations
//!
//! - Currently only analyzes public functions (by design)
//! - TypeScript support is planned but not yet implemented
//! - Complex custom types fall back to `Default::default()`
//!
//! See the [README](https://github.com/yourusername/auto_test) for more information.

pub mod core;
pub mod utils;

/// Generate test files for a Rust project.
/// 
/// This is the main entry point for generating integration tests.
/// Tests will be created in the `tests/` directory.
/// 
/// # Example
/// ```
/// use auto_test::generate_tests_for_project;
/// 
/// generate_tests_for_project("./my_rust_project")?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn generate_tests_for_project(project_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let analysis = core::analyzer::analyze_rust_project(project_path);
    let test_files = core::generator::rust_gen::RustGenerator::generate(&analysis);
    
    println!("Analyzed project: {}", project_path);
    println!("Found {} public functions across {} modules",
             analysis.functions.len(),
             test_files.len());

    for test_file in &test_files {
        println!("Generated: {}", test_file.path);
        utils::fs::FsUtils::write_test_file(test_file)?;
    }

    println!("Successfully generated {} test files", test_files.len());
    Ok(())
}
