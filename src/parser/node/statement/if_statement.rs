use crate::parser::node::{Node, Visitor};

pub struct IfStatement {
    pub condition: Box<dyn Node>,
    pub then_branch: Box<dyn Node>,
    pub else_branch: Option<Box<dyn Node>>,
}

impl Node for IfStatement {
    fn accept<T>(&mut self, visitor: &mut impl Visitor<T>) -> T
    where
        Self: Sized,
    {
        visitor.visit_if_stmt(self)
    }
}
