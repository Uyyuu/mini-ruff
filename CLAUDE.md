# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

mini-ruff is a Rust-based Python linter designed for learning purposes. It focuses on AST analysis, static analysis architecture, Rust trait design, and CLI tool development.

## Development Commands

### Building and Running
```bash
cargo build              # Build the project
cargo run -- <file.py>   # Run the linter on a Python file
cargo build --release    # Build optimized release binary
```

### Testing
```bash
cargo test               # Run all tests
cargo test <test_name>   # Run a specific test
cargo test -- --nocapture # Run tests with stdout/stderr output
```

### Code Quality
```bash
cargo clippy             # Run linter
cargo fmt                # Format code
cargo check              # Fast compile check without producing binary
```

## Architecture

### Layered Design
The architecture follows a clean separation of concerns:

```
CLI → Runner → Parser → Rule Engine → Diagnostics
```

### Core Module Structure
```
src/
├ main.rs              # Entry point
├ cli.rs               # CLI argument parsing (clap)
├ runner.rs            # Orchestrates linting flow
├ parser/
│ └ python.rs          # tree-sitter-python integration (source → AST)
├ lint/
│ ├ mod.rs             # Lint module coordinator
│ ├ rule.rs            # Rule trait definition
│ ├ rules/
│ │ ├ todo.rs          # MR001: TODO comment detection
│ │ ├ print_call.rs    # MR002: print() call detection
│ │ └ line_length.rs   # MR003: Line length limit (default 120 chars)
└ diagnostic.rs        # Diagnostic model and output formatting
```

### Core Traits

**Rule Trait**: All lint rules implement this trait
```rust
pub trait Rule {
    fn id(&self) -> &'static str;
    fn check(&self, ctx: &LintContext) -> Vec<Diagnostic>;
}
```

**LintContext**: Provides rules access to source code and AST
```rust
pub struct LintContext<'a> {
    pub source: &'a str,
    pub tree: &'a Tree,  // tree-sitter AST
}
```

**Diagnostic**: Represents a lint violation
```rust
pub struct Diagnostic {
    pub rule_id: String,
    pub message: String,
    pub line: usize,
    pub column: usize,
}
```

### Rule Engine
- Rules are stored as `Vec<Box<dyn Rule>>` and executed sequentially
- Each rule receives `LintContext` and returns `Vec<Diagnostic>`
- New rules should be added to `src/lint/rules/` and registered in the rule engine

## Supported Rules (v0.1)

- **MR001**: TODO comments in code (prevents TODO comments from remaining in production code)
- **MR002**: print() function calls (prevents debug print statements in production)
- **MR003**: Line length exceeds 120 characters

## Output Format

Diagnostics follow this format:
```
<file>:<line>:<col> <rule_id> <message>
```

Example:
```
example.py:10:5 MR001 TODO comment found
example.py:22:1 MR002 print() usage is not allowed
```

## Exit Codes

- **0**: No issues found
- **1**: Lint errors found
- **2**: Execution error (file not found, parse error, etc.)

## Error Handling Strategy

- `anyhow`: Application-level errors (file I/O, CLI errors)
- `thiserror`: Domain-level errors (custom error types for lint logic)

## Scope and Limitations (v0.1)

**Supported**:
- Single Python file analysis

**Not Supported** (future expansion):
- Project-wide analysis
- Type inference
- Autofix
- LSP mode
- Configuration files
- Directory scanning
- Parallel linting

## Adding New Rules

1. Create new rule file in `src/lint/rules/`
2. Implement the `Rule` trait
3. Define rule ID (format: MR###)
4. Implement `check()` method using tree-sitter queries
5. Register rule in the rule engine
6. Add tests for the rule

## Dependencies

- **clap**: CLI argument parsing
- **anyhow**: Error handling
- **thiserror**: Custom error types
- **tree-sitter-python**: Python AST parsing
