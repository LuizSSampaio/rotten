use crate::parser::node::{Statement, StatementVisitor};

pub struct IfStatement {
    pub condition: Box<dyn Statement>,
    pub then_branch: Box<dyn Statement>,
    pub else_branch: Option<Box<dyn Statement>>,
}

impl Statement for IfStatement {
    fn accept<T>(&mut self, visitor: &mut impl StatementVisitor<T>) -> T
    where
        Self: Sized,
    {
        visitor.visit_if_stmt(self)
    }
}
