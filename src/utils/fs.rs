use std::fs;
use std::path::Path;
use crate::core::models::TestFile;

pub struct FsUtils;

impl FsUtils {
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

    pub fn write_many(files: &[TestFile]) -> std::io::Result<()> {
        for f in files {
            Self::write_test_file(f)?;
        }
        Ok(())
    }
}
