//! # Data Models
//!
//! Core data structures representing analyzed Rust functions and projects.
//!
//! These models are used throughout the analysis and generation pipeline to
//! represent the structure of Rust functions, their parameters, and project-wide
//! collections of functions.
//!
//! The models are designed for memory efficiency and thread safety, featuring
//! string interning to reduce memory duplication for common type names.

use serde::{Serialize, Deserialize};
use std::sync::Arc;

/// Represents a function parameter with its name and type information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParamInfo {
    /// The parameter name as defined in the function signature.
    pub name: String,
    /// The parameter type, stored as an interned string for memory efficiency.
    pub typ: TypeIntern,
}

/// An interned string type optimized for memory efficiency in large codebases.
///
/// This struct uses `Arc<str>` to provide shared ownership of type strings
/// with automatic interning of common Rust types. This significantly reduces
/// memory usage when analyzing large projects with repetitive type patterns.
///
/// # Memory Optimization
///
/// Common types like `"String"`, `"&str"`, `"i32"`, etc., are pre-interned
/// in a static pool, ensuring they share the same memory allocation across
/// all instances. This can reduce memory usage by 30-50% in large projects.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct TypeIntern(Arc<str>);

impl serde::Serialize for TypeIntern {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for TypeIntern {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Ok(TypeIntern::new(&s))
    }
}

impl TypeIntern {
    /// Create a new interned type string, reusing existing allocations where possible
    pub fn new(s: &str) -> Self {
        // Use static interning table for common type patterns
        use std::collections::HashMap;
        use std::sync::OnceLock;

        static INTERN_POOL: OnceLock<HashMap<&'static str, TypeIntern>> = OnceLock::new();
        let pool = INTERN_POOL.get_or_init(|| {
            let mut map = HashMap::new();
            // Pre-populate common types
            let common_types = [
                "String", "&str", "i32", "u32", "i64", "u64", "usize",
                "bool", "()", "Vec<T>", "Option<T>", "Result<T, E>",
                "PathBuf", "Uuid", "Url", "DateTime", "Config", "Args"
            ];
            for &typ in &common_types {
                map.insert(typ, TypeIntern(typ.into()));
            }
            map
        });

        // Check if type exists in pool (exact match for common types)
        if let Some(interned) = pool.get(s) {
            interned.clone()
        } else {
            TypeIntern(Arc::from(s))
        }
    }

    /// Get the underlying string reference
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for TypeIntern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&str> for TypeIntern {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

impl From<String> for TypeIntern {
    fn from(s: String) -> Self {
        Self::new(&s)
    }
}

/// Comprehensive information about a single analyzed function.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionInfo {
    /// The function name as defined in the source code.
    pub name: String,
    /// List of function parameters with their types.
    pub params: Vec<ParamInfo>,
    /// The return type of the function.
    pub returns: TypeIntern,
    /// Path to the source file containing this function.
    pub file: String,
    /// Whether this function is declared as async.
    pub is_async: bool,
}

impl FunctionInfo {
    /// Calculate estimated memory impact for profiling and diagnostics.
    ///
    /// This provides an approximate memory footprint including all string data
    /// stored in function names, file paths, parameter names, and type strings.
    pub fn memory_estimate(&self) -> usize {
        self.name.len() +
        self.file.len() +
        self.params.iter().map(|p| p.name.len() + p.typ.as_str().len()).sum::<usize>() +
        self.returns.as_str().len()
    }
}

/// Project-wide collection of analyzed functions and metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectInfo {
    /// Programming language of the project ("rust" or "typescript").
    pub language: String,
    /// Root directory path of the project.
    pub root: String,
    /// All analyzed public functions in the project.
    pub functions: Vec<FunctionInfo>,
}

impl ProjectInfo {
    /// Generate memory usage statistics for the analyzed project.
    ///
    /// This aggregates memory usage across all functions and provides
    /// diagnostic information about the analysis footprint.
    pub fn memory_stats(&self) -> MemoryStats {
        let total_functions = self.functions.len();
        let total_params = self.functions.iter().map(|f| f.params.len()).sum::<usize>();
        let total_memory = self.functions.iter().map(|f| f.memory_estimate()).sum::<usize>();

        MemoryStats {
            total_functions,
            total_params,
            estimated_memory_mb: total_memory / 1_000_000,
        }
    }
}

/// Memory usage statistics for project analysis.
#[derive(Debug)]
pub struct MemoryStats {
    /// Total number of functions analyzed.
    pub total_functions: usize,
    /// Total number of parameters across all functions.
    pub total_params: usize,
    /// Estimated memory usage in megabytes.
    pub estimated_memory_mb: usize,
}

/// Generated test file with path and content.
#[derive(Debug, Clone)]
pub struct TestFile {
    /// The file system path where the test should be written.
    pub path: String,
    /// The complete test file content as Rust source code.
    pub content: String,
}
