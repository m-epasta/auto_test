use syn::{File, Item, FnArg, Pat, Type};
use quote::ToTokens;
use walkdir::WalkDir;
use crate::core::models::{FunctionInfo, ParamInfo, ProjectInfo};

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
                        // extract type as token string
                        let typ = match &*pat_type.ty {
                            Type::Reference(r) => {
                                // keep the & prefix for reference types
                                format!("&{}", r.elem.to_token_stream().to_string())
                            }
                            other => other.to_token_stream().to_string(),
                        };
                        params.push(ParamInfo { name, typ });
                    }
                }
            }

            // return type
            let returns = match &func.sig.output {
                syn::ReturnType::Default => "()".to_string(),
                syn::ReturnType::Type(_, ty) => ty.to_token_stream().to_string(),
            };

            functions.push(FunctionInfo {
                name: func.sig.ident.to_string(),
                params,
                returns,
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
