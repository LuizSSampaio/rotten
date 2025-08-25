use crate::{
    lexer::token::Token,
    parser::node::{Node, Visitor},
};

pub struct VariableExpression {
    pub name: Token,
}

impl Node for VariableExpression {
    fn accept<T>(&mut self, visitor: &mut impl Visitor<T>) -> T
    where
        Self: Sized,
    {
        visitor.visit_variable_expr(self)
    }
}
