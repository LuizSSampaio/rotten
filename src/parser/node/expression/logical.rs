use crate::{
    parser::node::Expression,
    token::Token,
};

#[derive(Debug, Clone)]
pub struct LogicalExpression {
    pub left: Box<Expression>,
    pub operator: Token,
    pub right: Box<Expression>,
}
