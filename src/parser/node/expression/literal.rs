use crate::parser::node::{Node, Visitor};

#[derive(Clone, Debug)]
pub enum Value {
    Bool(bool),
    Nil,
    Number(f64),
    String(String),
}

pub struct LiteralExpression {
    pub value: Value,
}

impl Node for LiteralExpression {
    fn accept(&mut self, visitor: &mut impl Visitor) -> impl Visitor
    where
        Self: Sized,
    {
        visitor.visit_literal_expr(self)
    }
}
