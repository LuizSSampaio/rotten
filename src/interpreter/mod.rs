use crate::{
    interpreter::error::{InterpreterError, InterpreterErrorMessage},
    parser::node::{Expression, ExpressionVisitor},
    token::{Token, TokenType, value::TokenValue},
};

use anyhow::Result;

mod error;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Interpreter {}

impl Interpreter {
    fn evaluate(&mut self, expr: &mut Expression) -> Result<TokenValue> {
        expr.accept::<Result<TokenValue>>(self)
    }

    fn as_number(&self, value: TokenValue, token: &Token) -> Result<f64> {
        value.clone().try_into().map_err(|_| {
            InterpreterError {
                message: InterpreterErrorMessage::UnexpectedValue {
                    is: value,
                    expect: TokenValue::Number(0.0),
                },
                token: Some(token.clone()),
            }
            .into()
        })
    }
}

impl ExpressionVisitor<Result<TokenValue>> for Interpreter {
    fn visit_assign(&mut self, name: &Token, value: &mut Expression) -> Result<TokenValue> {
        todo!()
    }

    fn visit_binary(
        &mut self,
        left: &mut Expression,
        operator: &Token,
        right: &mut Expression,
    ) -> Result<TokenValue> {
        let left_val = self.evaluate(left)?;
        let right_val = self.evaluate(right)?;

        match operator.kind {
            TokenType::Plus => {
                if let (TokenValue::String(_), _) | (_, TokenValue::String(_)) =
                    (&left_val, &right_val)
                {
                    return Ok(TokenValue::String(format!("{}{}", left_val, right_val)));
                }
            }
            TokenType::EqualEqual => {
                return Ok(TokenValue::Bool(left_val == right_val));
            }
            TokenType::BangEqual => {
                return Ok(TokenValue::Bool(left_val != right_val));
            }
            _ => {}
        }

        let left_num = self.as_number(left_val, operator)?;
        let right_num = self.as_number(right_val, operator)?;

        match operator.kind {
            TokenType::Plus => {
                return Ok(TokenValue::Number(left_num + right_num));
            }
            TokenType::Minus => {
                return Ok(TokenValue::Number(left_num - right_num));
            }
            TokenType::Slash => {
                return Ok(TokenValue::Number(left_num / right_num));
            }
            TokenType::Star => {
                return Ok(TokenValue::Number(left_num * right_num));
            }
            TokenType::Greater => {
                return Ok(TokenValue::Bool(left_num > right_num));
            }
            TokenType::GreaterEqual => {
                return Ok(TokenValue::Bool(left_num >= right_num));
            }
            TokenType::Less => {
                return Ok(TokenValue::Bool(left_num < right_num));
            }
            TokenType::LessEqual => {
                return Ok(TokenValue::Bool(left_num <= right_num));
            }
            _ => {}
        }

        Err(InterpreterError {
            message: InterpreterErrorMessage::Unreachable,
            token: Some(operator.to_owned()),
        }
        .into())
    }

    fn visit_call(
        &mut self,
        callee: &mut Expression,
        paren: &Token,
        arguments: &mut [Expression],
    ) -> Result<TokenValue> {
        todo!()
    }

    fn visit_get(&mut self, object: &mut Expression, name: &Token) -> Result<TokenValue> {
        todo!()
    }

    fn visit_grouping(&mut self, expression: &mut Expression) -> Result<TokenValue> {
        self.evaluate(expression)
    }

    fn visit_literal(&mut self, value: &TokenValue) -> Result<TokenValue> {
        Ok(value.to_owned())
    }

    fn visit_logical(
        &mut self,
        left: &mut Expression,
        operator: &Token,
        right: &mut Expression,
    ) -> Result<TokenValue> {
        todo!()
    }

    fn visit_set(
        &mut self,
        object: &mut Expression,
        name: &Token,
        value: &mut Expression,
    ) -> Result<TokenValue> {
        todo!()
    }

    fn visit_super(&mut self, keyword: &Token, method: &Token) -> Result<TokenValue> {
        todo!()
    }

    fn visit_this(&mut self, keyword: &Token) -> Result<TokenValue> {
        todo!()
    }

    fn visit_unary(&mut self, operator: &Token, right: &mut Expression) -> Result<TokenValue> {
        let right_val = self.evaluate(right)?;

        match operator.kind {
            TokenType::Bang => {
                let val: bool = right_val.into();
                return Ok(TokenValue::Bool(!val));
            }
            TokenType::Minus => {
                if let TokenValue::Number(num) = right_val {
                    return Ok(TokenValue::Number(-num));
                }
            }
            _ => {}
        }

        Err(InterpreterError {
            message: InterpreterErrorMessage::Unreachable,
            token: Some(operator.to_owned()),
        }
        .into())
    }

    fn visit_variable(&mut self, name: &Token) -> Result<TokenValue> {
        todo!()
    }
}
