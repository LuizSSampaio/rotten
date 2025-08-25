use crate::parser::node::{Statement, StatementVisitor};

pub struct BlockStatement {
    pub statements: Vec<Box<dyn Statement>>,
}

impl Statement for BlockStatement {
    fn accept<T>(&mut self, visitor: &mut impl StatementVisitor<T>) -> T
    where
        Self: Sized,
    {
        visitor.visit_block_stmt(self)
    }
}
