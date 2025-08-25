use crate::{
    token::Token,
    parser::node::{Expression, ExpressionVisitor},
};

pub struct LogicalExpression {
    pub left: Box<dyn Expression>,
    pub operator: Token,
    pub right: Box<dyn Expression>,
}

impl Expression for LogicalExpression {
    fn accept<T>(&mut self, visitor: &mut impl ExpressionVisitor<T>) -> T
    where
        Self: Sized,
    {
        visitor.visit_logical_expr(self)
    }
}
