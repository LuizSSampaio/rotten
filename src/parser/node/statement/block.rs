use crate::parser::node::Statement;

#[derive(Debug, Clone)]
pub struct BlockStatement {
    pub statements: Vec<Statement>,
}
