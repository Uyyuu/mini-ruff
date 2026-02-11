use clap::Parser;
use std::path::PathBuf;

/// mini-ruff: A minimal Python linter
#[derive(Parser, Debug)]
#[command(name = "mini-ruff")]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Python file to lint
    pub file: PathBuf,
}

impl Cli {
    /// Parse command line arguments
    pub fn parse_args() -> Self {
        Self::parse()
    }
}
