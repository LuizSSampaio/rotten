use crate::parser::node::{Node, Visitor};

pub struct WhileStatement {
    pub condition: Box<dyn Node>,
    pub body: Box<dyn Node>,
}

impl Node for WhileStatement {
    fn accept(&mut self, visitor: &mut impl Visitor) -> impl Visitor
    where
        Self: Sized,
    {
        visitor.visit_while_stmt(self)
    }
}
