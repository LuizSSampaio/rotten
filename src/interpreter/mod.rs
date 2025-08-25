use crate::{
    interpreter::error::{InterpreterError, InterpreterErrorMessage},
    parser::node::{Expression, ExpressionVisitor, expression},
    token::{Token, TokenType, TokenValue},
};

use anyhow::Result;

mod error;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Interpreter {}

impl Interpreter {
    fn evaluate(&mut self, expr: &mut Expression) -> Result<TokenValue> {
        expr.accept::<Result<TokenValue>>(self)
    }

    fn is_truthy(&self, value: TokenValue) -> bool {
        match value {
            TokenValue::Bool(val) => val,
            TokenValue::Number(val) => val != 0.0,
            TokenValue::String(content) => !content.is_empty(),
            TokenValue::Nil => false,
        }
    }

    fn expect_number(&self, value: TokenValue, token: &Token) -> Result<f64> {
        match value {
            TokenValue::Number(n) => Ok(n),
            other => Err(InterpreterError {
                message: InterpreterErrorMessage::UnexpectedValue {
                    is: other,
                    expect: TokenValue::Number(0.0),
                },
                token: Some(token.clone()),
            }
            .into()),
        }
    }
}

impl ExpressionVisitor<Result<TokenValue>> for Interpreter {
    fn visit_assign(
        &mut self,
        expr: &mut expression::assign::AssignExpression,
    ) -> Result<TokenValue> {
        todo!()
    }

    fn visit_binary(
        &mut self,
        expr: &mut expression::binary::BinaryExpression,
    ) -> Result<TokenValue> {
        let left = self.evaluate(&mut expr.left)?;
        let right = self.evaluate(&mut expr.right)?;

        let left = self.expect_number(left, &expr.operator)?;
        let right = self.expect_number(right, &expr.operator)?;

        match expr.operator.kind {
            TokenType::Minus => {
                return Ok(TokenValue::Number(left - right));
            }
            TokenType::Slash => {
                return Ok(TokenValue::Number(left / right));
            }
            TokenType::Star => {
                return Ok(TokenValue::Number(left * right));
            }
            _ => {}
        }

        Err(InterpreterError {
            message: InterpreterErrorMessage::Unreachable,
            token: Some(expr.operator.to_owned()),
        }
        .into())
    }

    fn visit_call(&mut self, expr: &mut expression::call::CallExpression) -> Result<TokenValue> {
        todo!()
    }

    fn visit_get(&mut self, expr: &mut expression::get::GetExpression) -> Result<TokenValue> {
        todo!()
    }

    fn visit_grouping(
        &mut self,
        expr: &mut expression::grouping::GroupingExpression,
    ) -> Result<TokenValue> {
        self.evaluate(&mut expr.expression)
    }

    fn visit_literal(
        &mut self,
        expr: &mut expression::literal::LiteralExpression,
    ) -> Result<TokenValue> {
        Ok(expr.value.to_owned())
    }

    fn visit_logical(
        &mut self,
        expr: &mut expression::logical::LogicalExpression,
    ) -> Result<TokenValue> {
        todo!()
    }

    fn visit_set(&mut self, expr: &mut expression::set::SetExpression) -> Result<TokenValue> {
        todo!()
    }

    fn visit_super(
        &mut self,
        expr: &mut expression::super_class::SuperExpression,
    ) -> Result<TokenValue> {
        todo!()
    }

    fn visit_this(&mut self, expr: &mut expression::this::ThisExpression) -> Result<TokenValue> {
        todo!()
    }

    fn visit_unary(&mut self, expr: &mut expression::unary::UnaryExpression) -> Result<TokenValue> {
        let right = self.evaluate(&mut expr.right)?;

        match expr.operator.kind {
            TokenType::Bang => return Ok(TokenValue::Bool(!self.is_truthy(right))),
            TokenType::Minus => {
                if let TokenValue::Number(num) = right {
                    return Ok(TokenValue::Number(-num));
                }
            }
            _ => {}
        }

        Err(InterpreterError {
            message: InterpreterErrorMessage::Unreachable,
            token: Some(expr.operator.to_owned()),
        }
        .into())
    }

    fn visit_variable(
        &mut self,
        expr: &mut expression::variable::VariableExpression,
    ) -> Result<TokenValue> {
        todo!()
    }
}
