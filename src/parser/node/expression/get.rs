use crate::{
    parser::node::Expression,
    token::Token,
};

#[derive(Debug, Clone)]
pub struct GetExpression {
    pub object: Box<Expression>,
    pub name: Token,
}
