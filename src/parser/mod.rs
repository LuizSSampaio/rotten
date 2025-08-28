use anyhow::Result;

use crate::{
    parser::{
        error::{ParserError, ParserErrorMessage},
        node::expression::Expression,
    },
    token::{Token, TokenType, value::TokenValue},
};

mod error;
pub mod node;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Expression> {
        self.expression()
    }

    fn match_tokens(&mut self, kinds: &[TokenType]) -> bool {
        for kind in kinds {
            if self.check(kind) {
                let _ = self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, kind: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

        match self.peek() {
            Ok(token) => &token.kind == kind,
            Err(_) => false,
        }
    }

    fn is_at_end(&self) -> bool {
        match self.peek() {
            Ok(token) => token.kind == TokenType::EndOfFile,
            Err(_) => true,
        }
    }

    fn peek(&self) -> Result<Token> {
        match self.tokens.get(self.current).cloned() {
            Some(token) => Ok(token),
            None => Err(ParserError {
                message: ParserErrorMessage::GetTokenError,
                token: None,
            }
            .into()),
        }
    }

    fn advance(&mut self) -> Result<Token> {
        if !self.is_at_end() {
            self.current += 1;
        }

        self.previous()
    }

    fn previous(&self) -> Result<Token> {
        match self.tokens.get(self.current - 1).cloned() {
            Some(token) => Ok(token),
            None => Err(ParserError {
                message: ParserErrorMessage::GetTokenError,
                token: None,
            }
            .into()),
        }
    }

    fn syncronize(&mut self) {
        let _ = self.advance();

        while !self.is_at_end() {
            match self.previous() {
                Ok(token) => {
                    if token.kind == TokenType::Semicolon {
                        return;
                    }
                }
                Err(_) => continue,
            }

            let Ok(token) = self.peek() else { continue };
            match token.kind {
                TokenType::Class
                | TokenType::Fun
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Return => {
                    return;
                }
                _ => {}
            }
        }

        let _ = self.advance();
    }

    fn consume(&mut self, kind: TokenType, message: ParserErrorMessage) -> Result<()> {
        if !self.check(&kind) {
            return Err(ParserError {
                message,
                token: Some(self.peek()?),
            }
            .into());
        }

        Ok(())
    }

    fn expression(&mut self) -> Result<Expression> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expression> {
        let mut expr = self.comparison()?;

        while self.match_tokens(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous()?;
            let right = self.comparison()?;

            expr = Expression::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expression> {
        let mut expr = self.term()?;

        while self.match_tokens(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous()?;
            let right = self.term()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expression> {
        let mut expr = self.factor()?;

        while self.match_tokens(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous()?;
            let right = self.factor()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expression> {
        let mut expr = self.unary()?;

        while self.match_tokens(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous()?;
            let right = self.unary()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expression> {
        if self.match_tokens(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous()?;
            let right = self.unary()?;
            return Ok(Expression::Unary {
                operator,
                right: Box::new(right),
            });
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<Expression> {
        if self.match_tokens(&[TokenType::False]) {
            return Ok(Expression::Literal {
                value: TokenValue::Bool(false),
            });
        }
        if self.match_tokens(&[TokenType::True]) {
            return Ok(Expression::Literal {
                value: TokenValue::Bool(true),
            });
        }
        if self.match_tokens(&[TokenType::Nil]) {
            return Ok(Expression::Literal {
                value: TokenValue::Nil,
            });
        }

        if self.match_tokens(&[TokenType::Number, TokenType::String]) {
            let previous = self.previous()?;
            let value = match previous.value {
                Some(value) => value,
                None => {
                    return Err(ParserError {
                        message: ParserErrorMessage::LiteralTokenWithoutValue,
                        token: Some(previous),
                    }
                    .into());
                }
            };
            return Ok(Expression::Literal { value });
        }

        if self.match_tokens(&[TokenType::LeftParen]) {
            let expr = self.expression()?;
            self.consume(
                TokenType::RightParen,
                ParserErrorMessage::ExpectRightParenthesis,
            );

            self.advance()?;
            return Ok(Expression::Grouping {
                expression: Box::new(expr),
            });
        }

        Err(ParserError {
            message: ParserErrorMessage::UnexpectedTokenType,
            token: Some(self.previous()?),
        }
        .into())
    }
}
