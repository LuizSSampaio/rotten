use crate::parser::node::{Statement, StatementVisitor};

pub struct WhileStatement {
    pub condition: Box<dyn Statement>,
    pub body: Box<dyn Statement>,
}

impl Statement for WhileStatement {
    fn accept<T>(&mut self, visitor: &mut impl StatementVisitor<T>) -> T
    where
        Self: Sized,
    {
        visitor.visit_while_stmt(self)
    }
}
