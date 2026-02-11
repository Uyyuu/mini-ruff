use crate::diagnostic::Diagnostic;
use crate::lint::LintEngine;
use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

/// Exit codes for the CLI
pub mod exit_code {
    /// No issues found
    pub const SUCCESS: i32 = 0;
    /// Lint errors found
    pub const LINT_ERROR: i32 = 1;
    /// Execution error (file not found, parse error, etc.)
    pub const EXECUTION_ERROR: i32 = 2;
}

/// Runner orchestrates the linting flow
pub struct Runner {
    engine: LintEngine,
}

impl Runner {
    /// Creates a new runner with default lint engine
    pub fn new() -> Self {
        Self {
            engine: LintEngine::new(),
        }
    }

    /// Runs the linter on the specified file
    ///
    /// Returns the exit code based on the result:
    /// - 0: No issues found
    /// - 1: Lint errors found
    /// - 2: Execution error
    pub fn run(&self, file_path: &Path) -> i32 {
        match self.run_internal(file_path) {
            Ok(diagnostics) => {
                if diagnostics.is_empty() {
                    exit_code::SUCCESS
                } else {
                    // Print diagnostics
                    for diagnostic in &diagnostics {
                        println!(
                            "{}",
                            diagnostic.format(file_path.to_str().unwrap_or("unknown"))
                        );
                    }
                    exit_code::LINT_ERROR
                }
            }
            Err(e) => {
                eprintln!("Error: {:#}", e);
                exit_code::EXECUTION_ERROR
            }
        }
    }

    /// Internal run implementation that returns Result
    fn run_internal(&self, file_path: &Path) -> Result<Vec<Diagnostic>> {
        // Read file
        let source = fs::read_to_string(file_path)
            .with_context(|| format!("Failed to read file: {}", file_path.display()))?;

        // Run lint engine
        let diagnostics = self.engine.check(&source);

        Ok(diagnostics)
    }
}

impl Default for Runner {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_run_no_issues() -> Result<()> {
        let mut file = NamedTempFile::new()?;
        writeln!(file, "print('hello')")?;

        let runner = Runner::new();
        let exit_code = runner.run(file.path());

        assert_eq!(exit_code, exit_code::SUCCESS);
        Ok(())
    }

    #[test]
    fn test_run_with_todo() -> Result<()> {
        let mut file = NamedTempFile::new()?;
        writeln!(file, "# TODO fix this")?;

        let runner = Runner::new();
        let exit_code = runner.run(file.path());

        assert_eq!(exit_code, exit_code::LINT_ERROR);
        Ok(())
    }
}
