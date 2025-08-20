use anyhow::Result;

use crate::lexer::error::LexerError;
use crate::lexer::token::TokenPosition;

pub struct Reader {
    pub source: Vec<char>,
    pub start: usize,
    pub current: usize,
    pub row: usize,
    pub column: usize,
}

impl Reader {
    pub fn new(source: String) -> Self {
        Self {
            source: source.chars().collect(),
            start: 0,
            current: 0,
            row: 1,
            column: 1,
        }
    }

    pub fn advance(&mut self) -> Result<char> {
        match self.source.get(self.current) {
            Some(&c) => {
                self.current += 1;
                self.column += 1;
                Ok(c)
            }
            None => Err(LexerError {
                message: String::from("Unexpected character."),
                lexeme: self.current_lexeme(),
                position: TokenPosition {
                    row: self.row,
                    column: self.column,
                },
            }
            .into()),
        }
    }

    pub fn next_is(&mut self, expected: char) -> bool {
        if self.current >= self.source.len() {
            return false;
        }
        if self.source[self.current] == expected {
            let _ = self.advance();
            true
        } else {
            false
        }
    }

    pub fn next_row(&mut self) {
        self.row += 1;
        self.column = 1;
    }

    pub fn peek(&self) -> char {
        self.source.get(self.current).copied().unwrap_or('\0')
    }

    pub fn peek_next(&self) -> char {
        self.source.get(self.current + 1).copied().unwrap_or('\0')
    }

    pub fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    pub fn current_lexeme(&self) -> String {
        self.source[self.start..self.current].iter().collect()
    }

    pub fn calculate_column(&self, lexeme_len: usize) -> usize {
        self.column - lexeme_len
    }
}
