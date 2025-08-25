use crate::{
    parser::node::{Node, Visitor},
    token::Token,
};

pub struct ReturnStatement {
    pub keyword: Token,
    pub value: Option<Box<dyn Node>>,
}

impl Node for ReturnStatement {
    fn accept<T>(&mut self, visitor: &mut impl Visitor<T>) -> T
    where
        Self: Sized,
    {
        visitor.visit_return_stmt(self)
    }
}
