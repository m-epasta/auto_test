use regex::Regex;

#[derive(Debug, Clone)]
pub struct FunctionInfo {
    pub name: String,
    pub args: Vec<String>,
    pub return_type: Option<String>,
}

pub struct VParser;

impl VParser {
    pub fn parse_function_signatures(content: &str) -> Vec<FunctionInfo> {
        let mut functions = Vec::new();
        // Regex to capture function signatures: fn name(args) type {
        // This is a simplified regex and might need refinement for complex cases
        let re = Regex::new(r"fn\s+(\w+)\s*\((.*?)\)\s*([\w\s\[\]&]*)").unwrap();

        for cap in re.captures_iter(content) {
            let name = cap[1].to_string();
            let args_str = &cap[2];
            let return_type_str = &cap[3];

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
            });
        }

        functions
    }

    pub fn generate_test(func: &FunctionInfo) -> String {
        format!(
            "fn test_{}() {{\n    // TODO: Implement test for {}\n    assert true\n}}\n",
            func.name, func.name
        )
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
        assert_eq!(funcs[0].args, vec!["a int", "b int"]);
        assert_eq!(funcs[0].return_type, Some("int".to_string()));
    }

    #[test]
    fn test_generate_test() {
        let func = FunctionInfo {
            name: "add".to_string(),
            args: vec!["a int".to_string(), "b int".to_string()],
            return_type: Some("int".to_string()),
        };
        let test_code = VParser::generate_test(&func);
        assert!(test_code.contains("fn test_add()"));
        assert!(test_code.contains("assert true"));
    }
}
