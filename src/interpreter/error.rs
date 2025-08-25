use std::error::Error;
use std::fmt::Display;

use crate::token::Token;

#[derive(Debug, Clone)]
pub enum InterpreterErrorMessage {
    Unreachable,
}

#[derive(Debug, Clone)]
pub struct InterpreterError {
    pub message: InterpreterErrorMessage,
    pub token: Option<Token>,
}

impl InterpreterError {
    fn message_to_string(&self) -> &str {
        match self.message {
            InterpreterErrorMessage::Unreachable => "Unreachable",
        }
    }
}

impl Display for InterpreterError {
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

impl Error for InterpreterError {}
