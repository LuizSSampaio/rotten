use crate::{
    lexer::token::Token,
    parser::node::{Node, Visitor},
};

pub struct LogicalExpression {
    pub left: Box<dyn Node>,
    pub operator: Token,
    pub right: Box<dyn Node>,
}

impl Node for LogicalExpression {
    fn accept(&mut self, visitor: &mut impl Visitor) -> impl Visitor
    where
        Self: Sized,
    {
        visitor.visit_logical_expr(self)
    }
}
