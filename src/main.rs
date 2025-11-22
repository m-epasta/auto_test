mod cli;
mod core;
mod utils;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    cli::run()
}
