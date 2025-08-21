use crate::{
    lexer::token::Token,
    parser::node::{Node, Visitor},
};

pub struct AssignExpression {
    pub token: Token,
    pub value: Box<dyn Node>,
}

impl Node for AssignExpression {
    fn accept<T>(&mut self, visitor: &mut impl Visitor) -> T
    where
        Self: Sized,
    {
        visitor.visit_assign_expr::<T>(self)
    }
}
