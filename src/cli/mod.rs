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

pub fn run() {
    let cli: Cli = Cli::parse();

    match cli.command {
        Commands::Generate(args) => generate::handle(args),
    }
}
