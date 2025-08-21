use crate::{
    lexer::token::Token,
    parser::node::{Node, Visitor},
};

pub struct VarStatement {
    pub name: Token,
    pub initializer: Option<Box<dyn Node>>,
}

impl Node for VarStatement {
    fn accept<T>(&mut self, visitor: &mut impl Visitor) -> T
    where
        Self: Sized,
    {
        visitor.visit_var_stmt::<T>(self)
    }
}
