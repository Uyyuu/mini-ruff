use tree_sitter::Node;

use crate::diagnostic::Diagnostic;
use crate::lint::rule::{LintContext, Rule};

/// Rule to detect print() function calls in source code
///
/// This rule detects direct print() calls to prevent
/// debug print statements from remaining in production code.
///
/// Rule ID: MR002
pub struct PrintCallRule;

impl Rule for PrintCallRule {
    fn id(&self) -> &'static str {
        "MR002"
    }

    fn check(&self, ctx: &LintContext) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();
        let root = ctx.tree.root_node();
        find_print_calls(root, ctx.source, self.id(), &mut diagnostics);
        diagnostics
    }
}

/// Recursively walks the AST to find print() calls
fn find_print_calls(node: Node, source: &str, rule_id: &str, diagnostics: &mut Vec<Diagnostic>) {
    if node.kind() == "call" {
        // Get the function part of the call expression
        if let Some(func) = node.child_by_field_name("function") {
            // Only match direct identifier "print", not attribute access like obj.print()
            if func.kind() == "identifier" {
                let name = &source[func.byte_range()];
                if name == "print" {
                    let start = func.start_position();
                    diagnostics.push(Diagnostic::new(
                        rule_id,
                        "print() usage is not allowed",
                        start.row + 1,    // Convert to 1-indexed
                        start.column + 1, // Convert to 1-indexed
                    ));
                }
            }
        }
    }

    // Recurse into child nodes
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        find_print_calls(child, source, rule_id, diagnostics);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_print() {
        let source = "x = 1 + 2\n";
        let ctx = LintContext::new(source);
        let rule = PrintCallRule;
        let diagnostics = rule.check(&ctx);

        assert_eq!(diagnostics.len(), 0);
    }

    #[test]
    fn test_print_with_string() {
        let source = "print(\"hello\")\n";
        let ctx = LintContext::new(source);
        let rule = PrintCallRule;
        let diagnostics = rule.check(&ctx);

        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].rule_id, "MR002");
        assert_eq!(diagnostics[0].message, "print() usage is not allowed");
        assert_eq!(diagnostics[0].line, 1);
        assert_eq!(diagnostics[0].column, 1);
    }

    #[test]
    fn test_print_no_args() {
        let source = "print()\n";
        let ctx = LintContext::new(source);
        let rule = PrintCallRule;
        let diagnostics = rule.check(&ctx);

        assert_eq!(diagnostics.len(), 1);
    }

    #[test]
    fn test_print_multiple_args() {
        let source = "print(x, y, z)\n";
        let ctx = LintContext::new(source);
        let rule = PrintCallRule;
        let diagnostics = rule.check(&ctx);

        assert_eq!(diagnostics.len(), 1);
    }

    #[test]
    fn test_method_call_not_detected() {
        let source = "logger.print(\"msg\")\n";
        let ctx = LintContext::new(source);
        let rule = PrintCallRule;
        let diagnostics = rule.check(&ctx);

        assert_eq!(diagnostics.len(), 0);
    }

    #[test]
    fn test_comment_not_detected() {
        let source = "# print(\"debug\")\n";
        let ctx = LintContext::new(source);
        let rule = PrintCallRule;
        let diagnostics = rule.check(&ctx);

        assert_eq!(diagnostics.len(), 0);
    }

    #[test]
    fn test_multiple_prints() {
        let source = "print(\"a\")\nprint(\"b\")\n";
        let ctx = LintContext::new(source);
        let rule = PrintCallRule;
        let diagnostics = rule.check(&ctx);

        assert_eq!(diagnostics.len(), 2);
        assert_eq!(diagnostics[0].line, 1);
        assert_eq!(diagnostics[1].line, 2);
    }
}
