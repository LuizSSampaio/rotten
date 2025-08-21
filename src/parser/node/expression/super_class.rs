use crate::{
    lexer::token::Token,
    parser::node::{Node, Visitor},
};

pub struct SuperExpression {
    pub keyword: Token,
    pub method: Token,
}

impl Node for SuperExpression {
    fn accept(&mut self, visitor: &mut impl Visitor) -> impl Visitor
    where
        Self: Sized,
    {
        visitor.visit_super_expr(self)
    }
}
