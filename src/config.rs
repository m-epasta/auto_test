//! # Configuration Management
//!
//! Advanced hierarchical configuration system supporting GitOps workflows
//! and cascading configuration sources with environment variable overrides.

use crate::error::{AutoTestError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
/// Enhanced hierarchical configuration for GitOps-style workflows.
///
/// Supports cascading configuration sources with environment override capabilities:
/// 1. Global user config (~/.config/auto_test/{config.toml,yaml})
/// 2. Project-specific config ({project}/.auto_test.{toml,yaml})
/// 3. Environment variables (AUTO_TEST_*)
/// 4. Inline overrides via CLI flags
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct Config {
    /// Project metadata for GitOps workflows
    #[serde(rename = "project")]
    pub project: ProjectConfig,

    /// Generation strategy and behavior
    #[serde(rename = "generation")]
    pub generation: GenerationConfig,

    /// Type-safe parameter generation
    #[serde(rename = "types")]
    pub types: TypeConfig,

    /// Performance and execution control
    #[serde(rename = "performance")]
    pub performance: PerformanceConfig,

    /// File discovery and filtering
    #[serde(rename = "filesystem")]
    pub filesystem: FilesystemConfig,

    // Legacy fields for backward compatibility
    #[serde(skip)]
    pub output_dir: String,
    #[serde(skip)]
    pub skip_functions: Vec<String>,
    #[serde(skip)]
    pub type_mappings: HashMap<String, String>,
    #[serde(skip)]
    pub include_private: bool,
    #[serde(skip)]
    pub parallel: bool,
    #[serde(skip)]
    pub parallel_chunk_size: usize,
    #[serde(skip)]
    pub respect_gitignore: bool,
    #[serde(skip)]
    pub skip_patterns: Vec<String>,
    #[serde(skip)]
    pub timeout_seconds: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct ProjectConfig {
    /// Project name for telemetry and caching
    pub name: Option<String>,
    /// Baseline branch for incremental generation
    pub baseline_branch: Option<String>,
    /// Version for compatibility checking
    pub version: Option<String>,
}

impl Default for ProjectConfig {
    fn default() -> Self {
        Self {
            name: None,
            baseline_branch: Some("main".to_string()),
            version: None,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct GenerationConfig {
    /// Test generation strategy: "integration", "unit", "property"
    pub strategy: String,
    /// Directory where generated tests are written
    pub output_dir: String,
    /// Functions to skip during generation (patterns)
    pub skip_functions: Vec<String>,
    /// Custom assertion patterns for types
    pub custom_assertions: HashMap<String, String>,
    /// Timeout in seconds for individual operations
    pub timeout_seconds: u64,
    /// Whether to include private functions
    pub include_private: bool,
}

impl Default for GenerationConfig {
    fn default() -> Self {
        Self {
            strategy: "integration".to_string(),
            output_dir: "tests".to_string(),
            skip_functions: Vec::new(),
            custom_assertions: HashMap::new(),
            timeout_seconds: 300,
            include_private: false,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct TypeConfig {
    /// Custom type constructors and mappings
    pub mappings: HashMap<String, String>,
    /// Constructor inference strategies
    pub constructor_inference: bool,
    /// Builder pattern detection
    pub builder_detection: bool,
}

impl Default for TypeConfig {
    fn default() -> Self {
        let mut mappings = HashMap::new();
        mappings.insert("PathBuf".to_string(), "std::path::PathBuf::from(\".\")".to_string());
        mappings.insert("Uuid".to_string(), "uuid::Uuid::new_v4()".to_string());

        Self {
            mappings,
            constructor_inference: true,
            builder_detection: true,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct PerformanceConfig {
    /// Parallel processing enablement
    pub parallel: bool,
    /// Maximum functions processed in parallel
    pub parallel_chunk_size: usize,
    /// Memory limit in MB for bounded processing
    pub memory_limit_mb: Option<usize>,
    /// Enable result caching
    pub caching_enabled: bool,
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            parallel: true,
            parallel_chunk_size: 25,
            memory_limit_mb: None,
            caching_enabled: false,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct FilesystemConfig {
    /// Respect .gitignore patterns
    pub respect_gitignore: bool,
    /// Additional file patterns to skip
    pub skip_patterns: Vec<String>,
}

impl Default for FilesystemConfig {
    fn default() -> Self {
        Self {
            respect_gitignore: true,
            skip_patterns: vec![
                "**/target/**".to_string(),
                "**/.git/**".to_string(),
                "**/node_modules/**".to_string(),
            ],
        }
    }
}

// Legacy fields for backward compatibility
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct LegacyConfig {
    pub output_dir: String,
    pub skip_functions: Vec<String>,
    pub type_mappings: HashMap<String, String>,
    pub include_private: bool,
    pub parallel: bool,
    pub parallel_chunk_size: usize,
    pub respect_gitignore: bool,
    pub skip_patterns: Vec<String>,
    pub timeout_seconds: u64,
}

impl Default for LegacyConfig {
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

impl From<LegacyConfig> for Config {
    fn from(legacy: LegacyConfig) -> Self {
        Self {
            // Use defaults for new hierarchical fields
            project: ProjectConfig::default(),
            generation: GenerationConfig {
                strategy: "integration".to_string(),
                output_dir: legacy.output_dir.clone(),
                skip_functions: legacy.skip_functions.clone(),
                custom_assertions: HashMap::new(),
                timeout_seconds: legacy.timeout_seconds,
                include_private: legacy.include_private,
            },
            types: TypeConfig {
                mappings: legacy.type_mappings.clone(),
                constructor_inference: true,
                builder_detection: true,
            },
            performance: PerformanceConfig {
                parallel: legacy.parallel,
                parallel_chunk_size: legacy.parallel_chunk_size,
                memory_limit_mb: None,
                caching_enabled: false,
            },
            filesystem: FilesystemConfig {
                respect_gitignore: legacy.respect_gitignore,
                skip_patterns: legacy.skip_patterns.clone(),
            },
            // Legacy fields preserved
            output_dir: legacy.output_dir,
            skip_functions: legacy.skip_functions,
            type_mappings: legacy.type_mappings,
            include_private: legacy.include_private,
            parallel: legacy.parallel,
            parallel_chunk_size: legacy.parallel_chunk_size,
            respect_gitignore: legacy.respect_gitignore,
            skip_patterns: legacy.skip_patterns,
            timeout_seconds: legacy.timeout_seconds,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            project: ProjectConfig::default(),
            generation: GenerationConfig::default(),
            types: TypeConfig::default(),
            performance: PerformanceConfig::default(),
            filesystem: FilesystemConfig::default(),
            // Legacy fields
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
    /// Supports both legacy flat format and new hierarchical format.
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
    /// Handles both legacy flat format and new hierarchical format for
    /// backward compatibility.
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

        let config = match path.extension().and_then(|s| s.to_str()) {
            Some("toml") => {
                Self::load_toml_with_fallback(&contents)?
            }
            Some("yaml") | Some("yml") => {
                Self::load_yaml_with_fallback(&contents)?
            }
            _ => return Err(AutoTestError::InvalidConfig {
                message: "Unsupported configuration file format. Use .toml or .yaml".to_string(),
            }),
        };

        // Sync legacy fields with hierarchical structure
        Ok(config.sync_legacy_fields())
    }

    /// Load TOML content, trying legacy first then upgrading to hierarchical format
    fn load_toml_with_fallback(contents: &str) -> Result<Self> {
        // Try legacy format first for backward compatibility
        if let Ok(legacy) = toml::from_str::<LegacyConfig>(contents) {
            return Ok(legacy.into());
        }

        // Try hierarchical format for new configs
        if let Ok(config) = toml::from_str::<Self>(contents) {
            return Ok(config);
        }

        // Parse error
        Err(AutoTestError::InvalidConfig {
            message: "Invalid TOML configuration format".to_string(),
        })
    }

    /// Load YAML content, trying legacy first then upgrading to hierarchical format
    fn load_yaml_with_fallback(contents: &str) -> Result<Self> {
        // Try legacy format first for backward compatibility
        if let Ok(legacy) = serde_yaml::from_str::<LegacyConfig>(contents) {
            return Ok(legacy.into());
        }

        // Try hierarchical format for new configs
        if let Ok(config) = serde_yaml::from_str::<Self>(contents) {
            return Ok(config);
        }

        // Parse error
        Err(AutoTestError::InvalidConfig {
            message: "Invalid YAML configuration format".to_string(),
        })
    }

    /// Synchronize legacy fields to match hierarchical structure
    fn sync_legacy_fields(mut self) -> Self {
        // Copy from hierarchical to legacy fields for backward compatibility
        self.output_dir = self.generation.output_dir.clone();
        self.skip_functions = self.generation.skip_functions.clone();
        self.type_mappings = self.types.mappings.clone();
        self.include_private = self.generation.include_private;
        self.parallel = self.performance.parallel;
        self.parallel_chunk_size = self.performance.parallel_chunk_size;
        self.respect_gitignore = self.filesystem.respect_gitignore;
        self.skip_patterns = self.filesystem.skip_patterns.clone();
        self.timeout_seconds = self.generation.timeout_seconds;

        self
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
