use syn::{File, Item, FnArg, Pat, Type};
use quote::ToTokens;
use walkdir::WalkDir;
use glob::Pattern;
use ignore::WalkBuilder;
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use crate::config::Config;
use crate::core::models::{FunctionInfo, ParamInfo, ProjectInfo, TypeIntern};
use crate::error::Result;

/// Analyze a single Rust file and return public functions with parameters & return types.
pub fn analyze_rust_file(file_path: &str) -> Vec<FunctionInfo> {
    let content = std::fs::read_to_string(file_path)
        .unwrap_or_else(|_| panic!("Cannot read file: {}", file_path));

    let ast: File = syn::parse_file(&content)
        .unwrap_or_else(|_| panic!("Failed to parse rust file: {}", file_path));

    let mut functions = Vec::new();

    for item in ast.items {
        if let Item::Fn(func) = item {
            // keep only pub functions
            if func.vis.to_token_stream().to_string() != "pub" {
                continue;
            }

            // params: collect name and type
            let mut params: Vec<ParamInfo> = Vec::new();
            for input in func.sig.inputs.iter() {
                match input {
                    FnArg::Receiver(_) => {
                        // method receiver, we skip or record as "self"
                        params.push(ParamInfo { name: "self".into(), typ: "Self".into() });
                    }
                    FnArg::Typed(pat_type) => {
                        // extract param name if available
                        let name = match &*pat_type.pat {
                            Pat::Ident(ident) => ident.ident.to_string(),
                            _ => "_".to_string(),
                        };
                        // extract type as token string with interning
                        let typ_str = match &*pat_type.ty {
                            Type::Reference(r) => {
                                // keep the & prefix for reference types
                                format!("&{}", r.elem.to_token_stream())
                            }
                            other => other.to_token_stream().to_string(),
                        };
                        params.push(ParamInfo { name, typ: TypeIntern::new(&typ_str) });
                    }
                }
            }

            // return type with interning
            let returns_str = match &func.sig.output {
                syn::ReturnType::Default => "()".to_string(),
                syn::ReturnType::Type(_, ty) => ty.to_token_stream().to_string(),
            };

            functions.push(FunctionInfo {
                name: func.sig.ident.to_string(),
                params,
                returns: TypeIntern::new(&returns_str),
                file: file_path.to_string(),
                is_async: func.sig.asyncness.is_some(),
            });
        }
    }

    functions
}

/// Walk project root and analyze all `.rs` files to build a ProjectInfo
pub fn analyze_rust_project(root: &str) -> ProjectInfo {
    let mut all_functions = Vec::new();

    for entry in WalkDir::new(root).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("rs") {
            let path_str = path.to_string_lossy().to_string();
            let mut funcs = analyze_rust_file(&path_str);
            all_functions.append(&mut funcs);
        }
    }

    ProjectInfo {
        language: "rust".into(),
        root: root.into(),
        functions: all_functions,
    }
}

/// Check if a file path should be skipped based on configuration
pub fn should_skip_file(file_path: &Path, config: &Config) -> bool {
    let path_str = file_path.to_string_lossy();

    // Skip standard ignored paths
    if is_standard_ignored_path(file_path) {
        return true;
    }

    // Skip configured patterns
    for skip_pattern in &config.skip_patterns {
        if let Ok(pattern) = Pattern::new(skip_pattern) {
            if pattern.matches(&path_str) {
                return true;
            }
        }
    }

    false
}

/// Check if a path is in standard ignored locations
pub fn is_standard_ignored_path(path: &Path) -> bool {
    let path_str = path.to_string_lossy();
    path_str.contains("/target/") ||
    path_str.contains("/.git/") ||
    path_str.contains("/node_modules/") ||
    path.starts_with(".") ||
    path_str.contains("/build/") ||
    path_str.contains("/dist/")
}

