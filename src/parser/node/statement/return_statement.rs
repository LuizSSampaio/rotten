use crate::{
    parser::node::{Statement, StatementVisitor},
    token::Token,
};

pub struct ReturnStatement {
    pub keyword: Token,
    pub value: Option<Box<dyn Statement>>,
}

impl Statement for ReturnStatement {
    fn accept<T>(&mut self, visitor: &mut impl StatementVisitor<T>) -> T
    where
        Self: Sized,
    {
        visitor.visit_return_stmt(self)
    }
}
