use crate::{
    parser::node::Expression,
    token::Token,
};

#[derive(Debug, Clone)]
pub struct AssignExpression {
    pub token: Token,
    pub value: Box<Expression>,
}
