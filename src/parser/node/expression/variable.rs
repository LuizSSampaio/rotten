use crate::{
    token::Token,
    parser::node::{Expression, ExpressionVisitor},
};

pub struct VariableExpression {
    pub name: Token,
}

impl Expression for VariableExpression {
    fn accept<T>(&mut self, visitor: &mut impl ExpressionVisitor<T>) -> T
    where
        Self: Sized,
    {
        visitor.visit_variable_expr(self)
    }
}
