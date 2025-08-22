use std::collections::HashMap;

use anyhow::Result;

use crate::lexer::emitter::Emitter;
use crate::lexer::error::{LexerError, LexerErrorMessage};
use crate::lexer::keywords;
use crate::lexer::reader::Reader;
use crate::lexer::token::{Token, TokenPosition, TokenType, TokenValue};

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
            None,
            String::new(),
            self.reader.row(),
            self.reader.column(),
        );

        Ok(self.emitter.tokens())
    }

    fn scan_token(&mut self) -> Result<()> {
        match self.reader.advance()? {
            '(' => self.add_token(TokenType::LeftParen, None),
            ')' => self.add_token(TokenType::RightParen, None),
            '{' => self.add_token(TokenType::LeftBrace, None),
            '}' => self.add_token(TokenType::RightBrace, None),
            ',' => self.add_token(TokenType::Comma, None),
            '.' => self.add_token(TokenType::Dot, None),
            '-' => self.add_token(TokenType::Minus, None),
            '+' => self.add_token(TokenType::Plus, None),
            ';' => self.add_token(TokenType::Semicolon, None),
            '*' => self.add_token(TokenType::Star, None),
            '!' => {
                if self.reader.next_is('=') {
                    self.add_token(TokenType::BangEqual, None);
                } else {
                    self.add_token(TokenType::Bang, None);
                }
            }
            '=' => {
                if self.reader.next_is('=') {
                    self.add_token(TokenType::EqualEqual, None);
                } else {
                    self.add_token(TokenType::Equal, None);
                }
            }
            '<' => {
                if self.reader.next_is('=') {
                    self.add_token(TokenType::LessEqual, None);
                } else {
                    self.add_token(TokenType::Less, None);
                }
            }
            '>' => {
                if self.reader.next_is('=') {
                    self.add_token(TokenType::GreaterEqual, None);
                } else {
                    self.add_token(TokenType::Greater, None);
                }
            }
            '/' => {
                if self.reader.next_is('/') {
                    while self.reader.peek() != '\n' && !self.reader.is_at_end() {
                        self.reader.advance()?;
                    }
                } else if self.reader.next_is('*') {
                    while self.reader.peek() != '*' && self.reader.peek_next() != '/' {
                        if self.reader.is_at_end() {
                            break;
                        }
                        self.reader.advance()?;
                    }
                    if !self.reader.is_at_end() {
                        self.reader.advance()?; // *
                        self.reader.advance()?; // /
                    }
                } else {
                    self.add_token(TokenType::Slash, None);
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
                    message: LexerErrorMessage::UnexpectedCharacter,
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
                message: LexerErrorMessage::UnterminatedString,
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
        self.add_token(TokenType::String, Some(TokenValue::String(value)));
        Ok(())
    }

    fn number(&mut self) -> Result<()> {
        while self.reader.peek().is_ascii_digit() {
            self.reader.advance()?;
        }

        if self.reader.peek() == '.' && self.reader.peek_next().is_ascii_digit() {
            self.reader.advance()?;

            while self.reader.peek().is_ascii_digit() {
                self.reader.advance()?;
            }
        }

        let lexeme = self.reader.current_lexeme();
        let value = lexeme.parse::<f64>().map_err(|_| LexerError {
            message: LexerErrorMessage::NumberParseError,
            lexeme: lexeme.clone(),
            position: TokenPosition {
                row: self.reader.row(),
                column: self.reader.column(),
            },
        })?;
        self.add_token(TokenType::Number, Some(TokenValue::Number(value)));
        Ok(())
    }

    fn identifier(&mut self) -> Result<()> {
        while self.reader.peek().is_ascii_alphanumeric() {
            self.reader.advance()?;
        }

        let slice = self.reader.current_lexeme();
        let token_type = self
            .keywords
            .get(&*slice)
            .cloned()
            .unwrap_or(TokenType::Identifier);
        self.add_token(token_type, None);
        Ok(())
    }

    fn add_token(&mut self, token_type: TokenType, token_value: Option<TokenValue>) {
        let lexeme = self.reader.current_lexeme();
        let column = self.reader.calculate_column(lexeme.len());
        self.emitter
            .add_token(token_type, token_value, lexeme, self.reader.row(), column);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::token::{Token, TokenPosition, TokenType};

    fn pos(row: usize, col: usize) -> TokenPosition {
        TokenPosition { row, column: col }
    }

    fn token(
        kind: TokenType,
        value: Option<TokenValue>,
        lexeme: &str,
        pos: TokenPosition,
    ) -> Token {
        Token {
            kind,
            value,
            lexeme: lexeme.to_string(),
            position: pos,
        }
    }

    fn error(message: LexerErrorMessage, lexeme: &str, pos: TokenPosition) -> LexerError {
        LexerError {
            message,
            lexeme: lexeme.to_string(),
            position: pos,
        }
    }

    fn scan(input: &str) -> Result<Vec<Token>> {
        let mut scanner = Scanner::new(input.to_string());
        scanner.scan_tokens()
    }

    #[test]
    fn simple_arithmetic() {
        let res = scan("1 + 2;").unwrap();
        let expected = vec![
            token(
                TokenType::Number,
                Some(TokenValue::Number(1.0)),
                "1",
                pos(1, 1),
            ),
            token(TokenType::Plus, None, "+", pos(1, 3)),
            token(
                TokenType::Number,
                Some(TokenValue::Number(2.0)),
                "2",
                pos(1, 5),
            ),
            token(TokenType::Semicolon, None, ";", pos(1, 6)),
            token(TokenType::EndOfFile, None, "", pos(1, 7)),
        ];
        assert_eq!(res, expected);
    }

    #[test]
    fn keywords_and_identifiers() {
        let res = scan("var x = 5.42;").unwrap();
        let expected = vec![
            token(TokenType::Var, None, "var", pos(1, 1)),
            token(TokenType::Identifier, None, "x", pos(1, 5)),
            token(TokenType::Equal, None, "=", pos(1, 7)),
            token(
                TokenType::Number,
                Some(TokenValue::Number(5.42)),
                "5.42",
                pos(1, 9),
            ),
            token(TokenType::Semicolon, None, ";", pos(1, 13)),
            token(TokenType::EndOfFile, None, "", pos(1, 14)),
        ];
        assert_eq!(res, expected);
    }

    #[test]
    fn string() {
        let res = scan("\"hello\"").unwrap();
        let expected = vec![
            token(
                TokenType::String,
                Some(TokenValue::String("hello".to_string())),
                "\"hello\"",
                pos(1, 1),
            ),
            token(TokenType::EndOfFile, None, "", pos(1, 8)),
        ];
        assert_eq!(res, expected);
    }

    #[test]
    fn line_comment() {
        let res = scan("// comment\nvar y = 10;").unwrap();
        let expected = vec![
            token(TokenType::Var, None, "var", pos(2, 1)),
            token(TokenType::Identifier, None, "y", pos(2, 5)),
            token(TokenType::Equal, None, "=", pos(2, 7)),
            token(
                TokenType::Number,
                Some(TokenValue::Number(10.0)),
                "10",
                pos(2, 9),
            ),
            token(TokenType::Semicolon, None, ";", pos(2, 11)),
            token(TokenType::EndOfFile, None, "", pos(2, 12)),
        ];
        assert_eq!(res, expected);
    }

    #[test]
    fn block_comment() {
        let res = scan("/* comment */ var z = 1;").unwrap();
        let expected = vec![
            token(TokenType::Var, None, "var", pos(1, 15)),
            token(TokenType::Identifier, None, "z", pos(1, 19)),
            token(TokenType::Equal, None, "=", pos(1, 21)),
            token(
                TokenType::Number,
                Some(TokenValue::Number(1.0)),
                "1",
                pos(1, 23),
            ),
            token(TokenType::Semicolon, None, ";", pos(1, 24)),
            token(TokenType::EndOfFile, None, "", pos(1, 25)),
        ];
        assert_eq!(res, expected);
    }

    #[test]
    fn multi_char_operators() {
        let res = scan("a != b == c").unwrap();
        let expected = vec![
            token(TokenType::Identifier, None, "a", pos(1, 1)),
            token(TokenType::BangEqual, None, "!=", pos(1, 3)),
            token(TokenType::Identifier, None, "b", pos(1, 6)),
            token(TokenType::EqualEqual, None, "==", pos(1, 8)),
            token(TokenType::Identifier, None, "c", pos(1, 11)),
            token(TokenType::EndOfFile, None, "", pos(1, 12)),
        ];
        assert_eq!(res, expected);
    }

    #[test]
    fn less_equal() {
        let res = scan("1 <= 2").unwrap();
        let expected = vec![
            token(
                TokenType::Number,
                Some(TokenValue::Number(1.0)),
                "1",
                pos(1, 1),
            ),
            token(TokenType::LessEqual, None, "<=", pos(1, 3)),
            token(
                TokenType::Number,
                Some(TokenValue::Number(2.0)),
                "2",
                pos(1, 6),
            ),
            token(TokenType::EndOfFile, None, "", pos(1, 7)),
        ];
        assert_eq!(res, expected);
    }

    #[test]
    fn greater_equal() {
        let res = scan("1 >= 2").unwrap();
        let expected = vec![
            token(
                TokenType::Number,
                Some(TokenValue::Number(1.0)),
                "1",
                pos(1, 1),
            ),
            token(TokenType::GreaterEqual, None, ">=", pos(1, 3)),
            token(
                TokenType::Number,
                Some(TokenValue::Number(2.0)),
                "2",
                pos(1, 6),
            ),
            token(TokenType::EndOfFile, None, "", pos(1, 7)),
        ];
        assert_eq!(res, expected);
    }

    #[test]
    fn unexpected_character() {
        let res = scan("@");
        assert!(res.is_err());
        let err = res.unwrap_err();
        assert_eq!(
            err.to_string(),
            error(LexerErrorMessage::UnexpectedCharacter, "@", pos(1, 2)).to_string()
        );
    }

    #[test]
    fn unterminated_string() {
        let res = scan("\"unterminated");
        assert!(res.is_err());
        let err = res.unwrap_err();
        assert_eq!(
            err.to_string(),
            error(
                LexerErrorMessage::UnterminatedString,
                "\"unterminated",
                pos(1, 14)
            )
            .to_string()
        );
    }

    #[test]
    fn empty_input() {
        let res = scan("").unwrap();
        let expected = vec![token(TokenType::EndOfFile, None, "", pos(1, 1))];
        assert_eq!(res, expected);
    }

    #[test]
    fn whitespace_only() {
        let res = scan(" \t\n ").unwrap();
        let expected = vec![token(TokenType::EndOfFile, None, "", pos(2, 2))];
        assert_eq!(res, expected);
    }

    #[test]
    fn multiline_string() {
        let res = scan("\"line1\\nline2\"").unwrap();
        let expected = vec![
            token(
                TokenType::String,
                Some(TokenValue::String("line1\\nline2".to_string())),
                "\"line1\\nline2\"",
                pos(1, 1),
            ),
            token(TokenType::EndOfFile, None, "", pos(1, 15)),
        ];
        assert_eq!(res, expected);
    }

    #[test]
    fn unterminated_block_comment() {
        let res = scan("/* unterminated").unwrap();
        let expected = vec![token(TokenType::EndOfFile, None, "", pos(1, 16))];
        assert_eq!(res, expected);
    }

    #[test]
    fn unicode_identifier() {
        let res = scan("π = 3.14;");
        assert!(res.is_err());
        let err = res.unwrap_err();
        assert_eq!(
            err.to_string(),
            error(LexerErrorMessage::UnexpectedCharacter, "π", pos(1, 2)).to_string()
        );
    }
}
