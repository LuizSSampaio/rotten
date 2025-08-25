use crate::{
    parser::node::Statement,
    token::Token,
};

#[derive(Debug, Clone)]
pub struct FunctionStatement {
    pub name: Token,
    pub params: Vec<Token>,
    pub body: Vec<Statement>,
}
