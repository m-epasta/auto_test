# Changelog

All notable changes to `auto_test` will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Initial release of auto_test
- AST-based analysis of Rust source code
- Integration test generation with type-aware assertions
- CLI tool with subcommands for project generation
- Library API for programmatic use
- Modular test file organization by module
- Support for common Rust types (String, Vec, Option, Result, primitives)
- Automatic test parameter generation
- Appropriate assertions based on return types

### Technical Improvements
- Proper dual crate architecture (library + binary)
- Error handling with `Result<>` throughout
- Type-safe AST processing with `syn` and `quote`
- Modular codebase with clear separation of concerns
- Integration test structure following Rust conventions

---

## [0.1.0] - 2025-11-22

### Added
- **Core Analysis Engine**: Implemented AST-based Rust code analysis using `syn`
- **Test Generation**: Created modular test generator with type-aware assertions
- **CLI Interface**: Added command-line tool with clap for argument parsing
- **Library API**: Exposed `generate_tests_for_project()` function for programmatic use
- **Integration Tests**: Generated integration-style tests that call public APIs
- **Type Support**: Added parameter generation for common types:
  - Primitives: `String`, `&str`, `bool`, integers
  - Containers: `Vec<T>`, `Option<T>`, `Result<T, E>`
  - References: `&T`, `&mut T`
- **Assertion Strategies**: Different assertions based on return types:
  - `Result→assert!(result.is_ok())`
  - `Option→assert!(result.is_some())`
  - `Vec→assert!(!result.is_empty())`
  - Numbers→`assert!(result >= 0)`

### Technical Details
- **Dependencies**: `syn = "2"`, `quote = "1.0.42"`, `clap = "4"`, `serde`
- **Architecture**: Library-first design with thin CLI wrapper
- **File Organization**: One test file per module in `tests/` directory
- **Error Handling**: Comprehensive `Result<>` propagation with clean error messages
- **Testing**: Both unit tests and integration tests included

### Known Limitations
- Only analyzes public functions (by design)
- TypeScript analyzer present but unimplemented (stub)
- Complex custom structs fall back to `Default::default()`
- Generated tests need manual completion of assertions

---

## Development History

### Initial Development (2025-11-15 to 2025-11-22)

- **Phase 1: Core Architecture** (Nov 15-17)
  - Set up Cargo workspace and module structure
  - Implemented basic AST parsing with `syn`
  - Created data models for FunctionInfo and ProjectInfo
  - Added file system utilities

- **Phase 2: Analysis Engine** (Nov 17-19)
  - Implemented `rust_analyzer.rs` for parsing Rust functions
  - Added TypeScript analyzer stub for future extension
  - Created `analyze_rust_project()` function for directory scanning
  - Added filtering for public functions only

- **Phase 3: Code Generation** (Nov 19-20)
  - Built `RustGenerator` for test stub creation
  - Implemented type-aware parameter generation
  - Added assertion strategies for different return types
  - Created integration test structure

- **Phase 4: CLI and Library API** (Nov 20-21)
  - Added clap-based CLI with subcommands
  - Created library-first architecture with `generate_tests_for_project()`
  - Implemented modular test file organization
  - Added comprehensive error handling

- **Phase 5: Polish and Documentation** (Nov 21-22)
  - Cleaned up code and removed unused functions
  - Added comprehensive documentation
  - Created professional README and CHANGELOG
  - Added tests and CI readiness

---

## Future Plans

### Planned for v0.2.0
- [ ] **TypeScript Support**: Complete TypeScript AST analyzer implementation
- [ ] **Unit Test Mode**: Option to generate unit tests instead of integration tests
- [ ] **Configuration File**: Custom assertion patterns and type mappings
- [ ] **Async Function Analysis**: Better handling of Rust async functions

### Planned for v0.3.0
- [ ] **Custom Test Templates**: User-defined test code snippets
- [ ] **Mock Generation**: Automatic dependency injection and mocking
- [ ] **CI/CD Integration**: GitHub Actions, GitLab CI templates
- [ ] **WebAssembly Support**: Browser-based test generation interface

---

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for contribution guidelines.

## Acknowledgments

- Built with [syn](https://crates.io/crates/syn) for AST parsing
- CLI powered by [clap](https://crates.io/crates/clap)
- Inspired by the need for automated test scaffolding in Rust projects
