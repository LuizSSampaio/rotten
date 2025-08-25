use crate::parser::node::Statement;

#[derive(Debug, Clone)]
pub struct ExpressionStatement {
    pub expression: Box<Statement>,
}
