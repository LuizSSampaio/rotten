use crate::{
    parser::node::{Statement, StatementVisitor},
    token::Token,
};

pub struct FunctionStatement {
    pub name: Token,
    pub params: Vec<Token>,
    pub body: Vec<Box<dyn Statement>>,
}

impl Statement for FunctionStatement {
    fn accept<T>(&mut self, visitor: &mut impl StatementVisitor<T>) -> T
    where
        Self: Sized,
    {
        visitor.visit_function_stmt(self)
    }
}
