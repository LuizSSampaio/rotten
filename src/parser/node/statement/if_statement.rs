use crate::parser::node::Statement;

#[derive(Debug, Clone)]
pub struct IfStatement {
    pub condition: Box<Statement>,
    pub then_branch: Box<Statement>,
    pub else_branch: Option<Box<Statement>>,
}
