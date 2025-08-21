use crate::{
    lexer::token::Token,
    parser::node::{Node, Visitor},
};

pub struct SetExpression {
    pub object: Box<dyn Node>,
    pub name: Token,
    pub value: Box<dyn Node>,
}

impl Node for SetExpression {
    fn accept(&mut self, visitor: &mut impl Visitor) -> impl Visitor
    where
        Self: Sized,
    {
        visitor.visit_set_expr(self)
    }
}
