use crate::lexer::token::{Token, TokenPosition, TokenType};

pub struct Emitter {
    pub tokens: Vec<Token>,
}

impl Emitter {
    pub fn new() -> Self {
        Self { tokens: Vec::new() }
    }

    pub fn add_token(&mut self, kind: TokenType, lexeme: String, row: usize, column: usize) {
        self.tokens.push(Token {
            kind,
            lexeme,
            position: TokenPosition { row, column },
        });
    }
}
