use std::{error::Error, fmt::Display};

use crate::lexer::token::{Token, TokenPosition, TokenType};

pub(in crate::lexer) struct Scanner {
    source: String,
    tokens: Vec<Token>,

    start: usize,
    current: usize,
    row: usize,
    column: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            row: 1,
            column: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> anyhow::Result<Vec<Token>> {
        while self.current < self.source.len() {
            self.start = self.current;
            self.scan_token()?
        }

        self.tokens.push(Token {
            kind: TokenType::EndOfFile,
            lexeme: String::new(),
            position: TokenPosition { row: 0, column: 1 },
        });

        Ok(self.tokens.to_owned())
    }

    fn advance(&mut self) -> char {
        let character = self.source.as_bytes()[self.current] as char;
        self.current += 1;
        self.column += 1;

        character
    }

    fn scan_token(&mut self) -> anyhow::Result<()> {
        match self.advance() {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '!' => {
                if self.next_is('=') {
                    self.add_token(TokenType::BangEqual);
                } else {
                    self.add_token(TokenType::Bang);
                }
            }
            '=' => {
                if self.next_is('=') {
                    self.add_token(TokenType::EqualEqual);
                } else {
                    self.add_token(TokenType::Equal);
                }
            }
            '<' => {
                if self.next_is('=') {
                    self.add_token(TokenType::LessEqual);
                } else {
                    self.add_token(TokenType::Less);
                }
            }
            '>' => {
                if self.next_is('=') {
                    self.add_token(TokenType::GreaterEqual);
                } else {
                    self.add_token(TokenType::Greater);
                }
            }
            '/' => self.add_token(TokenType::Slash),
            _ => {
                return Err(ScannerError {
                    message: String::from("Unexpected character."),
                    lexeme: self.source[self.start..self.current].to_string(),
                    position: TokenPosition {
                        row: self.row,
                        column: self.column,
                    },
                }
                .into());
            }
        }

        Ok(())
    }

    fn next_is(&mut self, expected: char) -> bool {
        if self.current >= self.source.len() {
            return false;
        }
        if self.source.as_bytes()[self.current] as char != expected {
            return false;
        }

        self.advance();
        true
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.tokens.push(Token {
            kind: token_type,
            lexeme: self.source[self.start..self.current].to_string(),
            position: TokenPosition {
                row: self.row,
                column: self.column,
            },
        });
    }
}

#[derive(Debug, Clone)]
struct ScannerError {
    message: String,
    lexeme: String,
    position: TokenPosition,
}

impl Display for ScannerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}:{}] Lexer Error: {}\n{}",
            self.position.row, self.position.column, self.message, self.lexeme
        )
    }
}

impl Error for ScannerError {}
