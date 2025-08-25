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

pub trait ExpressionVisitor<T> {
    fn visit_assign_expr(&mut self, expression: &mut AssignExpression) -> T;
    fn visit_binary_expr(&mut self, expression: &mut BinaryExpression) -> T;
    fn visit_call_expr(&mut self, expression: &mut CallExpression) -> T;
    fn visit_get_expr(&mut self, expression: &mut GetExpression) -> T;
    fn visit_grouping_expr(&mut self, expression: &mut GroupingExpression) -> T;
    fn visit_literal_expr(&mut self, expression: &mut LiteralExpression) -> T;
    fn visit_logical_expr(&mut self, expression: &mut LogicalExpression) -> T;
    fn visit_set_expr(&mut self, expression: &mut SetExpression) -> T;
    fn visit_super_expr(&mut self, expression: &mut SuperExpression) -> T;
    fn visit_this_expr(&mut self, expression: &mut ThisExpression) -> T;
    fn visit_unary_expr(&mut self, expression: &mut UnaryExpression) -> T;
    fn visit_variable_expr(&mut self, expression: &mut VariableExpression) -> T;
}

pub trait StatementVisitor<T> {
    fn visit_block_stmt(&mut self, statement: &mut BlockStatement) -> T;
    fn visit_class_stmt(&mut self, statement: &mut ClassStatement) -> T;
    fn visit_expression_stmt(&mut self, statement: &mut ExpressionStatement) -> T;
    fn visit_function_stmt(&mut self, statement: &mut FunctionStatement) -> T;
    fn visit_if_stmt(&mut self, statement: &mut IfStatement) -> T;
    fn visit_return_stmt(&mut self, statement: &mut ReturnStatement) -> T;
    fn visit_var_stmt(&mut self, statement: &mut VarStatement) -> T;
    fn visit_while_stmt(&mut self, statement: &mut WhileStatement) -> T;
    fn visit_for_stmt(&mut self, statement: &mut ForStatement) -> T;
}

pub trait Expression {
    fn accept<T>(&mut self, visitor: &mut impl ExpressionVisitor<T>) -> T
    where
        Self: Sized;
}

pub trait Statement {
    fn accept<T>(&mut self, visitor: &mut impl StatementVisitor<T>) -> T
    where
        Self: Sized;
}
