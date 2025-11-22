use clap::Parser;
use std::path::PathBuf;
use crate::config::{Config, find_project_root};


#[derive(Parser)]
pub struct GenerateArgs {
    /// Path to the project root
    pub path: String,

    /// Path to custom configuration file (auto_test.toml or auto_test.yaml)
    #[arg(long)]
    pub config_path: Option<PathBuf>,

    /// Output directory for tests (overrides config file)
    #[arg(long)]
    pub output_dir: Option<String>,

    /// Include private functions with #[cfg(test)] access
    #[arg(long)]
    pub include_private: bool,

    /// Skip tests for functions with these prefixes
    #[arg(long)]
    pub skip_prefixes: Vec<String>,

    /// Disable parallel processing (use sequential)
    #[arg(long)]
    pub no_parallel: bool,

    /// Do not respect .gitignore patterns
    #[arg(long)]
    pub no_gitignore: bool,
}


pub fn handle(args: GenerateArgs) -> Result<(), Box<dyn std::error::Error>> {
    let project_path = PathBuf::from(&args.path);

    // Load configuration
    let mut config = if let Some(config_path) = &args.config_path {
        // Load from specified config file
        Config::load_from_file(config_path)?
    } else {
        // Load from project root (auto-detection)
        let project_root = find_project_root(&project_path)
            .map_err(|e| format!("Could not find project root: {}", e))?;
        Config::load(&project_root)?
    };

    // Override config with CLI arguments
    if let Some(output_dir) = args.output_dir {
        config.output_dir = output_dir;
    }

    if args.include_private {
        config.include_private = true;
    }

    if !args.skip_prefixes.is_empty() {
        config.skip_functions.extend(args.skip_prefixes);
    }

    if args.no_parallel {
        config.parallel = false;
    }

    if args.no_gitignore {
        config.respect_gitignore = false;
    }

    // Generate tests with configuration
    crate::generate_tests_for_project_with_config(&project_path, &config)
}
