mini-ruff Design Spec v0.1

1. Goal

Rust製のPython向けミニLinterを実装する。

学習目的：

AST解析

静的解析アーキテクチャ

Rust trait設計

CLIツール設計

2. Scope (v0.1)
   対象

単一Pythonファイル

非対象

Project全体解析

型推論

autofix

LSP

3. Supported Rules (v0.1)
   Rule 1: TODO禁止

# TODO:

検出対象：

コメント内TODO

Rule 2: print() 禁止
print("debug")

理由：

本番コードに残る事故防止

Rule 3: 行長制限

Default: 120 chars

4. CLI Spec
   Command
   mini-ruff <file.py>

Exit Code
Code Meaning
0 問題なし
1 Lint errorあり
2 実行エラー 5. Output Format
<file>:<line>:<col> <rule_id> <message>

例：

example.py:10:5 MR001 TODO comment found
example.py:22:1 MR002 print() usage is not allowed

6. Architecture
   Layer構造
   CLI
   ↓
   Runner
   ↓
   Parser
   ↓
   Rule Engine
   ↓
   Diagnostics

7. Module Structure
   src/
   ├ main.rs
   ├ cli.rs
   ├ runner.rs
   ├ parser/
   │ └ python.rs
   ├ lint/
   │ ├ mod.rs
   │ ├ rule.rs
   │ ├ rules/
   │ │ ├ todo.rs
   │ │ ├ print_call.rs
   │ │ └ line_length.rs
   └ diagnostic.rs

8. Core Traits
   Rule Trait
   pub trait Rule {
   fn id(&self) -> &'static str;
   fn check(&self, ctx: &LintContext) -> Vec<Diagnostic>;
   }

LintContext
pub struct LintContext<'a> {
pub source: &'a str,
pub tree: &'a Tree,
}

9. Diagnostic Model
   pub struct Diagnostic {
   pub rule_id: String,
   pub message: String,
   pub line: usize,
   pub column: usize,
   }

10. Parser

tree-sitter-python 使用。

責務：

source → AST

11. Rule Engine
    Vec<Box<dyn Rule>>

を順に実行。

12. Runner Flow
    read file
    ↓
    parse AST
    ↓
    run rules
    ↓
    collect diagnostics
    ↓
    print result
    ↓
    set exit code

13. Error Handling

anyhow → application error

thiserror → domain error

14. Logging

tracing使用。

15. Future Expansion (Not in v0.1)

autofix

config file

directory scan

parallel lint

LSP mode
