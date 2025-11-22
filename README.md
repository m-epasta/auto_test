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

1. **Analysis**: Uses `syn` to parse Rust source files and extract public function signatures
2. **Type Inference**: Analyzes parameter types and return values
3. **Test Generation**: Creates integration tests with appropriate setup and assertions
4. **Organization**: Groups tests by module for maintainability

## Supported Types

The tool generates test parameters for:
- **Primitives**: `String`, `&str`, `i32`, `u64`, `bool`, etc.
- **Containers**: `Vec<T>`, `Option<T>`, `Result<T, E>`
- **References**: `&T`, `&mut T`
- **Custom Types**: Falls back to `Default::default()` for unknown structs

## Supported Assertions

Different assertion strategies based on return types:
- `Result<T, E>` → `assert!(result.is_ok())`
- `Option<T>` → `assert!(result.is_some())`
- `Vec<T>` → `assert!(!result.is_empty())`
- `String/&str` → `assert!(!result.is_empty())`
- Numbers → `assert!(result >= 0)`

## Limitations

- **Public functions only**: Currently analyzes only `pub` functions
- **Rust only**: No TypeScript support yet (planned)
- **Basic parameters**: Complex custom types use `Default::default()`
- **Integration tests**: Currently generates integration-style tests only

## Development

```bash
# Clone and build
git clone https://github.com/yourusername/auto_test.git
cd auto_test
cargo build

# Run tests
cargo test

# Run on example project
cargo run generate /path/to/example/project
```

## Contributing

Contributions are welcome! Please feel free to:
- Report bugs and issues
- Suggest new features
- Submit pull requests
- Improve documentation

## License

Licensed under the MIT License. See [LICENSE](LICENSE) for details.

## Changelog

See [CHANGELOG.md](CHANGELOG.md) for a list of changes and version history.
