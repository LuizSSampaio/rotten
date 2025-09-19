use std::{collections::HashMap, sync::Arc};

use crate::token::value::function::Function;

#[derive(Debug, Clone)]
pub struct Class {
    pub name: String,
    superclass: Option<Arc<Class>>,
    methods: HashMap<String, Function>,
}

impl Class {
    pub fn new(
        name: String,
        superclass: Option<Arc<Class>>,
        methods: HashMap<String, Function>,
    ) -> Self {
        Self {
            name,
            superclass,
            methods,
        }
    }

    pub fn get(&self, identifier: String) -> Option<Function> {
        if let Some(method) = self.methods.get(&identifier) {
            return Some(method.to_owned());
        }

        self.get_from_super(identifier)
    }

    pub fn get_from_super(&self, identifier: String) -> Option<Function> {
        if let Some(superclass) = &self.superclass {
            return superclass.get(identifier);
        }

        None
    }
}
