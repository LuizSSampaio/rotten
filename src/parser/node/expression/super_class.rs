use crate::{
    token::Token,
    parser::node::{Expression, ExpressionVisitor},
};

pub struct SuperExpression {
    pub keyword: Token,
    pub method: Token,
}

impl Expression for SuperExpression {
    fn accept<T>(&mut self, visitor: &mut impl ExpressionVisitor<T>) -> T
    where
        Self: Sized,
    {
        visitor.visit_super_expr(self)
    }
}
