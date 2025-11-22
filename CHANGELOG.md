# Changelog

All notable changes to this project will be documented in this file.

## [0.1.0] - 2025-11-22

### Added
- Initial release of AutoTest CLI tool and library
- AST-based analysis of Rust source code for automatic test generation
- Integration test generation with appropriate assertions based on return types
- CLI interface with simple `generate` command for project-wide test creation
- Library API for programmatic use in CI/CD pipelines and custom tooling
- Support for common Rust types: primitives, collections, and references
- Modular test organization with one file per module
- Automatic parameter generation with sensible defaults

### Fixed
- Proper error handling throughout the application
- Clean and predictable test file generation process
- Integration-style tests that call your public API

### Limitations
- Analyzes only public functions by design
- Supports Rust projects only
- Complex custom types fall back to default values
- Generates integration tests (unit test mode planned for future version)
