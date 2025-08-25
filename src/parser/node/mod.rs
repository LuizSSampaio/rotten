use crate::parser::node::expression::{
    assign::AssignExpression, binary::BinaryExpression, call::CallExpression, get::GetExpression,
    grouping::GroupingExpression, literal::LiteralExpression, logical::LogicalExpression,
    set::SetExpression, super_class::SuperExpression, this::ThisExpression, unary::UnaryExpression,
    variable::VariableExpression,
};

use crate::parser::node::statement::{
    block::BlockStatement, class::ClassStatement, expression::ExpressionStatement,
    for_statement::ForStatement, function::FunctionStatement, if_statement::IfStatement,
    return_statement::ReturnStatement, var::VarStatement, while_loop::WhileStatement,
};

pub mod expression;
pub mod statement;

#[derive(Debug, Clone)]
pub enum Expression {
    Assign(AssignExpression),
    Binary(BinaryExpression),
    Call(CallExpression),
    Get(GetExpression),
    Grouping(GroupingExpression),
    Literal(LiteralExpression),
    Logical(LogicalExpression),
    Set(SetExpression),
    Super(SuperExpression),
    This(ThisExpression),
    Unary(UnaryExpression),
    Variable(VariableExpression),
}

impl Expression {
    pub fn accept<T>(&mut self, visitor: &mut impl ExpressionVisitor<T>) -> T {
        match self {
            Expression::Assign(expr) => visitor.visit_assign(expr),
            Expression::Binary(expr) => visitor.visit_binary(expr),
            Expression::Call(expr) => visitor.visit_call(expr),
            Expression::Get(expr) => visitor.visit_get(expr),
            Expression::Grouping(expr) => visitor.visit_grouping(expr),
            Expression::Literal(expr) => visitor.visit_literal(expr),
            Expression::Logical(expr) => visitor.visit_logical(expr),
            Expression::Set(expr) => visitor.visit_set(expr),
            Expression::Super(expr) => visitor.visit_super(expr),
            Expression::This(expr) => visitor.visit_this(expr),
            Expression::Unary(expr) => visitor.visit_unary(expr),
            Expression::Variable(expr) => visitor.visit_variable(expr),
        }
    }
}

pub trait ExpressionVisitor<T> {
    fn visit_assign(&mut self, expr: &mut AssignExpression) -> T;
    fn visit_binary(&mut self, expr: &mut BinaryExpression) -> T;
    fn visit_call(&mut self, expr: &mut CallExpression) -> T;
    fn visit_get(&mut self, expr: &mut GetExpression) -> T;
    fn visit_grouping(&mut self, expr: &mut GroupingExpression) -> T;
    fn visit_literal(&mut self, expr: &mut LiteralExpression) -> T;
    fn visit_logical(&mut self, expr: &mut LogicalExpression) -> T;
    fn visit_set(&mut self, expr: &mut SetExpression) -> T;
    fn visit_super(&mut self, expr: &mut SuperExpression) -> T;
    fn visit_this(&mut self, expr: &mut ThisExpression) -> T;
    fn visit_unary(&mut self, expr: &mut UnaryExpression) -> T;
    fn visit_variable(&mut self, expr: &mut VariableExpression) -> T;
}

#[derive(Debug, Clone)]
pub enum Statement {
    Block(BlockStatement),
    Class(ClassStatement),
    Expression(ExpressionStatement),
    Function(FunctionStatement),
    If(IfStatement),
    Return(ReturnStatement),
    Var(VarStatement),
    While(WhileStatement),
    For(ForStatement),
}

impl Statement {
    pub fn accept<T>(&mut self, visitor: &mut impl StatementVisitor<T>) -> T {
        match self {
            Statement::Block(stmt) => visitor.visit_block(stmt),
            Statement::Class(stmt) => visitor.visit_class(stmt),
            Statement::Expression(stmt) => visitor.visit_expression(stmt),
            Statement::Function(stmt) => visitor.visit_function(stmt),
            Statement::If(stmt) => visitor.visit_if(stmt),
            Statement::Return(stmt) => visitor.visit_return(stmt),
            Statement::Var(stmt) => visitor.visit_var(stmt),
            Statement::While(stmt) => visitor.visit_while(stmt),
            Statement::For(stmt) => visitor.visit_for(stmt),
        }
    }
}

pub trait StatementVisitor<T> {
    fn visit_block(&mut self, stmt: &mut BlockStatement) -> T;
    fn visit_class(&mut self, stmt: &mut ClassStatement) -> T;
    fn visit_expression(&mut self, stmt: &mut ExpressionStatement) -> T;
    fn visit_function(&mut self, stmt: &mut FunctionStatement) -> T;
    fn visit_if(&mut self, stmt: &mut IfStatement) -> T;
    fn visit_return(&mut self, stmt: &mut ReturnStatement) -> T;
    fn visit_var(&mut self, stmt: &mut VarStatement) -> T;
    fn visit_while(&mut self, stmt: &mut WhileStatement) -> T;
    fn visit_for(&mut self, stmt: &mut ForStatement) -> T;
}
