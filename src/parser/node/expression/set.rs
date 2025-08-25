use crate::{
    parser::node::Expression,
    token::Token,
};

#[derive(Debug, Clone)]
pub struct SetExpression {
    pub object: Box<Expression>,
    pub name: Token,
    pub value: Box<Expression>,
}
