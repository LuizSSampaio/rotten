use crate::{
    parser::node::Expression,
    token::Token,
};

#[derive(Debug, Clone)]
pub struct UnaryExpression {
    pub operator: Token,
    pub right: Box<Expression>,
}
