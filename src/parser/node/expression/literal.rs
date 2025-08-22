use crate::{
    lexer::token::TokenValue,
    parser::node::{Node, Visitor},
};

pub struct LiteralExpression {
    pub value: TokenValue,
}

impl Node for LiteralExpression {
    fn accept<T>(&mut self, visitor: &mut impl Visitor) -> T
    where
        Self: Sized,
    {
        visitor.visit_literal_expr::<T>(self)
    }
}
