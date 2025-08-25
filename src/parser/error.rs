use std::error::Error;
use std::fmt::Display;

use crate::token::Token;

#[derive(Debug, Clone)]
pub enum ParserErrorMessage {
    GetTokenError,
    LiteralTokenWithoutValue,
    UnexpectedTokenType,
    ExpectRightParenthesis,
}

#[derive(Debug, Clone)]
pub struct ParserError {
    pub message: ParserErrorMessage,
    pub token: Option<Token>,
}

impl ParserError {
    fn message_to_string(&self) -> &str {
        match self.message {
            ParserErrorMessage::GetTokenError => "Failed to get token",
            ParserErrorMessage::LiteralTokenWithoutValue => "Literal type token without value",
            ParserErrorMessage::UnexpectedTokenType => "Unexpected token type",
            ParserErrorMessage::ExpectRightParenthesis => "'}' expected",
        }
    }
}

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.token.to_owned() {
            Some(token) => {
                write!(
                    f,
                    "[{}:{}] Error: {}\n{}",
                    token.position.row,
                    token.position.column,
                    self.message_to_string(),
                    token.lexeme
                )
            }
            _ => write!(f, "Error: {}", self.message_to_string()),
        }
    }
}

impl Error for ParserError {}
