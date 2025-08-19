use std::{collections::HashMap, error::Error, fmt::Display};

use crate::lexer::token::{Token, TokenPosition, TokenType};

pub(in crate::lexer) struct Scanner {
    source: String,
    tokens: Vec<Token>,

    start: usize,
    current: usize,
    row: usize,
    column: usize,

    keywords: HashMap<&'static str, TokenType>,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        let mut keywords = HashMap::new();
        keywords.insert("and", TokenType::And);
        keywords.insert("class", TokenType::Class);
        keywords.insert("else", TokenType::Else);
        keywords.insert("false", TokenType::False);
        keywords.insert("for", TokenType::For);
        keywords.insert("fun", TokenType::Fun);
        keywords.insert("if", TokenType::If);
        keywords.insert("nil", TokenType::Nil);
        keywords.insert("or", TokenType::Or);
        keywords.insert("print", TokenType::Print);
        keywords.insert("return", TokenType::Return);
        keywords.insert("super", TokenType::Super);
        keywords.insert("this", TokenType::This);
        keywords.insert("true", TokenType::True);
        keywords.insert("var", TokenType::Var);
        keywords.insert("while", TokenType::While);

        Self {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            row: 1,
            column: 0,
            keywords,
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
            position: TokenPosition {
                row: self.row,
                column: self.column,
            },
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
            '/' => {
                if self.next_is('/') {
                    while self.peek() != '\n' && self.current < self.source.len() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash);
                }
            }
            '"' => self.string()?,
            '0'..='9' => self.number()?,
            'a'..='z' | 'A'..='Z' | '_' => self.identifier(),
            '\r' | '\t' | ' ' => {}
            '\n' | '\0' => {
                self.row += 1;
                self.column = 0;
            }
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

    fn peek(&self) -> char {
        if self.current >= self.source.len() {
            return '\0';
        }

        self.source.as_bytes()[self.current] as char
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }

        self.source.as_bytes()[self.current + 1] as char
    }

    fn string(&mut self) -> anyhow::Result<()> {
        while self.peek() != '"' && self.current < self.source.len() {
            if self.peek() == '\n' {
                self.row += 1;
                self.column = 0;
            }
            self.advance();
        }

        if self.current >= self.source.len() {
            return Err(ScannerError {
                message: String::from("Unterminated string."),
                lexeme: self.source[self.start..self.current].to_string(),
                position: TokenPosition {
                    row: self.row,
                    column: self.column,
                },
            }
            .into());
        }

        self.advance();

        self.add_token(TokenType::String(
            // Remove the '"' from the string
            self.source[self.start + 1..self.current - 1].to_string(),
        ));
        Ok(())
    }

    fn number(&mut self) -> anyhow::Result<()> {
        while self.peek().is_numeric() {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_numeric() {
            self.advance();

            while self.peek().is_numeric() {
                self.advance();
            }
        }

        let value = match self.source[self.start..self.current].parse::<f64>() {
            Ok(value) => value,
            Err(e) => {
                return Err(ScannerError {
                    message: e.to_string(),
                    lexeme: self.source[self.start..self.current].to_string(),
                    position: TokenPosition {
                        row: self.row,
                        column: self.column,
                    },
                }
                .into());
            }
        };
        self.add_token(TokenType::Number(value));
        Ok(())
    }

    fn identifier(&mut self) {
        while self.peek().is_alphanumeric() {
            self.advance();
        }

        let identifier = match self.keywords.get(&self.source[self.start..self.current]) {
            Some(identifier) => identifier.to_owned(),
            None => TokenType::Identifier,
        };
        self.add_token(identifier);
    }

    fn add_token(&mut self, token_type: TokenType) {
        const COLUMN_OFFSET: usize = 1;

        let lexeme = self.source[self.start..self.current].to_string();
        let column = self.column - lexeme.len() + COLUMN_OFFSET;

        self.tokens.push(Token {
            kind: token_type,
            lexeme,
            position: TokenPosition {
                row: self.row,
                column,
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
            "[{}:{}] Error: {}\n{}",
            self.position.row, self.position.column, self.message, self.lexeme
        )
    }
}

impl Error for ScannerError {}
