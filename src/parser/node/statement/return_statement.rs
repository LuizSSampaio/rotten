use crate::{
    parser::node::Statement,
    token::Token,
};

#[derive(Debug, Clone)]
pub struct ReturnStatement {
    pub keyword: Token,
    pub value: Option<Box<Statement>>,
}
