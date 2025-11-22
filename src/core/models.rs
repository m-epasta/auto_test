//! # Data Models
//!
//! Core data structures representing analyzed Rust functions and projects.
//!
//! These models are used throughout the analysis and generation pipeline to
//! represent the structure of Rust functions, their parameters, and project-wide
//! collections of functions.

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParamInfo {
    pub name: String,
    pub typ: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionInfo {
    pub name: String,
    pub params: Vec<ParamInfo>,
    pub returns: String,
    pub file: String,
    pub is_async: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectInfo {
    pub language: String,
    pub root: String,
    pub functions: Vec<FunctionInfo>,
}

#[derive(Debug, Clone)]
pub struct TestFile {
    pub path: String,
    pub content: String,
}
