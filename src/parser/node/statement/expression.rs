use crate::parser::node::{Node, Visitor};

pub struct ExpressionStatement {
    pub expression: Box<dyn Node>,
}

impl Node for ExpressionStatement {
    fn accept(&mut self, visitor: &mut impl Visitor) -> impl Visitor
    where
        Self: Sized,
    {
        visitor.visit_expression_stmt(self)
    }
}
