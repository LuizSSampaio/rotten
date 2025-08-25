use crate::{
    token::Token,
    parser::node::{Expression, ExpressionVisitor},
};

pub struct ThisExpression {
    pub keyword: Token,
}

impl Expression for ThisExpression {
    fn accept<T>(&mut self, visitor: &mut impl ExpressionVisitor<T>) -> T
    where
        Self: Sized,
    {
        visitor.visit_this_expr(self)
    }
}
