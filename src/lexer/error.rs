use std::error::Error;
use std::fmt::Display;

use crate::lexer::token::TokenPosition;

#[derive(Debug, Clone)]
pub struct LexerError {
    pub message: String,
    pub lexeme: String,
    pub position: TokenPosition,
}

impl Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}:{}] Error: {}\n{}",
            self.position.row, self.position.column, self.message, self.lexeme
        )
    }
}

impl Error for LexerError {}
