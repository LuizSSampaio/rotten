use std::collections::{HashMap, hash_map};

use anyhow::Result;

use crate::{
    interpreter::error::{InterpreterError, InterpreterErrorMessage},
    token::{Token, value::TokenValue},
};

#[derive(Debug, Clone, PartialEq)]
pub struct EnvironmentHandler {
    environments: Vec<HashMap<String, TokenValue>>,
}

impl Default for EnvironmentHandler {
    fn default() -> Self {
        let mut handler = Self {
            environments: Default::default(),
        };
        handler.create_environment();

        handler
    }
}

impl EnvironmentHandler {
    pub fn create_environment(&mut self) {
        self.environments.push(Default::default());
    }

    pub fn delete_environment(&mut self) -> Result<()> {
        if self.environments.len() <= 1 {
            return Err(anyhow::anyhow!("Need at least one environment"));
        }

        self.environments.pop();
        Ok(())
    }

    pub fn define(&mut self, name: String, value: TokenValue) -> Result<()> {
        match self.environments.last_mut() {
            Some(env) => {
                env.insert(name, value);
                Ok(())
            }
            None => Err(anyhow::anyhow!("Need at least one environment")),
        }
    }

    pub fn get(&mut self, name: Token) -> Option<TokenValue> {
        for env in self.environments.iter_mut().rev() {
            if let Some(val) = env.get(&name.lexeme) {
                return Some(val.to_owned());
            }
        }

        None
    }

    pub fn assign(&mut self, name: Token, value: TokenValue) -> Result<()> {
        for env in self.environments.iter_mut().rev() {
            if let hash_map::Entry::Occupied(mut e) = env.entry(name.lexeme.to_owned()) {
                e.insert(value);
                return Ok(());
            }
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
