use crate::parser::node::{Node, Visitor};

pub struct BlockStatement {
    pub statements: Vec<Box<dyn Node>>,
}

impl Node for BlockStatement {
    fn accept<T>(&mut self, visitor: &mut impl Visitor<T>) -> T
    where
        Self: Sized,
    {
        visitor.visit_block_stmt(self)
    }
}
