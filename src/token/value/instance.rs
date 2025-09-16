use crate::{
    memory::environment::Environment,
    token::{
        Token,
        value::{Class, TokenValue},
    },
};

#[derive(Debug, Clone)]
pub struct Instance {
    pub class: Class,
    pub fields: Environment,
}

impl Instance {
    pub fn new(class: Class) -> Self {
        Self {
            class,
            fields: Default::default(),
        }
    }

    pub fn get(&self, name: &Token) -> Option<TokenValue> {
        if let Some(val) = self.fields.get(name) {
            return Some(val);
        }

        if let Some(method) = self.class.methods.get(&name.lexeme) {
            return Some(TokenValue::Function(method.to_owned()));
        }

        None
    }
}
