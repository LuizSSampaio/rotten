use crate::{
    lexer::token::Token,
    parser::node::{Node, Visitor},
};

pub struct FunctionStatement {
    pub name: Token,
    pub params: Vec<Token>,
    pub body: Vec<Box<dyn Node>>,
}

impl Node for FunctionStatement {
    fn accept(&mut self, visitor: &mut impl Visitor) -> impl Visitor
    where
        Self: Sized,
    {
        visitor.visit_function_stmt(self)
    }
}
