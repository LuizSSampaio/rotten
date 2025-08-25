use crate::parser::node::Expression;

#[derive(Debug, Clone)]
pub struct GroupingExpression {
    pub expression: Box<Expression>,
}
