use std::collections::HashMap;

use crate::{
    interpreter::error::{InterpreterError, InterpreterErrorMessage, ReturnValue},
    memory::handler::EnvironmentHandler,
    parser::node::{
        Expression, ExpressionVisitor,
        statement::{Statement, StatementVisitor},
    },
    token::{
        Token,
        kind::TokenType,
        value::{
            Class, TokenValue,
            function::{Function, FunctionData},
            instance::Instance,
        },
    },
};

use anyhow::Result;

mod error;

#[derive(Debug, Clone, PartialEq)]
pub struct Interpreter {
    environment: EnvironmentHandler,
}

impl Default for Interpreter {
    fn default() -> Self {
        let mut environment: EnvironmentHandler = Default::default();
        environment
            .define(
                "print".to_string(),
                TokenValue::Function(Function {
                    data: FunctionData {
                        body: None,
                        params: vec!["text".to_string()],
                        this: None,
                    },
                    call: |_, _, args| {
                        println!("{}", args[0]);
                        Ok(TokenValue::Nil)
                    },
                }),
            )
            .unwrap();

        Self { environment }
    }
}

impl Interpreter {
    pub fn interpret(&mut self, statements: &mut Vec<Statement>) -> Result<Option<TokenValue>> {
        let mut last_value = None;

        for statement in statements {
            last_value = statement.accept(self)?;
        }

        Ok(last_value)
    }

    fn evaluate(&mut self, expr: &mut Expression) -> Result<TokenValue> {
        expr.accept::<Result<TokenValue>>(self)
    }

    fn as_number(&self, value: TokenValue, token: &Token) -> Result<f64> {
        value.clone().try_into().map_err(|_| {
            InterpreterError {
                message: InterpreterErrorMessage::UnexpectedValue {
                    is: value,
                    expect: "Number".to_string(),
                },
                token: Some(token.clone()),
            }
            .into()
        })
    }

    fn create_function(
        &mut self,
        name: &Token,
        params: &[Token],
        body: &mut Statement,
    ) -> Result<Function> {
        let body = match body {
            Statement::Block { statements } => statements,
            _ => {
                return Err(InterpreterError {
                    message: InterpreterErrorMessage::MissingBlock,
                    token: Some(name.to_owned()),
                }
                .into());
            }
        };

        Ok(Function {
            data: FunctionData {
                body: Some(body.to_owned()),
                params: params.iter().map(|param| param.lexeme.to_owned()).collect(),
                this: None,
            },
            call: |interpreter, data, args| {
                interpreter.environment.create_environment();

                if let Some(this) = data.this.clone() {
                    interpreter
                        .environment
                        .define("this".to_string(), TokenValue::Instance(this))?;
                }

                for (index, param) in args.iter().enumerate() {
                    interpreter
                        .environment
                        .define(data.params.get(index).unwrap().to_owned(), param.to_owned())?;
                }

                let mut body = match data.body.to_owned() {
                    Some(body) => body,
                    None => {
                        return Err(InterpreterError {
                            message: InterpreterErrorMessage::MissingBlock,
                            token: None,
                        }
                        .into());
                    }
                };
                let val = interpreter
                    .visit_block(&mut body)?
                    .unwrap_or(TokenValue::Nil);
                interpreter.environment.delete_environment()?;
                Ok(val)
            },
        })
    }
}

impl ExpressionVisitor<Result<TokenValue>> for Interpreter {
    fn visit_assign(&mut self, name: &Token, value: &mut Expression) -> Result<TokenValue> {
        let value = self.evaluate(value)?;
        self.environment.assign(name, value.to_owned())?;
        Ok(value)
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
                if right_num == 0.0 {
                    return Err(InterpreterError {
                        message: InterpreterErrorMessage::DivisionByZero,
                        token: Some(operator.to_owned()),
                    }
                    .into());
                }
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
        let callee = self.evaluate(callee)?;

        let mut val_arguments = Vec::new();
        for argument in arguments {
            val_arguments.push(self.evaluate(argument)?);
        }

        match callee {
            TokenValue::Function(mut func) => {
                if func.data.params.len() != val_arguments.len() {
                    return Err(InterpreterError {
                        message: InterpreterErrorMessage::ArgumentMismatch {
                            has: val_arguments.len(),
                            expect: func.data.params.len(),
                        },
                        token: Some(paren.to_owned()),
                    }
                    .into());
                }

                Ok((func.call)(self, &mut func.data, &val_arguments)?)
            }
            TokenValue::Class(class) => Ok(TokenValue::Instance(Instance::new(class.clone()))),
            _ => Err(InterpreterError {
                message: InterpreterErrorMessage::IsNotCallable,
                token: Some(paren.to_owned()),
            }
            .into()),
        }
    }

    fn visit_get(&mut self, object: &mut Expression, name: &Token) -> Result<TokenValue> {
        let object = self.evaluate(object)?;

        match object {
            TokenValue::Instance(instance) => Ok(instance
                .read()
                .unwrap()
                .get(name)
                .unwrap_or(TokenValue::Nil)),
            _ => Err(InterpreterError {
                message: InterpreterErrorMessage::UnexpectedValue {
                    is: object,
                    expect: "Instance".to_string(),
                },
                token: Some(name.to_owned()),
            }
            .into()),
        }
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
        let left = self.evaluate(left)?;

        match operator.kind {
            TokenType::Or => {
                if bool::try_from(left.clone())? {
                    return Ok(left);
                }
            }
            TokenType::And => {
                if !bool::try_from(left.clone())? {
                    return Ok(left);
                }
            }
            _ => {
                return Err(InterpreterError {
                    token: Some(operator.to_owned()),
                    message: InterpreterErrorMessage::Unreachable,
                }
                .into());
            }
        }

        self.evaluate(right)
    }

