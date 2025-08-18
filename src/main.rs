use std::{
    fs::File,
    io::{self, Read, Write},
    path::PathBuf,
};

use clap::Parser;
use log::error;

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
        Some(file_path) => run_file(file_path),
        None => run_repl(),
    }
}

fn run_file(path: PathBuf) {
    let display = path.display();

    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(e) => {
            panic!("Couldn't open {}: {}", display, e)
        }
    };

    let mut content = String::new();
    match file.read_to_string(&mut content) {
        Ok(_) => {}
        Err(e) => panic!("Couldn't read {}: {}", display, e),
    }

    run(content)
}

fn run_repl() {
    println!("Welcome to rotten v{}", env!("CARGO_PKG_VERSION"));

    loop {
        print!("> ");
        io::stdout().flush().expect("Couldn't flush stdout");

        let mut line = String::new();
        match io::stdin().read_line(&mut line) {
            Ok(_) => {}
            Err(e) => {
                error!("Couldn't read input: {}", e);
                continue;
            }
        }

        if line.trim() == ".exit" {
            break;
        }

        run(line);
    }
}

fn run(source: String) {
    let tokens: Vec<&str> = source.split_whitespace().collect();
    for token in tokens {
        println!("{}", token);
    }
}
