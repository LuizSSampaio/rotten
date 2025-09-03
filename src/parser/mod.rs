use anyhow::Result;

use crate::{
    parser::{
        error::{ParserError, ParserErrorMessage},
        node::{expression::Expression, statement::Statement},
    },
    token::{Token, kind::TokenType, value::TokenValue},
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

    pub fn parse(&mut self) -> Vec<Statement> {
        let mut statements = Vec::new();

        while !self.is_at_end() {
            let statement = match self.declaration() {
                Ok(stmt) => stmt,
                Err(e) => {
                    println!("{}", e);
                    log::error!("{}", e);
                    self.syncronize();
                    continue;
                }
            };
            statements.push(statement);
        }

        statements
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

    fn consume(&mut self, kind: TokenType) -> Result<Token> {
        if !self.check(&kind) {
            return Err(ParserError {
                message: ParserErrorMessage::ExpectToken(kind),
                token: Some(self.peek()?),
            }
            .into());
        }

        self.advance()
    }

    fn expression(&mut self) -> Result<Expression> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expression> {
        let expr = self.or()?;

        if self.match_tokens(&[TokenType::Equal]) {
            let equals = self.previous()?;
            let value = self.assignment()?;

            match expr {
                Expression::Variable { name } => {
                    return Ok(Expression::Assign {
                        name,
                        value: Box::new(value),
                    });
                }
                _ => {
                    return Err(ParserError {
                        message: ParserErrorMessage::InvalidAssignment,
                        token: Some(equals),
                    }
                    .into());
                }
            }
        }

        Ok(expr)
    }

    fn or(&mut self) -> Result<Expression> {
        let mut expr = self.and()?;

        while self.match_tokens(&[TokenType::Or]) {
            let operator = self.previous()?;
            let right = self.and()?;
            expr = Expression::Logical {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            }
        }

        Ok(expr)
    }

    fn and(&mut self) -> Result<Expression> {
        let mut expr = self.equality()?;

        while self.match_tokens(&[TokenType::And]) {
            let operator = self.previous()?;
            let right = self.equality()?;
            expr = Expression::Logical {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            }
        }

        Ok(expr)
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

        self.call()
    }

    fn call(&mut self) -> Result<Expression> {
        let mut expr = self.primary()?;

        loop {
            if self.match_tokens(&[TokenType::LeftParen]) {
                let mut arguments = Vec::new();
                if !self.check(&TokenType::RightParen) {
                    loop {
                        arguments.push(self.expression()?);
                        if !self.match_tokens(&[TokenType::Comma]) {
                            break;
                        }
                    }
                }

                let paren = self.consume(TokenType::RightParen)?;

                expr = Expression::Call {
                    callee: Box::new(expr),
                    paren,
                    arguments,
                }
            } else {
                break;
            }
        }

        Ok(expr)
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

        if self.match_tokens(&[TokenType::Identifier]) {
            return Ok(Expression::Variable {
                name: self.previous()?,
            });
        }

        if self.match_tokens(&[TokenType::LeftParen]) {
            let expr = self.expression()?;
            self.consume(TokenType::RightParen)?;

            return Ok(Expression::Grouping {
                expression: Box::new(expr),
            });
        }

        Err(ParserError {
            message: ParserErrorMessage::UnexpectedTokenType,
            token: Some(self.peek()?),
        }
        .into())
    }

    fn declaration(&mut self) -> Result<Statement> {
        if self.match_tokens(&[TokenType::Fun]) {
            return self.function_declaration();
        }

        if self.match_tokens(&[TokenType::Var]) {
            return self.var_declaration();
        }

        self.statement()
    }

    fn function_declaration(&mut self) -> Result<Statement> {
        let name = self.consume(TokenType::Identifier)?;

        self.consume(TokenType::LeftParen)?;
        let mut parameters = Vec::new();
        if !self.check(&TokenType::RightParen) {
            loop {
                parameters.push(self.consume(TokenType::Identifier)?);

                if !self.match_tokens(&[TokenType::Comma]) {
                    break;
                }
            }
        }
        self.consume(TokenType::RightParen)?;

        self.consume(TokenType::LeftBrace)?;
        let body = self.block_statement()?;

        Ok(Statement::Function {
            name,
            params: parameters,
            body: Box::new(body),
        })
    }

    fn var_declaration(&mut self) -> Result<Statement> {
        let name = self.consume(TokenType::Identifier)?;

        let mut initializer = None;
        if self.match_tokens(&[TokenType::Equal]) {
            initializer = Some(Box::new(self.expression()?));
        }

        self.consume(TokenType::Semicolon)?;
        Ok(Statement::Var { name, initializer })
    }

    fn statement(&mut self) -> Result<Statement> {
        if self.match_tokens(&[TokenType::While]) {
            return self.while_statement();
        }

        if self.match_tokens(&[TokenType::For]) {
            return self.for_statement();
        }

        if self.match_tokens(&[TokenType::LeftBrace]) {
            return self.block_statement();
        }

        if self.match_tokens(&[TokenType::If]) {
            return self.if_statement();
        }

        self.expression_statement()
    }

    fn while_statement(&mut self) -> Result<Statement> {
        self.consume(TokenType::LeftParen)?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen)?;
        let body = self.statement()?;

        Ok(Statement::While {
            condition: Box::new(condition),
            body: Box::new(body),
        })
    }

    fn for_statement(&mut self) -> Result<Statement> {
        self.consume(TokenType::LeftParen)?;

        let initializer = if self.match_tokens(&[TokenType::Semicolon]) {
            None
        } else if self.match_tokens(&[TokenType::Var]) {
            Some(self.var_declaration()?)
        } else {
            Some(self.expression_statement()?)
        };

        let condition = match !self.check(&TokenType::Semicolon) {
            true => Box::new(self.expression()?),
            false => Box::new(Expression::Literal {
                value: TokenValue::Bool(true),
            }),
        };
        self.consume(TokenType::Semicolon)?;

        let increment = match !self.check(&TokenType::RightParen) {
            true => Some(Box::new(self.expression()?)),
            false => None,
        };
        self.consume(TokenType::RightParen)?;

        let mut body = self.statement()?;

        if let Some(increment) = increment {
            body = Statement::Block {
                statements: vec![
                    body,
                    Statement::Expression {
                        expression: increment,
                    },
                ],
            }
        }

        body = Statement::While {
            condition,
            body: Box::new(body),
        };

        if let Some(initializer) = initializer {
            body = Statement::Block {
                statements: vec![initializer, body],
            }
        }

        Ok(body)
    }

    fn block_statement(&mut self) -> Result<Statement> {
        let mut statements = Vec::new();

        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            statements.push(self.declaration()?);
        }

        self.consume(TokenType::RightBrace)?;
        Ok(Statement::Block { statements })
    }

    fn if_statement(&mut self) -> Result<Statement> {
        self.consume(TokenType::LeftParen)?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen)?;

        let then_branch = self.statement()?;
        let mut else_branch = None;
        if self.match_tokens(&[TokenType::Else]) {
            else_branch = Some(Box::new(self.statement()?));
        }

        Ok(Statement::If {
            condition: Box::new(condition),
            then_branch: Box::new(then_branch),
            else_branch,
        })
    }

    fn expression_statement(&mut self) -> Result<Statement> {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon)?;

        Ok(Statement::Expression {
            expression: Box::new(expr),
        })
    }
}
