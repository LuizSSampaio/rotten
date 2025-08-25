use crate::{
    parser::node::Statement,
    token::Token,
};

#[derive(Debug, Clone)]
pub struct VarStatement {
    pub name: Token,
    pub initializer: Option<Box<Statement>>,
}
