use crate::{
    lexer::token::Token,
    parser::node::{Node, Visitor},
};

pub struct ThisExpression {
    pub keyword: Token,
}

impl Node for ThisExpression {
    fn accept<T>(&mut self, visitor: &mut impl Visitor<T>) -> T
    where
        Self: Sized,
    {
        visitor.visit_this_expr(self)
    }
}
