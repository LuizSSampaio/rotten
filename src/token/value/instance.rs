use std::sync::{Arc, RwLock};

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
    this: Option<Arc<RwLock<Instance>>>,
}

impl Instance {
    pub fn new(class: Class) -> Arc<RwLock<Self>> {
        let this = Self {
            class,
            fields: Default::default(),
            this: None,
        };

        let res = Arc::new(RwLock::new(this));
        res.write().unwrap().this = Some(res.clone());
        res
    }

    pub fn get(&self, name: &Token) -> Option<TokenValue> {
        if let Some(val) = self.fields.get(name) {
            return Some(val);
        }

        if let Some(method) = self.class.methods.get(&name.lexeme) {
            let mut method = method.to_owned();
            method.data.this = self.this.clone();
            return Some(TokenValue::Function(method));
        }

        None
    }
}
