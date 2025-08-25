use crate::parser::node::{Node, Visitor};

pub struct ForStatement {
    pub initializer: Option<Box<dyn Node>>,
    pub condition: Option<Box<dyn Node>>,
    pub increment: Option<Box<dyn Node>>,
    pub body: Box<dyn Node>,
}

impl Node for ForStatement {
    fn accept<T>(&mut self, visitor: &mut impl Visitor<T>) -> T
    where
        Self: Sized,
    {
        visitor.visit_for_stmt(self)
    }
}
