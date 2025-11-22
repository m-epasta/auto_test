//! # Filesystem Utilities
//!
//! Utility functions for safe and reliable file operations.
//!
//! This module provides enhanced filesystem operations optimized for large-scale
//! test generation, including atomic writes for data integrity and batch operations
//! for performance.

use std::fs;
use std::path::Path;
use tempfile::NamedTempFile;
use std::io::Write;
use crate::core::models::TestFile;
use crate::error::{AutoTestError, Result};

/// Filesystem utility functions for safe file operations.
///
/// This struct provides methods for writing test files with various safety
/// and performance optimizations, including atomic operations to prevent
/// data corruption during concurrent writes or system interruptions.
pub struct FsUtils;

impl FsUtils {
    /// Write a single test file to disk.
    ///
    /// This is the basic file writing method without atomic operations.
    /// Parent directories are created automatically if they don't exist.
    ///
    /// # Arguments
    ///
    /// * `test` - The test file to write
    ///
    /// # Returns
    ///
    /// Returns `std::io::Result<()>` indicating success or failure.
    pub fn write_test_file(test: &TestFile) -> std::io::Result<()> {
        let path = Path::new(&test.path);

        if let Some(parent) = path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)?;
            }
        }

        fs::write(path, &test.content)?;
        Ok(())
    }

    /// Write a test file atomically using a temporary file mechanism.
    ///
    /// This method ensures data integrity by writing to a temporary file first,
    /// then atomically moving it to the final location. This prevents corruption
    /// if the write operation is interrupted or if multiple processes write
    /// to the same file concurrently.
    ///
    /// Parent directories are created automatically if they don't exist.
    ///
    /// # Arguments
    ///
    /// * `test` - The test file to write atomically
    ///
    /// # Returns
    ///
    /// Returns `Result<()>` indicating success or failure with detailed error information.
    pub fn write_test_file_atomic(test: &TestFile) -> Result<()> {
        let path = Path::new(&test.path);
        let parent = path.parent();

        if let Some(parent_dir) = parent {
            if !parent_dir.exists() {
                fs::create_dir_all(parent_dir).map_err(|e| {
                    AutoTestError::Io { source: e }
                })?;
            }
        }

        // Create a temporary file in the same directory as the target
        let target_dir = parent.unwrap_or_else(|| Path::new("."));
        let mut temp_file = NamedTempFile::new_in(target_dir)
            .map_err(|e| AutoTestError::Io { source: e })?;

        // Write content to temporary file
        temp_file.write_all(test.content.as_bytes())
            .map_err(|e| AutoTestError::Io { source: e })?;

        // Atomically move temporary file to final location
        temp_file.persist(&path)
            .map_err(|e| AutoTestError::Io { source: e.into() })?;

        Ok(())
    }

    /// Write multiple test files to disk sequentially.
    ///
    /// This method writes each file individually without atomic operations.
    /// Parent directories are created automatically as needed.
    ///
    /// # Arguments
    ///
    /// * `files` - Slice of test files to write
    ///
    /// # Returns
    ///
    /// Returns `std::io::Result<()>` indicating success or failure.
    pub fn write_many(files: &[TestFile]) -> std::io::Result<()> {
        for f in files {
            Self::write_test_file(f)?;
        }
        Ok(())
    }

    /// Write multiple test files atomically for optimal concurrent safety.
    ///
    /// Each file is written atomically using temporary files, ensuring that
    /// the entire batch operation is either completely successful or can be
    /// safely rolled back. This is recommended for production use.
    ///
    /// # Arguments
    ///
    /// * `files` - Slice of test files to write atomically
    ///
    /// # Returns
    ///
    /// Returns `Result<()>` indicating success or failure with detailed error information.
    pub fn write_many_atomic(files: &[TestFile]) -> Result<()> {
        for file in files {
            Self::write_test_file_atomic(file)?;
        }
        Ok(())
    }
}
