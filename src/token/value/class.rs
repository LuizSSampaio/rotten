use std::collections::HashMap;

use crate::token::value::function::Function;

#[derive(Debug, Clone)]
pub struct Class {
    pub name: String,
    pub methods: HashMap<String, Function>,
}
