use crate::parser::node::{Statement, StatementVisitor};

pub struct ExpressionStatement {
    pub expression: Box<dyn Statement>,
}

impl Statement for ExpressionStatement {
    fn accept<T>(&mut self, visitor: &mut impl StatementVisitor<T>) -> T
    where
        Self: Sized,
    {
        visitor.visit_expression_stmt(self)
    }
}
