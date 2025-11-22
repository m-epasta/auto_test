use crate::error::{AutoTestError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Configuration for AutoTest behavior.
///
/// This struct defines all user-configurable options for test generation.
/// It can be loaded from `auto_test.toml` or `auto_test.yaml` files.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct Config {
    /// Directory where generated tests will be written
    pub output_dir: String,
    /// Functions to skip during test generation
    pub skip_functions: Vec<String>,
    /// Custom type mappings for parameter generation
    pub type_mappings: HashMap<String, String>,
    /// Whether to include private functions in generation
    pub include_private: bool,
    /// Whether to use parallel processing
    pub parallel: bool,
    /// Maximum number of functions to process in parallel
    pub parallel_chunk_size: usize,
    /// Whether to respect .gitignore patterns
    pub respect_gitignore: bool,
    /// Additional file patterns to skip
    pub skip_patterns: Vec<String>,
    /// Timeout in seconds for individual operations
    pub timeout_seconds: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            output_dir: "tests".to_string(),
            skip_functions: Vec::new(),
            type_mappings: HashMap::new(),
            include_private: false,
            parallel: true,
            parallel_chunk_size: 25,
            respect_gitignore: true,
            skip_patterns: vec![
                "**/target/**".to_string(),
                "**/.git/**".to_string(),
                "**/node_modules/**".to_string(),
            ],
            timeout_seconds: 300,
        }
    }
}

impl Config {
    /// Load configuration from the standard locations in a project root.
    ///
    /// Looks for configuration files in this order:
    /// 1. auto_test.toml
    /// 2. auto_test.yaml
    /// 3. Default configuration
    ///
    /// # Arguments
    ///
    /// * `project_root` - Path to the project root directory
    ///
    /// # Returns
    ///
    /// The loaded configuration, or an error if loading fails
    pub fn load(project_root: &Path) -> Result<Self> {
        // Try TOML first
        let toml_path = project_root.join("auto_test.toml");
        if toml_path.exists() {
            return Self::load_from_file(&toml_path);
        }

        // Try YAML
        let yaml_path = project_root.join("auto_test.yaml");
        if yaml_path.exists() {
            return Self::load_from_file(&yaml_path);
        }

        // Fall back to defaults
        Ok(Self::default())
    }

    /// Load configuration from a specific file path.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the configuration file
    ///
    /// # Returns
    ///
    /// The loaded configuration, or an error if loading fails
    pub fn load_from_file(path: &Path) -> Result<Self> {
        let contents = std::fs::read_to_string(path)
            .map_err(|e| AutoTestError::FileRead {
                path: path.to_path_buf(),
                source: e,
            })?;

        match path.extension().and_then(|s| s.to_str()) {
            Some("toml") => {
                let config: Self = toml::from_str(&contents)
                    .map_err(|e| AutoTestError::InvalidConfig { message: format!("TOML parse error: {}", e) })?;
                Ok(config)
            }
            Some("yaml") | Some("yml") => {
                let config: Self = serde_yaml::from_str(&contents)?;
                Ok(config)
            }
            _ => Err(AutoTestError::InvalidConfig {
                message: "Unsupported configuration file format. Use .toml or .yaml".to_string(),
            }),
        }
    }

    /// Save the current configuration to a TOML file.
    ///
    /// # Arguments
    ///
    /// * `path` - Path where to save the configuration file
    ///
    /// # Returns
    ///
    /// Ok if saving succeeded, or an error
    pub fn save_to_file(&self, path: &Path) -> Result<()> {
        let contents = toml::to_string_pretty(self)
            .map_err(|e| AutoTestError::InvalidConfig { message: format!("TOML serialization error: {}", e) })?;

        std::fs::write(path, contents)
            .map_err(|e| AutoTestError::FileWrite {
                path: path.to_path_buf(),
                source: e,
            })?;

        Ok(())
    }

