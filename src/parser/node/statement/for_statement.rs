use crate::parser::node::Statement;

#[derive(Debug, Clone)]
pub struct ForStatement {
    pub initializer: Option<Box<Statement>>,
    pub condition: Option<Box<Statement>>,
    pub increment: Option<Box<Statement>>,
    pub body: Box<Statement>,
}
