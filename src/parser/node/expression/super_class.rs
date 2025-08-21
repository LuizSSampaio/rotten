use crate::{
    lexer::token::Token,
    parser::node::{Node, Visitor},
};

pub struct SuperExpression {
    pub keyword: Token,
    pub method: Token,
}

impl Node for SuperExpression {
    fn accept<T>(&mut self, visitor: &mut impl Visitor) -> T
    where
        Self: Sized,
    {
        visitor.visit_super_expr::<T>(self)
    }
}
