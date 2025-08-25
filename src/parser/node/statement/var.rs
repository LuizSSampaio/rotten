use crate::{
    parser::node::{Statement, StatementVisitor},
    token::Token,
};

pub struct VarStatement {
    pub name: Token,
    pub initializer: Option<Box<dyn Statement>>,
}

impl Statement for VarStatement {
    fn accept<T>(&mut self, visitor: &mut impl StatementVisitor<T>) -> T
    where
        Self: Sized,
    {
        visitor.visit_var_stmt(self)
    }
}
