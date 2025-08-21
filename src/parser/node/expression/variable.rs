use crate::{
    lexer::token::Token,
    parser::node::{Node, Visitor},
};

pub struct VariableExpression {
    pub name: Token,
}

impl Node for VariableExpression {
    fn accept(&mut self, visitor: &mut impl Visitor) -> impl Visitor
    where
        Self: Sized,
    {
        visitor.visit_variable_expr(self)
    }
}