/// Walk project root with filtering and analyze files respecting config
pub fn analyze_rust_project_filtered(project_root: &Path, config: &Config) -> Result<ProjectInfo> {
    let mut all_functions = Vec::new();
    let mut processed_files = HashSet::new();

    let walker: Vec<PathBuf> = if config.respect_gitignore {
        // Use ignore crate to respect .gitignore
        WalkBuilder::new(project_root)
            .hidden(false) // Don't skip hidden files by default
            .git_ignore(true)
            .git_global(true)
            .build()
            .filter_map(|e| e.ok())
            .map(|e| e.path().to_path_buf())
            .collect()
    } else {
        // Use walkdir without gitignore
        WalkDir::new(project_root)
            .into_iter()
            .filter_map(|e| e.ok())
            .map(|e| e.path().to_path_buf())
            .collect()
    };

    for entry in walker {
        let path = &entry;

        // Skip directories
        if path.is_dir() {
            continue;
        }

        // Skip non-Rust files
        if path.extension().and_then(|s| s.to_str()) != Some("rs") {
            continue;
        }

        // Skip based on config
        if should_skip_file(path, config) {
            continue;
        }

        // Avoid processing the same file multiple times
        let path_str = path.to_string_lossy().to_string();
        if processed_files.contains(&path_str) {
            continue;
        }
        processed_files.insert(path_str.clone());

        // Analyze the file
        match std::fs::read_to_string(&path) {
            Ok(content) => {
                match syn::parse_file(&content) {
                    Ok(ast) => {
                        let functions = extract_functions_from_ast(&ast, &path_str, config);
                        all_functions.extend(functions);
                    }
                    Err(e) => {
                        eprintln!("Warning: Failed to parse {}: {}", path_str, e);
                        // Continue processing other files
                    }
                }
            }
            Err(e) => {
                eprintln!("Warning: Could not read {}: {}", path_str, e);
                // Continue processing other files
            }
        }
    }

    Ok(ProjectInfo {
        language: "rust".into(),
        root: project_root.to_string_lossy().to_string(),
        functions: all_functions,
    })
}

/// Extract functions from AST with configuration filtering
fn extract_functions_from_ast(ast: &File, file_path: &str, config: &Config) -> Vec<FunctionInfo> {
    let mut functions = Vec::new();

    for item in &ast.items {
        if let Item::Fn(func) = item {
            // Check visibility based on config
            let is_public = func.vis.to_token_stream().to_string() == "pub";
            if !is_public && !config.include_private {
                continue;
            }

            // Skip functions based on config
            let func_name = func.sig.ident.to_string();
            if config.should_skip_function(&func_name) {
                continue;
            }

            // Extract parameters
            let mut params: Vec<ParamInfo> = Vec::new();
            for input in func.sig.inputs.iter() {
                match input {
                    FnArg::Receiver(_) => {
                        params.push(ParamInfo {
                            name: "self".into(),
                            typ: "Self".into(),
                        });
                    }
                    FnArg::Typed(pat_type) => {
                        let name = match &*pat_type.pat {
                            Pat::Ident(ident) => ident.ident.to_string(),
                            _ => "_".to_string(),
                        };

                        let typ_str = match &*pat_type.ty {
                            Type::Reference(r) => {
                                format!("&{}", r.elem.to_token_stream())
                            }
                            other => other.to_token_stream().to_string(),
                        };

                        params.push(ParamInfo { name, typ: TypeIntern::new(&typ_str) });
                    }
                }
            }

            // Extract return type with interning
            let returns_str = match &func.sig.output {
                syn::ReturnType::Default => "()".to_string(),
                syn::ReturnType::Type(_, ty) => ty.to_token_stream().to_string(),
            };

            functions.push(FunctionInfo {
                name: func_name,
                params,
                returns: TypeIntern::new(&returns_str),
                file: file_path.to_string(),
                is_async: func.sig.asyncness.is_some(),
            });
        }
    }

    functions
}
