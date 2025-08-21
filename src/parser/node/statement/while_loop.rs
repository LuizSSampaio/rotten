use crate::parser::node::{Node, Visitor};

pub struct WhileStatement {
    pub condition: Box<dyn Node>,
    pub body: Box<dyn Node>,
}

impl Node for WhileStatement {
    fn accept<T>(&mut self, visitor: &mut impl Visitor) -> T
    where
        Self: Sized,
    {
        visitor.visit_while_stmt::<T>(self)
    }
}
