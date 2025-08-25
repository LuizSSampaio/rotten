use crate::token::Token;

#[derive(Debug, Clone)]
pub struct SuperExpression {
    pub keyword: Token,
    pub method: Token,
}
