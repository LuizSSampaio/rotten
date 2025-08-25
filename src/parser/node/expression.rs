use crate::token::{Token, TokenValue};

#[derive(Debug, Clone)]
pub enum Expression {
    Assign {
        name: Token,
        value: Box<Expression>,
    },
    Binary {
        left: Box<Expression>,
        operator: Token,
        right: Box<Expression>,
    },
    Call {
        callee: Box<Expression>,
        paren: Token,
        arguments: Vec<Expression>,
    },
    Get {
        object: Box<Expression>,
        name: Token,
    },
    Grouping {
        expression: Box<Expression>,
    },
    Literal {
        value: TokenValue,
    },
    Logical {
        left: Box<Expression>,
        operator: Token,
        right: Box<Expression>,
    },
    Set {
        object: Box<Expression>,
        name: Token,
        value: Box<Expression>,
    },
    Super {
        keyword: Token,
        method: Token,
    },
    This {
        keyword: Token,
    },
    Unary {
        operator: Token,
        right: Box<Expression>,
    },
    Variable {
        name: Token,
    },
}

// We'll update the visitor pattern to work with the new enum variants later
// For now, we'll keep the existing visitor traits but update them to work with the new structure

pub trait ExpressionVisitor<T> {
    fn visit_assign(&mut self, name: &Token, value: &mut Expression) -> T;
    fn visit_binary(
        &mut self,
        left: &mut Expression,
        operator: &Token,
        right: &mut Expression,
    ) -> T;
    fn visit_call(
        &mut self,
        callee: &mut Expression,
        paren: &Token,
        arguments: &mut [Expression],
    ) -> T;
    fn visit_get(&mut self, object: &mut Expression, name: &Token) -> T;
    fn visit_grouping(&mut self, expression: &mut Expression) -> T;
    fn visit_literal(&mut self, value: &TokenValue) -> T;
    fn visit_logical(
        &mut self,
        left: &mut Expression,
        operator: &Token,
        right: &mut Expression,
    ) -> T;
    fn visit_set(&mut self, object: &mut Expression, name: &Token, value: &mut Expression) -> T;
    fn visit_super(&mut self, keyword: &Token, method: &Token) -> T;
    fn visit_this(&mut self, keyword: &Token) -> T;
    fn visit_unary(&mut self, operator: &Token, right: &mut Expression) -> T;
    fn visit_variable(&mut self, name: &Token) -> T;
}

impl Expression {
    pub fn accept<T>(&mut self, visitor: &mut impl ExpressionVisitor<T>) -> T {
        match self {
            Expression::Assign { name, value } => visitor.visit_assign(name, value),
            Expression::Binary {
                left,
                operator,
                right,
            } => visitor.visit_binary(left, operator, right),
            Expression::Call {
                callee,
                paren,
                arguments,
            } => visitor.visit_call(callee, paren, arguments),
            Expression::Get { object, name } => visitor.visit_get(object, name),
            Expression::Grouping { expression } => visitor.visit_grouping(expression),
            Expression::Literal { value } => visitor.visit_literal(value),
            Expression::Logical {
                left,
                operator,
                right,
            } => visitor.visit_logical(left, operator, right),
            Expression::Set {
                object,
                name,
                value,
            } => visitor.visit_set(object, name, value),
            Expression::Super { keyword, method } => visitor.visit_super(keyword, method),
            Expression::This { keyword } => visitor.visit_this(keyword),
            Expression::Unary { operator, right } => visitor.visit_unary(operator, right),
            Expression::Variable { name } => visitor.visit_variable(name),
        }
    }
}

