//! # CLI Module
//!
//! Command-line interface for AutoTest.
//!
//! This module provides a user-friendly command-line interface to the AutoTest
//! functionality. It uses the [`clap`] crate for argument parsing and provides
//! clear help text and subcommands.
//!
//! ## Usage
//!
//! ```bash
//! # Generate tests for a project
//! auto_test generate /path/to/project
//!
//! # Show help
//! auto_test --help
//! ```
//!
//! The CLI acts as a thin wrapper around the library functionality, providing
//! error handling and formatted output for command-line users.

use clap::{Parser, Subcommand};

mod generate;

#[derive(Parser)]
#[command(name = "autotest")]
#[command(version = "0.1.0")]
#[command(about = "Generate automated tests for Rust & TS projects")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Generate tests for a project
    Generate(generate::GenerateArgs),
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let cli: Cli = Cli::parse();

    match cli.command {
        Commands::Generate(args) => generate::handle(args),
    }
}
