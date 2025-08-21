use crate::parser::node::{Node, Visitor};

pub struct GroupingExpression {
    pub expression: Box<dyn Node>,
}

impl Node for GroupingExpression {
    fn accept<T>(&mut self, visitor: &mut impl Visitor) -> T
    where
        Self: Sized,
    {
        visitor.visit_grouping_expr::<T>(self)
    }
}
