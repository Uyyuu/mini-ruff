use crate::diagnostic::Diagnostic;
use crate::lint::rule::{LintContext, Rule};

/// Rule to detect TODO comments in source code
///
/// This rule detects uppercase "TODO" in comments to prevent
/// unfinished work from remaining in production code.
///
/// Rule ID: MR001
pub struct TodoRule;

impl Rule for TodoRule {
    fn id(&self) -> &'static str {
        "MR001"
    }

    fn check(&self, ctx: &LintContext) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();

        // Scan source code line by line
        for (line_num, line) in ctx.source.lines().enumerate() {
            // Look for "TODO" in the line
            if let Some(col) = line.find("TODO") {
                // Line numbers are 1-indexed for user output
                // Column numbers are 1-indexed for user output
                diagnostics.push(Diagnostic::new(
                    self.id(),
                    "TODO comment found",
                    line_num + 1, // Convert to 1-indexed
                    col + 1,      // Convert to 1-indexed
                ));
            }
        }

        diagnostics
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_todo() {
        let source = r#"print("hello")
x = 1
"#;
        let ctx = LintContext::new(source);
        let rule = TodoRule;
        let diagnostics = rule.check(&ctx);

        assert_eq!(diagnostics.len(), 0);
    }

    #[test]
    fn test_single_todo() {
        let source = r#"# TODO fix this
print("hello")
"#;
        let ctx = LintContext::new(source);
        let rule = TodoRule;
        let diagnostics = rule.check(&ctx);

        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].rule_id, "MR001");
        assert_eq!(diagnostics[0].message, "TODO comment found");
        assert_eq!(diagnostics[0].line, 1);
        assert_eq!(diagnostics[0].column, 3); // Position of 'T' in "# TODO"
    }

    #[test]
    fn test_multiple_todos() {
        let source = r#"# TODO a
x = 1
# TODO b
"#;
        let ctx = LintContext::new(source);
        let rule = TodoRule;
        let diagnostics = rule.check(&ctx);

        assert_eq!(diagnostics.len(), 2);
        assert_eq!(diagnostics[0].line, 1);
        assert_eq!(diagnostics[1].line, 3);
    }

    #[test]
    fn test_lowercase_todo_not_detected() {
        // v0.1 only detects uppercase TODO
        let source = r#"# todo fix this
"#;
        let ctx = LintContext::new(source);
        let rule = TodoRule;
        let diagnostics = rule.check(&ctx);

        assert_eq!(diagnostics.len(), 0);
    }

    #[test]
    fn test_todo_with_colon() {
        let source = r#"# TODO: implement this feature
"#;
        let ctx = LintContext::new(source);
        let rule = TodoRule;
        let diagnostics = rule.check(&ctx);

        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].line, 1);
    }
}
