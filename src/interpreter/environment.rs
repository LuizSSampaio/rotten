use std::collections::HashMap;

use crate::{
    interpreter::error::{InterpreterError, InterpreterErrorMessage},
    token::{Token, value::TokenValue},
};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Environment {
    values: HashMap<String, TokenValue>,
}

impl Environment {
    pub fn define(&mut self, name: String, value: TokenValue) {
        self.values.insert(name, value);
    }

    pub fn get(&mut self, name: Token) -> anyhow::Result<TokenValue> {
        match self.values.get(&name.lexeme) {
            Some(val) => Ok(val.to_owned()),
            None => Err(InterpreterError {
                message: InterpreterErrorMessage::UndefinedVariable {
                    lexeme: name.lexeme.to_owned(),
                },
                token: Some(name),
            }
            .into()),
        }
    }
}

