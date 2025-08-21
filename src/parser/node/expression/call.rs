use crate::{
    lexer::token::Token,
    parser::node::{Node, Visitor},
};

pub struct CallExpression {
    pub callee: Box<dyn Node>,
    pub paren: Token,
    pub arguments: Vec<Box<dyn Node>>,
}

impl Node for CallExpression {
    fn accept(&mut self, visitor: &mut impl Visitor) -> impl Visitor
    where
        Self: Sized,
    {
        visitor.visit_call_expr(self)
    }
}
