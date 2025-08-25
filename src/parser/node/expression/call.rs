use crate::{
    token::Token,
    parser::node::{Node, Visitor},
};

pub struct CallExpression {
    pub callee: Box<dyn Node>,
    pub paren: Token,
    pub arguments: Vec<Box<dyn Node>>,
}

impl Node for CallExpression {
    fn accept<T>(&mut self, visitor: &mut impl Visitor<T>) -> T
    where
        Self: Sized,
    {
        visitor.visit_call_expr(self)
    }
}
