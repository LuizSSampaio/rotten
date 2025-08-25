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
    fn accept<T>(&mut self, visitor: &mut impl Visitor<T>) -> T
    where
        Self: Sized,
    {
        visitor.visit_logical_expr(self)
    }
}
