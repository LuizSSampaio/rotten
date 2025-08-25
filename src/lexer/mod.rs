use crate::{lexer::scanner::Scanner, token::Token};

mod emitter;
mod error;
mod keywords;
mod reader;
mod scanner;

pub fn run(source: String) -> anyhow::Result<Vec<Token>> {
    let mut scanner = Scanner::new(source);
    scanner.scan_tokens()
}
