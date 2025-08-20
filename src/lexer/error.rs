use std::error::Error;
use std::fmt::Display;

use crate::lexer::token::TokenPosition;

#[derive(Debug, Clone)]
pub enum LexerErrorMessage {
    UnexpectedCharacter,
    UnterminatedString,
    NumberParseError,
}

#[derive(Debug, Clone)]
pub struct LexerError {
    pub message: LexerErrorMessage,
    pub lexeme: String,
    pub position: TokenPosition,
}

impl LexerError {
    fn message_to_string(&self) -> &str {
        match self.message {
            LexerErrorMessage::UnexpectedCharacter => "Unexpected character.",
            LexerErrorMessage::UnterminatedString => "Unterminated string.",
            LexerErrorMessage::NumberParseError => "Failed to parse number.",
        }
    }
}

impl Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}:{}] Error: {}\n{}",
            self.position.row,
            self.position.column,
            self.message_to_string(),
            self.lexeme
        )
    }
}

impl Error for LexerError {}
