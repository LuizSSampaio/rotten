use crate::{
    token::Token,
    parser::node::{Expression, ExpressionVisitor},
};

pub struct GetExpression {
    pub object: Box<dyn Expression>,
    pub name: Token,
}

impl Expression for GetExpression {
    fn accept<T>(&mut self, visitor: &mut impl ExpressionVisitor<T>) -> T
    where
        Self: Sized,
    {
        visitor.visit_get_expr(self)
    }
}
