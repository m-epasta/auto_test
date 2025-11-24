use regex::Regex;

#[derive(Debug, Clone)]
pub struct FunctionInfo {
    pub name: String,
    pub args: Vec<String>,
    pub return_type: Option<String>,
    pub is_public: bool,
    pub receiver: Option<String>, // For methods: Some("MyStruct")
}

#[derive(Debug, Clone)]
pub struct StructInfo {
    pub name: String,
    pub fields: Vec<String>,
}

pub struct VParser;

impl VParser {
    /// Parse function signatures including standalone functions and methods
    pub fn parse_function_signatures(content: &str) -> Vec<FunctionInfo> {
        let mut functions = Vec::new();

        // Enhanced regex to capture:
        // - pub fn or fn
        // - Optional receiver: (r ReceiverType)
        // - Function name
        // - Args
        // - Return type (including [], ?, &, etc.)
        let fn_re = Regex::new(
            r"(?m)^(pub\s+)?fn\s+(?:\((\w+)\s+(\w+)\)\s+)?(\w+)\s*\((.*?)\)\s*([\w\[\]\?\&\s]*)",
        )
        .unwrap();

        for cap in fn_re.captures_iter(content) {
            let is_public = cap.get(1).is_some();
            let receiver = cap.get(3).map(|m| m.as_str().to_string());
            let name = cap[4].to_string();
            let args_str = &cap[5];
            let return_type_str = &cap[6];

            let args: Vec<String> = args_str
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();

            let return_type = if return_type_str.trim().is_empty() {
                None
            } else {
                Some(return_type_str.trim().to_string())
            };

            functions.push(FunctionInfo {
                name,
                args,
                return_type,
                is_public,
                receiver,
            });
        }

        functions
    }

    /// Parse struct definitions
    pub fn parse_structs(content: &str) -> Vec<StructInfo> {
        let mut structs = Vec::new();
        let struct_re = Regex::new(r"(?m)^(?:pub\s+)?struct\s+(\w+)\s*\{([^}]*)\}").unwrap();

        for cap in struct_re.captures_iter(content) {
            let name = cap[1].to_string();
            let fields_str = &cap[2];

            let fields: Vec<String> = fields_str
                .lines()
                .map(|l| l.trim().to_string())
                .filter(|l| !l.is_empty() && !l.starts_with("//"))
                .collect();

            structs.push(StructInfo { name, fields });
        }

        structs
    }

    /// Generate a test function with smart assertions based on return type
    pub fn generate_test(func: &FunctionInfo) -> String {
        let test_name = if let Some(ref receiver) = func.receiver {
            format!("test_{}_{}", receiver.to_lowercase(), func.name)
        } else {
            format!("test_{}", func.name)
        };

        let setup = Self::generate_test_setup(func);
        let call = Self::generate_function_call(func);
        let assertion = Self::generate_smart_assertion(&func.return_type);

        format!(
            "fn {}() {{\n{}\n    let result = {}\n{}\n}}\n",
            test_name, setup, call, assertion
        )
    }

    /// Generate setup code for test (e.g., struct initialization for methods)
    fn generate_test_setup(func: &FunctionInfo) -> String {
        if let Some(ref receiver) = func.receiver {
            format!(
                "    // TODO: Initialize {} instance\n    // let instance = {}{{ }}",
                receiver, receiver
            )
        } else {
            "    // TODO: Set up test data".to_string()
        }
    }

    /// Generate the function call
    fn generate_function_call(func: &FunctionInfo) -> String {
        let params: Vec<String> = func
            .args
            .iter()
            .enumerate()
            .map(|(i, arg)| {
                let type_hint = arg.split_whitespace().last().unwrap_or("unknown");
                Self::generate_param_value(type_hint, i)
            })
            .collect();

        if let Some(ref _receiver) = func.receiver {
            format!("instance.{}({})", func.name, params.join(", "))
        } else {
            format!("{}({})", func.name, params.join(", "))
        }
    }

