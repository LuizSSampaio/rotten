use crate::{
    token::Token,
    parser::node::{Expression, ExpressionVisitor},
};

pub struct CallExpression {
    pub callee: Box<dyn Expression>,
    pub paren: Token,
    pub arguments: Vec<Box<dyn Expression>>,
}

impl Expression for CallExpression {
    fn accept<T>(&mut self, visitor: &mut impl ExpressionVisitor<T>) -> T
    where
        Self: Sized,
    {
        visitor.visit_call_expr(self)
    }
}
