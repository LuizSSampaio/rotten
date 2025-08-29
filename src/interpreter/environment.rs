use std::collections::HashMap;

use anyhow::Result;

use crate::{
    interpreter::error::{InterpreterError, InterpreterErrorMessage},
    token::{Token, value::TokenValue},
};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Environment {
    enclosing: Option<Box<Environment>>,
    values: HashMap<String, TokenValue>,
}

impl Environment {
    pub fn new(enclosing: Box<Environment>) -> Self {
        Self {
            enclosing: Some(enclosing),
            values: HashMap::default(),
        }
    }

    pub fn define(&mut self, name: String, value: TokenValue) {
        self.values.insert(name, value);
    }

    pub fn get(&mut self, name: Token) -> Result<TokenValue> {
        match self.values.get(&name.lexeme) {
            Some(val) => Ok(val.to_owned()),
            None => {
                if let Some(mut enclosing) = self.enclosing.to_owned() {
                    return enclosing.get(name);
                }

                Err(InterpreterError {
                    message: InterpreterErrorMessage::UndefinedVariable {
                        lexeme: name.lexeme.to_owned(),
                    },
                    token: Some(name),
                }
                .into())
            }
        }
    }

    pub fn assign(&mut self, name: Token, value: TokenValue) -> Result<()> {
        match self.values.contains_key(&name.lexeme) {
            true => {
                self.values.insert(name.lexeme, value);
                Ok(())
            }
            false => {
                if let Some(mut enclosing) = self.enclosing.to_owned() {
                    return enclosing.assign(name, value);
                }

                Err(InterpreterError {
                    message: InterpreterErrorMessage::UndefinedVariable {
                        lexeme: name.lexeme.to_owned(),
                    },
                    token: Some(name),
                }
                .into())
            }
        }
    }
}

