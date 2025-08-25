use crate::{
    parser::node::{Statement, StatementVisitor},
    token::Token,
};

pub struct ClassStatement {
    pub name: Token,
    pub superclass: Option<Box<dyn Statement>>,
    pub methods: Vec<Box<dyn Statement>>,
}

impl Statement for ClassStatement {
    fn accept<T>(&mut self, visitor: &mut impl StatementVisitor<T>) -> T
    where
        Self: Sized,
    {
        visitor.visit_class_stmt(self)
    }
}