    /// Generate a sample parameter value based on type
    fn generate_param_value(type_hint: &str, index: usize) -> String {
        match type_hint {
            "int" | "i8" | "i16" | "i32" | "i64" => (index + 1).to_string(),
            "u8" | "u16" | "u32" | "u64" => (index + 1).to_string(),
            "f32" | "f64" => format!("{}.0", index + 1),
            "bool" => "true".to_string(),
            "string" => format!("'{}'", "test"),
            t if t.starts_with("[]") => "[]".to_string(),
            t if t.starts_with("?") => "none".to_string(),
            _ => format!("/* TODO: {} */", type_hint),
        }
    }

    /// Generate smart assertions based on return type
    fn generate_smart_assertion(return_type: &Option<String>) -> String {
        match return_type {
            None => "    // Function returns nothing".to_string(),
            Some(t) => {
                let t = t.trim();
                if t.is_empty() {
                    return "    // Function returns nothing".to_string();
                }

                match t {
                    "int" | "i8" | "i16" | "i32" | "i64" | "u8" | "u16" | "u32" | "u64" => {
                        "    assert result == 0 // TODO: Replace with expected value".to_string()
                    }
                    "bool" => {
                        "    assert result == true // TODO: Verify expected boolean".to_string()
                    }
                    "string" => {
                        "    assert result.len > 0 // TODO: Verify string content".to_string()
                    }
                    "f32" | "f64" => {
                        "    assert result >= 0.0 // TODO: Verify float value".to_string()
                    }
                    t if t.starts_with("[]") => {
                        "    assert result.len >= 0 // TODO: Verify array content".to_string()
                    }
                    t if t.starts_with("?") => {
                        "    assert result != none // TODO: Verify optional value".to_string()
                    }
                    _ => format!("    assert true // TODO: Add assertion for type: {}", t),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_function() {
        let content = "fn add(a int, b int) int { return a + b }";
        let funcs = VParser::parse_function_signatures(content);
        assert_eq!(funcs.len(), 1);
        assert_eq!(funcs[0].name, "add");
        assert_eq!(funcs[0].args.len(), 2);
        assert_eq!(funcs[0].return_type, Some("int".to_string()));
        assert!(!funcs[0].is_public);
    }

    #[test]
    fn test_parse_public_function() {
        let content = "pub fn greet(name string) string { return 'Hello' }";
        let funcs = VParser::parse_function_signatures(content);
        assert_eq!(funcs.len(), 1);
        assert!(funcs[0].is_public);
        assert_eq!(funcs[0].name, "greet");
    }

    #[test]
    fn test_parse_method() {
        let content = "fn (u User) get_name() string { return u.name }";
        let funcs = VParser::parse_function_signatures(content);
        assert_eq!(funcs.len(), 1);
        assert_eq!(funcs[0].receiver, Some("User".to_string()));
        assert_eq!(funcs[0].name, "get_name");
    }

    #[test]
    fn test_generate_smart_assertions() {
        let func_int = FunctionInfo {
            name: "add".to_string(),
            args: vec![],
            return_type: Some("int".to_string()),
            is_public: false,
            receiver: None,
        };
        let test_code = VParser::generate_test(&func_int);
        assert!(test_code.contains("assert result == 0"));

        let func_bool = FunctionInfo {
            name: "is_valid".to_string(),
            args: vec![],
            return_type: Some("bool".to_string()),
            is_public: false,
            receiver: None,
        };
        let test_code_bool = VParser::generate_test(&func_bool);
        assert!(test_code_bool.contains("assert result == true"));
    }

    #[test]
    fn test_parse_struct() {
        let content = "struct User {\n    name string\n    age int\n}";
        let structs = VParser::parse_structs(content);
        assert_eq!(structs.len(), 1);
        assert_eq!(structs[0].name, "User");
        assert_eq!(structs[0].fields.len(), 2);
    }
}
