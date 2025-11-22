#[cfg(test)]
mod tests {
    use auto_test::*;

    #[test] fn test_run_integration() {
        // Arrange
        // Create a temporary directory or use test fixtures

        // Act
        let result = auto_test::generate_tests_for_project("/tmp/test_project");

        // Assert
        // Verify that test generation succeeded
        assert!(result.is_ok());
    }
}
