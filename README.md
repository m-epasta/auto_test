# AutoTest

[![Crates.io](https://img.shields.io/crates/v/auto_test.svg)](https://crates.io/crates/auto_test)
[![Documentation](https://docs.rs/auto_test/badge.svg)](https://docs.rs/auto_test)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

A Rust library and CLI tool for automatically generating test stubs for Rust projects. Uses AST analysis to understand your code structure and create meaningful integration tests.

## Features

- 🔍 **AST-based Analysis**: Analyzes Rust source code using `syn` and `quote`
- 🧪 **Integration Tests**: Generates integration tests that call your public API
- 📁 **Modular Organization**: Creates separate test files for each module
- 🛠️ **CLI Tool**: Command-line interface for easy project integration
- 📚 **Library API**: Programmatic access for custom tooling and CI/CD integration
- 🏗️ **Type-Aware**: Generates appropriate assertions based on return types
- 🧹 **Clean Architecture**: Follows Rust best practices with proper error handling

## Installation

### As a Cargo binary
```bash
cargo install auto_test
```

### As a library dependency
```toml
[dependencies]
auto_test = "0.1"
```

## Usage

### Command Line

Generate tests for your entire Rust project:
```bash
auto_test generate /path/to/your/rust/project
```

Or from within a project directory:
```bash
cd /path/to/your/rust/project
auto_test generate .
```

### Configuration

AutoTest supports advanced hierarchical configuration for enterprise workflows. Create an `.auto_test.toml` or `.auto_test.yaml` file in your project root:

```toml
# Project metadata for GitOps workflows
[project]
name = "my_service"
baseline_branch = "main"

# Generation strategy and behavior
[generation]
strategy = "integration"  # "integration", "unit", or "property"
output_dir = "tests"
skip_functions = ["internal_", "test_"]
timeout_seconds = 120

# Custom assertion patterns
[generation.custom_assertions]
"MyResult" = "assert_matches!(result, MyResult::Ok(_))"
"MyError" = "assert!(result.is_err())"

# Type-safe parameter generation
[types]
constructor_inference = true
builder_detection = true

[types.mappings]
"MyDomainType" = "MyDomainType::builder().build()"
"ComplexType" = "ComplexType::new(\"default\")"

# Performance and execution control
[performance]
parallel = true
parallel_chunk_size = 25
memory_limit_mb = 512
caching_enabled = false

# File discovery and filtering
[filesystem]
respect_gitignore = true
skip_patterns = [
    "**/target/**",
    "**/node_modules/**",
    "**/dist/**"
]
```

### Library API

```rust
use auto_test::generate_tests_for_project;

// Generate integration tests for a project
match generate_tests_for_project("./my_project") {
    Ok(()) => println!("Tests generated successfully!"),
    Err(e) => eprintln!("Error: {}", e),
}
```

## Example Output

For a project with this structure:
```
src/
├── lib.rs
├── cli/
│   ├── mod.rs
│   └── generate.rs
└── core/
    ├── mod.rs
    └── analyzer.rs
```

AutoTest generates:
```
tests/
├── integration_tests.rs
├── cli_generate_tests.rs
└── core_analyzer_rust_analyzer_tests.rs
```

Each test file contains integration tests that call your public APIs:

```rust
#[cfg(test)]
mod tests {
    use auto_test::*;

    #[test]
    fn test_your_function_integration() {
        // Arrange
        // (generated parameter setup)

        // Act
        let result = auto_test::generate_tests_for_project("/tmp/test");

        // Assert
        assert!(result.is_ok());
    }
}
```

## How It Works

1. **Analysis**: Parses Rust source code to extract public function signatures
2. **Type Inference**: Analyzes parameter types and return values
3. **Test Generation**: Creates integration tests with appropriate setup and assertions
4. **Organization**: Groups tests by module for maintainability

## Supported Types

AutoTest generates test parameters for common Rust types:
- **Primitives**: `String`, `&str`, `i32`, `u64`, `bool`, and other primitive types
- **Collections**: `Vec<T>`, `Option<T>`, and other standard library types
- **References**: `&T`, `&mut T` reference types
- **Custom Types**: Falls back to `Default::default()` for complex structs

## Supported Assertions

AutoTest generates appropriate assertions based on return types:
- `Result<T, E>` → Checks for successful operation
- `Option<T>` → Ensures value is present
- `Vec<T>` → Verifies collection is not empty
- `String/&str` → Confirms content is not empty
- Numbers → Validates expected value ranges

## Limitations

- Analyzes only public functions by design
- Currently supports Rust only
- Complex custom types use default values
- Generates integration-style tests

## License

Licensed under the MIT License. See [LICENSE](LICENSE) for details.

## Changelog

See [CHANGELOG.md](CHANGELOG.md) for a list of changes and version history.
