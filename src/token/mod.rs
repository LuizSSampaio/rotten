pub mod kind;
pub mod value;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Token {
    pub kind: kind::TokenType,
    pub value: Option<value::TokenValue>,
    pub lexeme: String,
    pub position: TokenPosition,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct TokenPosition {
    pub row: usize,
    pub column: usize,
}
