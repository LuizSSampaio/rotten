use crate::{
    lexer::token::Token,
    parser::node::{Node, Visitor},
};

pub struct GetExpression {
    pub object: Box<dyn Node>,
    pub name: Token,
}

impl Node for GetExpression {
    fn accept(&mut self, visitor: &mut impl Visitor) -> impl Visitor
    where
        Self: Sized,
    {
        visitor.visit_get_expr(self)
    }
}
