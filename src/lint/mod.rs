pub mod rule;
pub mod rules;

use crate::diagnostic::Diagnostic;
use rule::{LintContext, Rule};

/// Lint engine that runs all registered rules
pub struct LintEngine {
    rules: Vec<Box<dyn Rule>>,
}

impl LintEngine {
    /// Creates a new lint engine with default rules
    pub fn new() -> Self {
        let rules: Vec<Box<dyn Rule>> = vec![Box::new(rules::todo::TodoRule)];

        Self { rules }
    }

    /// Runs all rules against the given source code
    pub fn check(&self, source: &str) -> Vec<Diagnostic> {
        let ctx = LintContext::new(source);
        let mut diagnostics = Vec::new();

        for rule in &self.rules {
            diagnostics.extend(rule.check(&ctx));
        }

        diagnostics
    }
}

impl Default for LintEngine {
    fn default() -> Self {
        Self::new()
    }
}
