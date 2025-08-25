use crate::parser::node::{Statement, StatementVisitor};

pub struct ForStatement {
    pub initializer: Option<Box<dyn Statement>>,
    pub condition: Option<Box<dyn Statement>>,
    pub increment: Option<Box<dyn Statement>>,
    pub body: Box<dyn Statement>,
}

impl Statement for ForStatement {
    fn accept<T>(&mut self, visitor: &mut impl StatementVisitor<T>) -> T
    where
        Self: Sized,
    {
        visitor.visit_for_stmt(self)
    }
}
