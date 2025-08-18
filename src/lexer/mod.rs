use crate::lexer::scanner::Scanner;

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
