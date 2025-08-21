use crate::{
    lexer::token::Token,
    parser::node::{Node, Visitor},
};

pub struct UnaryExpression {
    pub operator: Token,
    pub right: Box<dyn Node>,
}

impl Node for UnaryExpression {
    fn accept(&mut self, visitor: &mut impl Visitor) -> impl Visitor
    where
        Self: Sized,
    {
        visitor.visit_unary_expr(self)
    }
}
