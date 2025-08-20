use std::collections::HashMap;

use anyhow::Result;

use crate::lexer::emitter::Emitter;
use crate::lexer::error::{LexerError, LexerErrorMessage};
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
        let value = lexeme.parse::<f64>().map_err(|_| LexerError {
            message: LexerErrorMessage::NumberParseError,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::token::{Token, TokenPosition, TokenType};

    fn pos(row: usize, col: usize) -> TokenPosition {
        TokenPosition { row, column: col }
    }

    fn token(kind: TokenType, lexeme: &str, pos: TokenPosition) -> Token {
        Token {
            kind,
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

    #[test]
    fn simple_arithmetic() {
        let input = "1 + 2;".to_string();
        let mut scanner = Scanner::new(input);
        let res = scanner.scan_tokens().unwrap();
        let expected = vec![
            token(TokenType::Number(1.0), "1", pos(1, 1)),
            token(TokenType::Plus, "+", pos(1, 3)),
            token(TokenType::Number(2.0), "2", pos(1, 5)),
            token(TokenType::Semicolon, ";", pos(1, 6)),
            token(TokenType::EndOfFile, "", pos(1, 7)),
        ];
        assert_eq!(res, expected);
    }

    #[test]
    fn keywords_and_identifiers() {
        let input = "var x = 5.42;".to_string();
        let mut scanner = Scanner::new(input);
        let res = scanner.scan_tokens().unwrap();
        let expected = vec![
            token(TokenType::Var, "var", pos(1, 1)),
            token(TokenType::Identifier, "x", pos(1, 5)),
            token(TokenType::Equal, "=", pos(1, 7)),
            token(TokenType::Number(5.42), "5.42", pos(1, 9)),
            token(TokenType::Semicolon, ";", pos(1, 13)),
            token(TokenType::EndOfFile, "", pos(1, 14)),
        ];
        assert_eq!(res, expected);
    }

    #[test]
    fn string() {
        let input = "\"hello\"".to_string();
        let mut scanner = Scanner::new(input);
        let res = scanner.scan_tokens().unwrap();
        let expected = vec![
            token(
                TokenType::String("hello".to_string()),
                "\"hello\"",
                pos(1, 1),
            ),
            token(TokenType::EndOfFile, "", pos(1, 8)),
        ];
        assert_eq!(res, expected);
    }

    #[test]
    fn line_comment() {
        let input = "// comment\nvar y = 10;".to_string();
        let mut scanner = Scanner::new(input);
        let res = scanner.scan_tokens().unwrap();
        let expected = vec![
            token(TokenType::Var, "var", pos(2, 1)),
            token(TokenType::Identifier, "y", pos(2, 5)),
            token(TokenType::Equal, "=", pos(2, 7)),
            token(TokenType::Number(10.0), "10", pos(2, 9)),
            token(TokenType::Semicolon, ";", pos(2, 11)),
            token(TokenType::EndOfFile, "", pos(2, 12)),
        ];
        assert_eq!(res, expected);
    }

    #[test]
    fn block_comment() {
        let input = "/* comment */ var z = 1;".to_string();
        let mut scanner = Scanner::new(input);
        let res = scanner.scan_tokens().unwrap();
        let expected = vec![
            token(TokenType::Var, "var", pos(1, 15)),
            token(TokenType::Identifier, "z", pos(1, 19)),
            token(TokenType::Equal, "=", pos(1, 21)),
            token(TokenType::Number(1.0), "1", pos(1, 23)),
            token(TokenType::Semicolon, ";", pos(1, 24)),
            token(TokenType::EndOfFile, "", pos(1, 25)),
        ];
        assert_eq!(res, expected);
    }

    #[test]
    fn multi_char_operators() {
        let input = "a != b == c".to_string();
        let mut scanner = Scanner::new(input);
        let res = scanner.scan_tokens().unwrap();
        let expected = vec![
            token(TokenType::Identifier, "a", pos(1, 1)),
            token(TokenType::BangEqual, "!=", pos(1, 3)),
            token(TokenType::Identifier, "b", pos(1, 6)),
            token(TokenType::EqualEqual, "==", pos(1, 8)),
            token(TokenType::Identifier, "c", pos(1, 11)),
            token(TokenType::EndOfFile, "", pos(1, 12)),
        ];
        assert_eq!(res, expected);
    }

    #[test]
    fn less_equal() {
        let input = "1 <= 2".to_string();
        let mut scanner = Scanner::new(input);
        let res = scanner.scan_tokens().unwrap();
        let expected = vec![
            token(TokenType::Number(1.0), "1", pos(1, 1)),
            token(TokenType::LessEqual, "<=", pos(1, 3)),
            token(TokenType::Number(2.0), "2", pos(1, 6)),
            token(TokenType::EndOfFile, "", pos(1, 7)),
        ];
        assert_eq!(res, expected);
    }

    #[test]
    fn greater_equal() {
        let input = "1 >= 2".to_string();
        let mut scanner = Scanner::new(input);
        let res = scanner.scan_tokens().unwrap();
        let expected = vec![
            token(TokenType::Number(1.0), "1", pos(1, 1)),
            token(TokenType::GreaterEqual, ">=", pos(1, 3)),
            token(TokenType::Number(2.0), "2", pos(1, 6)),
            token(TokenType::EndOfFile, "", pos(1, 7)),
        ];
        assert_eq!(res, expected);
    }

    #[test]
    fn unexpected_character() {
        let input = "@".to_string();
        let mut scanner = Scanner::new(input);
        let res = scanner.scan_tokens();
        assert!(res.is_err());
        let err = res.unwrap_err();
        assert_eq!(
            err.to_string(),
            error(LexerErrorMessage::UnexpectedCharacter, "@", pos(1, 2)).to_string()
        );
    }

    #[test]
    fn unterminated_string() {
        let input = "\"unterminated".to_string();
        let mut scanner = Scanner::new(input);
        let res = scanner.scan_tokens();
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
        let input = "".to_string();
        let mut scanner = Scanner::new(input);
        let res = scanner.scan_tokens().unwrap();
        let expected = vec![token(TokenType::EndOfFile, "", pos(1, 1))];
        assert_eq!(res, expected);
    }

    #[test]
    fn whitespace_only() {
        let input = " \t\n ".to_string();
        let mut scanner = Scanner::new(input);
        let res = scanner.scan_tokens().unwrap();
        let expected = vec![token(TokenType::EndOfFile, "", pos(2, 2))];
        assert_eq!(res, expected);
    }

    #[test]
    fn multiline_string() {
        let input = "\"line1\\nline2\"".to_string();
        let mut scanner = Scanner::new(input);
        let res = scanner.scan_tokens().unwrap();
        let expected = vec![
            token(
                TokenType::String("line1\\nline2".to_string()),
                "\"line1\\nline2\"",
                pos(1, 1),
            ),
            token(TokenType::EndOfFile, "", pos(1, 15)),
        ];
        assert_eq!(res, expected);
    }

    #[test]
    fn unterminated_block_comment() {
        let input = "/* unterminated".to_string();
        let mut scanner = Scanner::new(input);
        let res = scanner.scan_tokens().unwrap();
        let expected = vec![token(TokenType::EndOfFile, "", pos(1, 16))];
        assert_eq!(res, expected);
    }

    #[test]
    fn unicode_identifier() {
        let input = "π = 3.14;".to_string();
        let mut scanner = Scanner::new(input);
        let res = scanner.scan_tokens();
        assert!(res.is_err());
        let err = res.unwrap_err();
        assert_eq!(
            err.to_string(),
            error(LexerErrorMessage::UnexpectedCharacter, "π", pos(1, 2)).to_string()
        );
    }
}
