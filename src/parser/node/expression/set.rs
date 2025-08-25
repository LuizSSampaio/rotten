use crate::{
    token::Token,
    parser::node::{Expression, ExpressionVisitor},
};

pub struct SetExpression {
    pub object: Box<dyn Expression>,
    pub name: Token,
    pub value: Box<dyn Expression>,
}

impl Expression for SetExpression {
    fn accept<T>(&mut self, visitor: &mut impl ExpressionVisitor<T>) -> T
    where
        Self: Sized,
    {
        visitor.visit_set_expr(self)
    }
}
