use crate::{
    parser::node::{Expression, ExpressionVisitor},
    token::Token,
};

pub struct AssignExpression {
    pub token: Token,
    pub value: Box<dyn Expression>,
}

impl Expression for AssignExpression {
    fn accept<T>(&mut self, visitor: &mut impl ExpressionVisitor<T>) -> T
    where
        Self: Sized,
    {
        visitor.visit_assign_expr(self)
    }
}
