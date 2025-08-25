use crate::parser::node::Statement;

#[derive(Debug, Clone)]
pub struct WhileStatement {
    pub condition: Box<Statement>,
    pub body: Box<Statement>,
}
