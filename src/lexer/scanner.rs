use std::collections::HashMap;

use anyhow::Result;

use crate::lexer::emitter::Emitter;
use crate::lexer::error::LexerError;
use crate::lexer::keywords;
use crate::lexer::reader::Reader;
use crate::lexer::token::{Token, TokenPosition, TokenType};

pub(in crate::lexer) struct Scanner {
    reader: Reader,
    keywords: HashMap<&'static str, TokenType>,
    emitter: Emitter,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            reader: Reader::new(source),
            keywords: keywords::create_keywords(),
            emitter: Emitter::new(),
        }
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>> {
        while !self.reader.is_at_end() {
            self.reader.start_to_current();
            self.scan_token()?
        }

        self.emitter.add_token(
            TokenType::EndOfFile,
            String::new(),
            self.reader.row(),
            self.reader.column(),
        );

        Ok(self.emitter.tokens())
    }

    fn scan_token(&mut self) -> Result<()> {
        match self.reader.advance()? {
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
                if self.reader.next_is('=') {
                    self.add_token(TokenType::BangEqual);
                } else {
                    self.add_token(TokenType::Bang);
                }
            }
            '=' => {
                if self.reader.next_is('=') {
                    self.add_token(TokenType::EqualEqual);
                } else {
                    self.add_token(TokenType::Equal);
                }
            }
            '<' => {
                if self.reader.next_is('=') {
                    self.add_token(TokenType::LessEqual);
                } else {
                    self.add_token(TokenType::Less);
                }
            }
            '>' => {
                if self.reader.next_is('=') {
                    self.add_token(TokenType::GreaterEqual);
                } else {
                    self.add_token(TokenType::Greater);
                }
            }
            '/' => {
                if self.reader.next_is('/') {
                    while self.reader.peek() != '\n' && !self.reader.is_at_end() {
                        self.reader.advance()?;
                    }
                } else if self.reader.next_is('*') {
                    while self.reader.peek() != '*'
                        || self.reader.peek_next() != '/' && !self.reader.is_at_end()
                    {
                        self.reader.advance()?;
                    }
                    if !self.reader.is_at_end() {
                        self.reader.advance()?; // *
                        self.reader.advance()?; // /
                    }
                } else {
                    self.add_token(TokenType::Slash);
                }
            }
            '"' => self.string()?,
            '0'..='9' => self.number()?,
            'a'..='z' | 'A'..='Z' | '_' => self.identifier()?,
            '\r' | '\t' | ' ' => {}
            '\n' | '\0' => {
                self.reader.next_row();
            }
            _ => {
                return Err(LexerError {
                    message: String::from("Unexpected character."),
                    lexeme: self.reader.current_lexeme(),
                    position: TokenPosition {
                        row: self.reader.row(),
                        column: self.reader.column(),
                    },
                }
                .into());
            }
        }

        Ok(())
    }

    fn string(&mut self) -> Result<()> {
        while self.reader.peek() != '"' && !self.reader.is_at_end() {
            if self.reader.peek() == '\n' {
                self.reader.next_row();
            }
            self.reader.advance()?;
        }

        if self.reader.is_at_end() {
            return Err(LexerError {
                message: String::from("Unterminated string."),
                lexeme: self.reader.current_lexeme(),
                position: TokenPosition {
                    row: self.reader.row(),
                    column: self.reader.column(),
                },
            }
            .into());
        }

        self.reader.advance()?; // "

        let lexeme = self.reader.current_lexeme();
        let value = lexeme[1..lexeme.len() - 1].to_string();
        self.add_token(TokenType::String(value));
        Ok(())
    }

    fn number(&mut self) -> Result<()> {
        while self.reader.peek().is_numeric() {
            self.reader.advance()?;
        }

        if self.reader.peek() == '.' && self.reader.peek_next().is_numeric() {
            self.reader.advance()?;

            while self.reader.peek().is_numeric() {
                self.reader.advance()?;
            }
        }

        let lexeme = self.reader.current_lexeme();
        let value = lexeme.parse::<f64>().map_err(|e| LexerError {
            message: e.to_string(),
            lexeme: lexeme.clone(),
            position: TokenPosition {
                row: self.reader.row(),
                column: self.reader.column(),
            },
        })?;
        self.add_token(TokenType::Number(value));
        Ok(())
    }

    fn identifier(&mut self) -> Result<()> {
        while self.reader.peek().is_alphanumeric() {
            self.reader.advance()?;
        }

        let slice = self.reader.current_lexeme();
        let token_type = self
            .keywords
            .get(&*slice)
            .cloned()
            .unwrap_or(TokenType::Identifier);
        self.add_token(token_type);
        Ok(())
    }

    fn add_token(&mut self, token_type: TokenType) {
        let lexeme = self.reader.current_lexeme();
        let column = self.reader.calculate_column(lexeme.len());
        self.emitter
            .add_token(token_type, lexeme, self.reader.row(), column);
    }
}
