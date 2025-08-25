use crate::{
    token::Token,
    parser::node::{Node, Visitor},
};

pub struct UnaryExpression {
    pub operator: Token,
    pub right: Box<dyn Node>,
}

impl Node for UnaryExpression {
    fn accept<T>(&mut self, visitor: &mut impl Visitor<T>) -> T
    where
        Self: Sized,
    {
        visitor.visit_unary_expr(self)
    }
}
