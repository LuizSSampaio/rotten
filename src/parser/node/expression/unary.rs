use crate::{
    parser::node::{Expression, ExpressionVisitor},
    token::Token,
};

pub struct UnaryExpression {
    pub operator: Token,
    pub right: Box<dyn Expression>,
}

impl Expression for UnaryExpression {
    fn accept<T>(&mut self, visitor: &mut impl ExpressionVisitor<T>) -> T
    where
        Self: Sized,
    {
        visitor.visit_unary_expr(self)
    }
}
