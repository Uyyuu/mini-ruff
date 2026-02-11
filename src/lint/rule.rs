use crate::diagnostic::Diagnostic;

/// Context provided to lint rules for checking
pub struct LintContext<'a> {
    /// Source code content
    pub source: &'a str,
}

impl<'a> LintContext<'a> {
    /// Creates a new lint context
    pub fn new(source: &'a str) -> Self {
        Self { source }
    }
}

/// Trait that all lint rules must implement
pub trait Rule {
    /// Returns the unique rule identifier (e.g., "MR001")
    fn id(&self) -> &'static str;

    /// Checks the source code and returns any violations found
    fn check(&self, ctx: &LintContext) -> Vec<Diagnostic>;
}