    /// Get the value for a type mapping, falling back to defaults.
    ///
    /// # Arguments
    ///
    /// * `type_name` - The type name to look up
    ///
    /// # Returns
    ///
    /// The mapped value if found, None otherwise
    pub fn get_type_mapping(&self, type_name: &str) -> Option<&String> {
        self.type_mappings.get(type_name)
    }

    /// Check if a function should be skipped based on configuration.
    ///
    /// # Arguments
    ///
    /// * `function_name` - The name of the function to check
    ///
    /// # Returns
    ///
    /// True if the function should be skipped
    pub fn should_skip_function(&self, function_name: &str) -> bool {
        self.skip_functions.iter().any(|skip| function_name.contains(skip))
    }
}

/// Find the project root by searching for common project indicators.
pub fn find_project_root(start_path: &Path) -> Result<PathBuf> {
    let mut current = start_path.canonicalize().map_err(|e| AutoTestError::Io { source: e })?;

    loop {
        // Check for Cargo.toml (Rust project)
        if current.join("Cargo.toml").exists() {
            return Ok(current);
        }

        // Check for package.json (Node.js project, might have Rust code)
        if current.join("package.json").exists() {
            return Ok(current);
        }

        // Check for common project files
        if ["lib.rs", "main.rs", "src"].iter().any(|file| current.join(file).exists()) {
            return Ok(current);
        }

        // Move up one directory
        if let Some(parent) = current.parent() {
            current = parent.to_path_buf();
        } else {
            break;
        }
    }

    Err(AutoTestError::ProjectRootNotFound {
        path: start_path.to_path_buf(),
    })
}

/// Load configuration from a project path.
///
/// This is a convenience function that finds the project root and loads configuration.
///
/// # Arguments
///
/// * `project_path` - Path within the project (doesn't need to be the root)
///
/// # Returns
///
/// The loaded configuration
pub fn load_config(project_path: &Path) -> Result<Config> {
    let project_root = find_project_root(project_path)?;
    Config::load(&project_root)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.output_dir, "tests");
        assert!(!config.include_private);
        assert!(config.respect_gitignore);
    }

    #[test]
    fn test_should_skip_function() {
        let mut config = Config::default();
        config.skip_functions = vec!["test_".to_string(), "skip_me".to_string()];

        assert!(config.should_skip_function("test_function"));
        assert!(config.should_skip_function("some_skip_me_function"));
        assert!(!config.should_skip_function("normal_function"));
    }

    #[test]
    fn test_load_from_toml_file() {
        let temp_dir = tempdir().unwrap();
        let config_path = temp_dir.path().join("auto_test.toml");

        let toml_content = r#"
output_dir = "custom_tests"
include_private = true
skip_functions = ["internal_", "test_"]
respect_gitignore = false
"#;

        fs::write(&config_path, toml_content).unwrap();

        let config = Config::load_from_file(&config_path).unwrap();
        assert_eq!(config.output_dir, "custom_tests");
        assert!(config.include_private);
        assert!(config.should_skip_function("internal_func"));
        assert!(!config.respect_gitignore);
    }

    #[test]
    fn test_load_from_yaml_file() {
        let temp_dir = tempdir().unwrap();
        let config_path = temp_dir.path().join("auto_test.yaml");

        let yaml_content = r#"
output_dir: "yaml_tests"
parallel: false
skip_patterns:
  - "**/docs/**"
  - "**/examples/**"
type_mappings:
  MyCustomType: "MyCustomType::new()"
"#;

        fs::write(&config_path, yaml_content).unwrap();

        let config = Config::load_from_file(&config_path).unwrap();
        assert_eq!(config.output_dir, "yaml_tests");
        assert!(!config.parallel);
        assert!(config.skip_patterns.contains(&"**/docs/**".to_string()));
        assert_eq!(config.get_type_mapping("MyCustomType").unwrap(), "MyCustomType::new()");
    }
}
