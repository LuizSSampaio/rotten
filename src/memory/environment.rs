use std::collections::{HashMap, hash_map};

use crate::token::{Token, value::TokenValue};

use anyhow::Result;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Environment {
    data: HashMap<String, TokenValue>,
}

impl Environment {
    pub fn define(&mut self, name: String, value: TokenValue) {
        self.data.insert(name, value);
    }

    pub fn get(&self, name: &Token) -> Option<TokenValue> {
        self.data.get(&name.lexeme).map(|val| val.to_owned())
    }

    pub fn assign(&mut self, name: &Token, value: TokenValue) -> Result<()> {
        if let hash_map::Entry::Occupied(mut e) = self.data.entry(name.lexeme.to_owned()) {
            e.insert(value);
            return Ok(());
        }

        anyhow::bail!("Undefined variable")
    }
}
