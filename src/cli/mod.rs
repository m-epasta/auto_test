use tracing::info;
use tracing_subscriber;

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
    // Initialize structured logging
    tracing_subscriber::fmt()
        .json()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let cli: Cli = Cli::try_parse()?;

    info!(
        command = "cli_start",
        version = env!("CARGO_PKG_VERSION"),
        "AutoTest CLI started"
    );

    let result = match cli.command {
        Commands::Generate(args) => generate::handle(args),
    };

    match &result {
        Ok(_) => info!(command = "cli_complete", "AutoTest CLI completed successfully"),
        Err(e) => tracing::error!(command = "cli_error", error = %e, "AutoTest CLI failed"),
    }

    result
}
