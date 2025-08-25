use crate::{
    parser::node::Expression,
    token::Token,
};

#[derive(Debug, Clone)]
pub struct CallExpression {
    pub callee: Box<Expression>,
    pub paren: Token,
    pub arguments: Vec<Expression>,
}
