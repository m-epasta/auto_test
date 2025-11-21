use clap::Parser;
use crate::core::analyzer::analyze_rust_project;


#[derive(Parser)]
pub struct GenerateArgs {
    /// Path to the project root
    pub path: String,
}


pub fn handle(args: GenerateArgs) {
    let project_info: crate::core::models::ProjectInfo = analyze_rust_project(&args.path);
    println!("Generating tests for: {}", args.path);

    println!("Found {} functions", project_info.functions.len());
    for func in &project_info.functions {
        println!("{} in ({})", func.name, func.file);
    }


}