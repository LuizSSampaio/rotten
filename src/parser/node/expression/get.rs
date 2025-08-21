use crate::{
    lexer::token::Token,
    parser::node::{Node, Visitor},
};

pub struct GetExpression {
    pub object: Box<dyn Node>,
    pub name: Token,
}

impl Node for GetExpression {
    fn accept<T>(&mut self, visitor: &mut impl Visitor) -> T
    where
        Self: Sized,
    {
        visitor.visit_get_expr::<T>(self)
    }
}
