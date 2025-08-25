use crate::parser::node::{Expression, ExpressionVisitor};

pub struct GroupingExpression {
    pub expression: Box<dyn Expression>,
}

impl Expression for GroupingExpression {
    fn accept<T>(&mut self, visitor: &mut impl ExpressionVisitor<T>) -> T
    where
        Self: Sized,
    {
        visitor.visit_grouping_expr(self)
    }
}
