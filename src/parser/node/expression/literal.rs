use crate::{
    token::TokenValue,
    parser::node::{Expression, ExpressionVisitor},
};

pub struct LiteralExpression {
    pub value: TokenValue,
}

impl Expression for LiteralExpression {
    fn accept<T>(&mut self, visitor: &mut impl ExpressionVisitor<T>) -> T
    where
        Self: Sized,
    {
        visitor.visit_literal_expr(self)
    }
}
