use crate::{
    parser::node::Statement,
    token::Token,
};

#[derive(Debug, Clone)]
pub struct ClassStatement {
    pub name: Token,
    pub superclass: Option<Box<Statement>>,
    pub methods: Vec<Statement>,
}
