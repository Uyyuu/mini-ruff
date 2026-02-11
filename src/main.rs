mod cli;
mod diagnostic;
mod lint;
mod runner;

use cli::Cli;
use runner::Runner;

fn main() {
    let args = Cli::parse_args();
    let runner = Runner::new();
    let exit_code = runner.run(&args.file);
    std::process::exit(exit_code);
}
