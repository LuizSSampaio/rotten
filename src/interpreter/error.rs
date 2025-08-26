use std::error::Error;
use std::fmt::Display;

use crate::token::{Token, value::TokenValue};

#[derive(Debug, Clone)]
pub enum InterpreterErrorMessage {
    Unreachable,
    UnexpectedValue { is: TokenValue, expect: TokenValue },
}

#[derive(Debug, Clone)]
pub struct InterpreterError {
    pub message: InterpreterErrorMessage,
    pub token: Option<Token>,
}

impl InterpreterError {
    fn message_to_string(&self) -> String {
        match &self.message {
            InterpreterErrorMessage::Unreachable => "Unreachable".to_string(),
            InterpreterErrorMessage::UnexpectedValue { is, expect } => {
                format!("Unexpected value\nis: {}\nexpect: {}", is, expect)
            }
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
