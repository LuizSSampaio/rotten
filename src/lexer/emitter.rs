use crate::token::{Token, TokenPosition, TokenType, value::TokenValue};

pub struct Emitter {
    tokens: Vec<Token>,
}

impl Emitter {
    pub fn new() -> Self {
        Self { tokens: Vec::new() }
    }

    pub fn tokens(&self) -> Vec<Token> {
        self.tokens.clone()
    }

    pub fn add_token(
        &mut self,
        kind: TokenType,
        value: Option<TokenValue>,
        lexeme: String,
        row: usize,
        column: usize,
    ) {
        self.tokens.push(Token {
            kind,
            value,
            lexeme,
            position: TokenPosition { row, column },
        });
    }
}
