use std::sync::{Arc, RwLock};

use crate::{
    interpreter::Interpreter,
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
    pub fn new(
        class: Class,
        interpreter: &mut Interpreter,
        arguments: Vec<TokenValue>,
    ) -> anyhow::Result<Arc<RwLock<Self>>> {
        let this = Self {
            class: class.clone(),
            fields: Default::default(),
            this: None,
        };

        let res = Arc::new(RwLock::new(this));
        res.write().unwrap().this = Some(res.clone());

        if let Some(initializer) = res.read().unwrap().class.methods.get(&class.name) {
            let mut initializer = initializer.to_owned();
            initializer.data.this = Some(res.clone());
            (initializer.call)(interpreter, &mut initializer.data, &arguments)?;
        }

        Ok(res)
    }

    pub fn get(&self, name: &Token) -> Option<TokenValue> {
        if let Some(val) = self.fields.get(name) {
            return Some(val);
        }

        if name.lexeme == self.class.name {
            return Some(TokenValue::Instance(self.this.clone().unwrap()));
        }

        if let Some(method) = self.class.methods.get(&name.lexeme) {
            let mut method = method.to_owned();
            method.data.this = self.this.clone();
            return Some(TokenValue::Function(method));
        }

        None
    }
}
