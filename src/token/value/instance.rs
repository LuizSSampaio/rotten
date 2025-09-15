use crate::{memory::environment::Environment, token::value::Class};

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
}
