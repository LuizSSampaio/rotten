use std::error::Error;
use std::fmt::Display;

use crate::token::{Token, value::TokenValue};

#[derive(Debug, Clone)]
pub enum InterpreterErrorMessage {
    Unreachable,
    UnexpectedValue { is: TokenValue, expect: TokenValue },
    DivisionByZero,
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
                format!(
                    "Unexpected value\nis: {}\nexpect: {}",
                    Self::format_token_value(is),
                    Self::format_token_value(expect)
                )
            }
            InterpreterErrorMessage::DivisionByZero => "Attempt to divide by zero".to_string(),
        }
    }

    fn format_token_value(value: &TokenValue) -> String {
        let debug_str = format!("{:?}", value);
        let variant_name = debug_str
            .split_once('(')
            .map(|(name, _)| name)
            .unwrap_or(&debug_str);
        variant_name.to_lowercase()
    }
}

impl Display for InterpreterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.token.to_owned() {
            Some(token) => {
                write!(
                    f,
                    "[{}:{}] Interpreter Error: {}\n{}",
                    token.position.row,
                    token.position.column,
                    self.message_to_string(),
                    token.lexeme
                )
            }
            _ => write!(f, "Interpreter Error: {}", self.message_to_string()),
        }
    }
}

impl Error for InterpreterError {}
