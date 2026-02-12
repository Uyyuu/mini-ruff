use crate::diagnostic::Diagnostic;
use tree_sitter::Tree;

/// Context provided to lint rules for checking
pub struct LintContext<'a> {
    /// Source code content
    pub source: &'a str,
    /// tree-sitter AST
    pub tree: Tree,
}

impl<'a> LintContext<'a> {
    /// Creates a new lint context by parsing source code into AST
    pub fn new(source: &'a str) -> Self {
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&tree_sitter_python::LANGUAGE.into())
            .expect("Failed to set Python language");
        let tree = parser.parse(source, None).expect("Failed to parse source");

        Self { source, tree }
    }
}

/// Trait that all lint rules must implement
pub trait Rule {
    /// Returns the unique rule identifier (e.g., "MR001")
    fn id(&self) -> &'static str;

    /// Checks the source code and returns any violations found
    fn check(&self, ctx: &LintContext) -> Vec<Diagnostic>;
}
