pub struct FunctionInfo {
    pub name: String,
    pub params: Option<Vec<String>>,
    pub returns: String,
    pub file: String
}

pub struct ProjectInfo {
    pub language: String,
    pub root: String,
    pub functions: Vec<FunctionInfo>,
}

pub struct TestFile {
    pub path: String,
    pub content: String
}