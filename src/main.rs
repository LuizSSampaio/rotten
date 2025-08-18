use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, about = "A rotten language trash interpreter", long_about = None)]
struct Args {
    /// Path to the .rot file to execute.
    /// When omitted the REPL will be started.
    script: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();
    match args.script {
        Some(file_path) => todo!(),
        None => todo!(),
    }
}
