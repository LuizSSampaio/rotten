use crate::{
    lexer::token::Token,
    parser::node::{Node, Visitor},
};

pub struct ClassStatement {
    pub name: Token,
    pub superclass: Option<Box<dyn Node>>,
    pub methods: Vec<Box<dyn Node>>,
}

impl Node for ClassStatement {
    fn accept(&mut self, visitor: &mut impl Visitor) -> impl Visitor
    where
        Self: Sized,
    {
        visitor.visit_class_stmt(self)
    }
}
