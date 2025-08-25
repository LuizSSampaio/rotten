use crate::{
    parser::node::{Node, Visitor},
    token::Token,
};

pub struct ClassStatement {
    pub name: Token,
    pub superclass: Option<Box<dyn Node>>,
    pub methods: Vec<Box<dyn Node>>,
}

impl Node for ClassStatement {
    fn accept<T>(&mut self, visitor: &mut impl Visitor<T>) -> T
    where
        Self: Sized,
    {
        visitor.visit_class_stmt(self)
    }
}
