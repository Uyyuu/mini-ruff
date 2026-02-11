/// Represents a lint violation found in the code
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    /// Rule ID (e.g., "MR001")
    pub rule_id: String,
    /// Human-readable error message
    pub message: String,
    /// Line number (1-indexed)
    pub line: usize,
    /// Column number (1-indexed)
    pub column: usize,
}

impl Diagnostic {
    /// Creates a new diagnostic
    pub fn new(
        rule_id: impl Into<String>,
        message: impl Into<String>,
        line: usize,
        column: usize,
    ) -> Self {
        Self {
            rule_id: rule_id.into(),
            message: message.into(),
            line,
            column,
        }
    }

    /// Formats diagnostic for CLI output
    /// Format: <file>:<line>:<column> <rule_id> <message>
    pub fn format(&self, file_path: &str) -> String {
        format!(
            "{}:{}:{} {} {}",
            file_path, self.line, self.column, self.rule_id, self.message
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diagnostic_format() {
        let diag = Diagnostic::new("MR001", "TODO comment found", 10, 5);
        assert_eq!(
            diag.format("example.py"),
            "example.py:10:5 MR001 TODO comment found"
        );
    }
}
