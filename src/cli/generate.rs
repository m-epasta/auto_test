use clap::Parser;

#[derive(Parser)]
pub struct GenerateArgs {
    /// Path to the project root
    pub path: String,
}

pub fn handle(args: GenerateArgs) -> Result<(), Box<dyn std::error::Error>> {
    // Use the library function for the actual work
    auto_test::generate_tests_for_project(&args.path)
}
