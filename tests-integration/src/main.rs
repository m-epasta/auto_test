use auto_test::generate_tests_for_project;
use std::process::Command;
use std::path::Path;
use std::fs;
use tempfile::TempDir;

/// Main entry point for integration tests.
/// This binary validates the complete AutoTest pipeline from code analysis
/// to test generation and compilation verification.
fn main() {
    println!("Running AutoTest Integration Tests...");

    // Test 1: Basic compilation test
    if let Err(e) = test_basic_compilation() {
        eprintln!("Error: Basic compilation test failed: {}", e);
        std::process::exit(1);
    }
    println!("Basic compilation test passed");

    // Test 2: Memory optimization test
    if let Err(e) = test_memory_optimization() {
        eprintln!("Error: Memory optimization test failed: {}", e);
        std::process::exit(1);
    }
    println!("Memory optimization test passed");

    // Test 3: Large project scalability test
    if let Err(e) = test_large_project_scalability() {
        eprintln!("Error: Large project scalability test failed: {}", e);
        std::process::exit(1);
    }
    println!("Large project scalability test passed");

    println!("All integration tests passed");
}

fn test_basic_compilation() -> Result<(), Box<dyn std::error::Error>> {
    println!("  Testing basic test compilation...");

    let temp_dir = TempDir::new()?;
    let project_path = temp_dir.path();

    create_test_project(project_path)?;
    generate_tests_for_project(project_path.to_str().unwrap())?;

    // Run cargo test on generated tests
    let output = Command::new("cargo")
        .args(&["test", "--quiet"])
        .current_dir(project_path)
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Generated tests failed to compile: {}", stderr).into());
    }

    Ok(())
}

fn test_memory_optimization() -> Result<(), Box<dyn std::error::Error>> {
    println!("  Testing memory optimization...");

    // Test string interning
    use auto_test::core::models::TypeIntern;

    let type1 = TypeIntern::new("String");
    let type2 = TypeIntern::new("String");
    let type3 = TypeIntern::new("i32");

    // Verify interning works
    assert_eq!(type1, type2, "String interning failed: identical strings should be equal");
    assert_ne!(type1, type3, "String interning failed: different strings should not be equal");

    // Test serialization round-trip
    let serialized = serde_json::to_string(&type1)?;
    let deserialized: TypeIntern = serde_json::from_str(&serialized)?;

    assert_eq!(type1, deserialized, "Serialization round-trip failed");

    Ok(())
}

fn test_large_project_scalability() -> Result<(), Box<dyn std::error::Error>> {
    println!("  Testing large project scalability...");

    let temp_dir = TempDir::new()?;
    let project_path = temp_dir.path();

    create_large_test_project(project_path, 100)?; // 100 functions

    let start = std::time::Instant::now();
    generate_tests_for_project(project_path.to_str().unwrap())?;
    let duration = start.elapsed();

    println!("    Generated tests for 100 functions in {:.2}s", duration.as_secs_f64());

    // Should complete in reasonable time (< 10 seconds on modern hardware)
    if duration.as_secs() > 10 {
        return Err(format!("Performance test failed: took {}s", duration.as_secs()).into());
    }

    // Verify generated tests compile
    let output = Command::new("cargo")
        .args(&["check", "--quiet"])
        .current_dir(project_path)
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Generated tests for large project failed: {}", stderr).into());
    }

    Ok(())
}

fn create_test_project(project_root: &Path) -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all(project_root.join("src"))?;

    let cargo_toml = r#"
[package]
name = "test_project"
version = "0.1.0"
edition = "2021"

[lib]
name = "test_project"
path = "src/lib.rs"
"#;
    fs::write(project_root.join("Cargo.toml"), cargo_toml)?;

    let lib_rs = r#"
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn process_string(input: String) -> String {
    format!("Processed: {}", input)
}

pub fn check_result(value: bool) -> Result<(), String> {
    if value { Ok(()) } else { Err("failed".to_string()) }
}

pub fn get_items() -> Vec<String> {
    vec!["item1".to_string(), "item2".to_string()]
}
"#;
    fs::write(project_root.join("src").join("lib.rs"), lib_rs)?;

    Ok(())
}

fn create_large_test_project(project_root: &Path, function_count: usize) -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all(project_root.join("src"))?;

    let cargo_toml = r#"
[package]
name = "large_test_project"
version = "0.1.0"
edition = "2021"

[lib]
name = "large_test_project"
path = "src/lib.rs"
"#;
    fs::write(project_root.join("Cargo.toml"), cargo_toml)?;

    let mut lib_rs = String::from("pub mod functions {\n    use std::collections::HashMap;\n");

    // Generate many functions to test scalability
    for i in 0..function_count {
        lib_rs.push_str(&format!("
    pub fn func_{}(input: i32) -> i32 {{
        input + {}
    }}", i, i));
    }

    // Add various function types to test generation diversity
    lib_rs.push_str(r#"

    pub fn process_string(input: String) -> String {
        format!("Processed: {}", input)
    }

    pub fn check_condition(value: bool) -> Result<(), String> {
        if value { Ok(()) } else { Err("condition failed".to_string()) }
    }

    pub fn get_list() -> Vec<String> {
        vec!["item1".to_string(), "item2".to_string()]
    }

    pub fn create_map() -> HashMap<String, i32> {
        let mut map = HashMap::new();
        map.insert("key1".to_string(), 1);
        map.insert("key2".to_string(), 2);
        map
    }
"#);

    lib_rs.push_str("\n}\n");
    fs::write(project_root.join("src").join("lib.rs"), lib_rs)?;

    Ok(())
}
