use crate::{parser::node::expression::Expression, token::Token};

#[derive(Debug, Clone)]
pub enum Statement {
    Block {
        statements: Vec<Statement>,
    },
    Class {
        name: Token,
        superclass: Option<Box<Expression>>,
        methods: Vec<Statement>,
    },
    Expression {
        expression: Box<Expression>,
    },
    Function {
        name: Token,
        params: Vec<Token>,
        body: Vec<Statement>,
    },
    If {
        condition: Box<Expression>,
        then_branch: Box<Statement>,
        else_branch: Option<Box<Statement>>,
    },
    Return {
        keyword: Token,
        value: Option<Box<Expression>>,
    },
    Var {
        name: Token,
        initializer: Option<Box<Expression>>,
    },
    While {
        condition: Box<Expression>,
        body: Box<Statement>,
    },
}

pub trait StatementVisitor<T> {
    fn visit_block(&mut self, statements: &mut [Statement]) -> T;
    fn visit_class(
        &mut self,
        name: &Token,
        superclass: &mut Option<Box<Expression>>,
        methods: &mut [Statement],
    ) -> T;
    fn visit_expression(&mut self, expression: &mut Expression) -> T;
    fn visit_function(&mut self, name: &Token, params: &[Token], body: &mut [Statement]) -> T;
    fn visit_if(
        &mut self,
        condition: &mut Expression,
        then_branch: &mut Statement,
        else_branch: &mut Option<Box<Statement>>,
    ) -> T;
    fn visit_return(&mut self, keyword: &Token, value: &mut Option<Box<Expression>>) -> T;
    fn visit_var(&mut self, name: &Token, initializer: &mut Option<Box<Expression>>) -> T;
    fn visit_while(&mut self, condition: &mut Expression, body: &mut Statement) -> T;
}

impl Statement {
    pub fn accept<T>(&mut self, visitor: &mut impl StatementVisitor<T>) -> T {
        match self {
            Statement::Block { statements } => visitor.visit_block(statements),
            Statement::Class {
                name,
                superclass,
                methods,
            } => visitor.visit_class(name, superclass, methods),
            Statement::Expression { expression } => visitor.visit_expression(expression),
            Statement::Function { name, params, body } => {
                visitor.visit_function(name, params, body)
            }
            Statement::If {
                condition,
                then_branch,
                else_branch,
            } => visitor.visit_if(condition, then_branch, else_branch),
            Statement::Return { keyword, value } => visitor.visit_return(keyword, value),
            Statement::Var { name, initializer } => visitor.visit_var(name, initializer),
            Statement::While { condition, body } => visitor.visit_while(condition, body),
        }
    }
}
