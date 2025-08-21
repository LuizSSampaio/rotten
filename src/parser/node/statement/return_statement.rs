use crate::{
    lexer::token::Token,
    parser::node::{Node, Visitor},
};

pub struct ReturnStatement {
    pub keyword: Token,
    pub value: Option<Box<dyn Node>>,
}

impl Node for ReturnStatement {
    fn accept<T>(&mut self, visitor: &mut impl Visitor) -> T
    where
        Self: Sized,
    {
        visitor.visit_return_stmt::<T>(self)
    }
}
