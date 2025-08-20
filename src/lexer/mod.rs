use crate::lexer::scanner::Scanner;

mod emitter;
mod error;
mod keywords;
mod reader;
mod scanner;
pub mod token;

pub fn run(source: String) -> anyhow::Result<()> {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens()?;
    for token in tokens {
        println!("{:?}", token);
    }

    Ok(())
}
