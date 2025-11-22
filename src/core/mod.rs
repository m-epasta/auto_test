//! # Core Module
//!
//! Core functionality for analyzing Rust code and generating tests.
//!
//! This module contains three main submodules:
//!
//! ## Modules
//!
//! - [`analyzer`]: Parses Rust source code and extracts function signatures
//! - [`models`]: Data structures representing analyzed functions and projects
//! - [`generator`]: Generates test code from analyzed data

pub mod analyzer;
pub mod models;
pub mod generator;