    fn visit_set(
        &mut self,
        object: &mut Expression,
        name: &Token,
        value: &mut Expression,
    ) -> Result<TokenValue> {
        let object = self.evaluate(object)?;
        let value = self.evaluate(value)?;

        match object {
            TokenValue::Instance(instance) => {
                instance
                    .write()
                    .unwrap()
                    .fields
                    .define(name.lexeme.to_owned(), value.clone());
                Ok(value)
            }
            _ => Err(InterpreterError {
                message: InterpreterErrorMessage::UnexpectedValue {
                    is: object,
                    expect: "Instace".to_string(),
                },
                token: Some(name.to_owned()),
            }
            .into()),
        }
    }

    fn visit_super(&mut self, keyword: &Token, method: &Token) -> Result<TokenValue> {
        todo!()
    }

    fn visit_this(&mut self, keyword: &Token) -> Result<TokenValue> {
        match self.environment.get(keyword) {
            Some(val) => Ok(val),
            None => Err(InterpreterError {
                message: InterpreterErrorMessage::UndefinedVariable {
                    lexeme: keyword.lexeme.clone(),
                },
                token: Some(keyword.to_owned()),
            }
            .into()),
        }
    }

    fn visit_unary(&mut self, operator: &Token, right: &mut Expression) -> Result<TokenValue> {
        let right_val = self.evaluate(right)?;

        match operator.kind {
            TokenType::Bang => {
                let val: bool = right_val.try_into()?;
                return Ok(TokenValue::Bool(!val));
            }
            TokenType::Minus => {
                let num = self.as_number(right_val, operator)?;
                return Ok(TokenValue::Number(-num));
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
        match self.environment.get(name) {
            Some(val) => Ok(val),
            None => Err(InterpreterError {
                message: InterpreterErrorMessage::UndefinedVariable {
                    lexeme: name.lexeme.to_owned(),
                },
                token: Some(name.to_owned()),
            }
            .into()),
        }
    }
}

impl StatementVisitor<Result<Option<TokenValue>>> for Interpreter {
    fn visit_block(&mut self, statements: &mut [Statement]) -> Result<Option<TokenValue>> {
        self.environment.create_environment();
        for stmt in statements {
            match stmt.accept(self) {
                Ok(_) => {}
                Err(err) => {
                    self.environment.delete_environment()?;
                    if let Some(rv) = err.downcast_ref::<ReturnValue>() {
                        return Ok(Some(rv.0.clone()));
                    } else {
                        return Err(err);
                    }
                }
            }
        }

        self.environment.delete_environment()?;
        Ok(None)
    }

    fn visit_class(
        &mut self,
        name: &Token,
        superclass: &mut Option<Box<Expression>>,
        methods: &mut [Statement],
    ) -> Result<Option<TokenValue>> {
        self.environment
            .define(name.lexeme.clone(), TokenValue::Nil)?;

        let mut methods_map = HashMap::new();
        for method in methods {
            match method {
                Statement::Function { name, params, body } => {
                    methods_map.insert(
                        name.lexeme.clone(),
                        self.create_function(name, params, body)?,
                    );
                }
                _ => {
                    return Err(InterpreterError {
                        message: InterpreterErrorMessage::IsNotCallable,
                        token: Some(name.to_owned()),
                    }
                    .into());
                }
            }
        }

        let class = TokenValue::Class(Class {
            name: name.lexeme.clone(),
            methods: methods_map,
        });

        self.environment.assign(name, class.clone())?;
        Ok(Some(class))
    }

    fn visit_expression(&mut self, expression: &mut Expression) -> Result<Option<TokenValue>> {
        Ok(Some(self.evaluate(expression)?))
    }

    fn visit_function(
        &mut self,
        name: &Token,
        params: &[Token],
        body: &mut Statement,
    ) -> Result<Option<TokenValue>> {
        let function = TokenValue::Function(self.create_function(name, params, body)?);
        self.environment.define(name.lexeme.to_owned(), function)?;
        Ok(None)
    }

    fn visit_if(
        &mut self,
        condition: &mut Expression,
        then_branch: &mut Statement,
        else_branch: &mut Option<Box<Statement>>,
    ) -> Result<Option<TokenValue>> {
        if bool::try_from(self.evaluate(condition)?)? {
            then_branch.accept(self)?;
        } else if let Some(else_branch) = else_branch {
            else_branch.accept(self)?;
        }

        Ok(None)
    }

    fn visit_return(&mut self, value: &mut Option<Box<Expression>>) -> Result<Option<TokenValue>> {
        let value = match value {
            Some(val) => self.evaluate(val)?,
            None => TokenValue::Nil,
        };

        Err(ReturnValue(value).into())
    }

    fn visit_var(
        &mut self,
        name: &Token,
        initializer: &mut Option<Box<Expression>>,
    ) -> Result<Option<TokenValue>> {
        let mut value = TokenValue::Nil;
        if let Some(initializer) = initializer {
            value = self.evaluate(initializer)?;
        }

        self.environment
            .define(name.lexeme.to_owned(), value.to_owned())?;
        Ok(Some(value))
    }

    fn visit_while(
        &mut self,
        condition: &mut Expression,
        body: &mut Statement,
    ) -> Result<Option<TokenValue>> {
        while bool::try_from(self.evaluate(condition)?)? {
            body.accept(self)?;
        }

        Ok(None)
    }
}
