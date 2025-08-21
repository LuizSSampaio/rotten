use crate::parser::node::{Node, Visitor};

pub struct GroupingExpression {
    pub expression: Box<dyn Node>,
}

impl Node for GroupingExpression {
    fn accept(&mut self, visitor: &mut impl Visitor) -> impl Visitor
    where
        Self: Sized,
    {
        visitor.visit_grouping_expr(self)
    }
}
