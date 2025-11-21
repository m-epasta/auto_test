use std::path;

use syn::{File, Item};
use quote::ToTokens;
use walkdir::WalkDir;
use crate::core::models::{FunctionInfo, ProjectInfo};

pub fn analyze_rust_files(file_path: &str) -> Vec<FunctionInfo> {
    let content = std::fs::read_to_string(file_path)
        .expect("Cannot read file");

    let ast: File = syn::parse_file(&content)
        .expect(&format!("Failed to parse rust file: {}", file_path));

    let mut functions = Vec::new();

    for item in ast.items {
        if let Item::Fn(func) = item {
            if func.vis.to_token_stream().to_string() == "pub" {
                let params: Vec<String> = func.sig.inputs.iter()
                    .map(|arg| quote::quote!(#arg).to_string())
                    .collect(); 

                let ret_type = match &func.sig.output {
                    syn::ReturnType::Default => "()".to_string(),
                    syn::ReturnType::Type(_, ty) => quote::quote!(#ty).to_string(),
                };

                functions.push(FunctionInfo {
                    name: func.sig.ident.to_string(),
                    params: Some(params),
                    returns: ret_type,
                    file: file_path.to_string(),
                });
            }
        }
    }

    functions
}


pub fn analyze_rust_project(root: &str) -> ProjectInfo {
    let mut all_functions = Vec::new();

    for entry in WalkDir::new(root)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map(|ext| ext == "rs").unwrap_or(false))
    {
        let path_str = entry.path().to_string_lossy();
        let mut funcs = analyze_rust_files(&path_str);
        all_functions.append(&mut funcs);
    }

    ProjectInfo { 
        language: "rust".into(),
        root: root.into(),
        functions: all_functions,
    }
}