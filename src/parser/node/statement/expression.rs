use crate::parser::node::{Node, Visitor};

pub struct ExpressionStatement {
    pub expression: Box<dyn Node>,
}

impl Node for ExpressionStatement {
    fn accept<T>(&mut self, visitor: &mut impl Visitor) -> T
    where
        Self: Sized,
    {
        visitor.visit_expression_stmt::<T>(self)
    }
}
