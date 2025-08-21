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

pub trait Visitor {
    fn visit_assign_expr(&mut self, expression: &mut AssignExpression) -> impl Visitor;
    fn visit_binary_expr(&mut self, expression: &mut BinaryExpression) -> impl Visitor;
    fn visit_call_expr(&mut self, expression: &mut CallExpression) -> impl Visitor;
    fn visit_get_expr(&mut self, expression: &mut GetExpression) -> impl Visitor;
    fn visit_grouping_expr(&mut self, expression: &mut GroupingExpression) -> impl Visitor;
    fn visit_literal_expr(&mut self, expression: &mut LiteralExpression) -> impl Visitor;
    fn visit_logical_expr(&mut self, expression: &mut LogicalExpression) -> impl Visitor;
    fn visit_set_expr(&mut self, expression: &mut SetExpression) -> impl Visitor;
    fn visit_super_expr(&mut self, expression: &mut SuperExpression) -> impl Visitor;
    fn visit_this_expr(&mut self, expression: &mut ThisExpression) -> impl Visitor;
    fn visit_unary_expr(&mut self, expression: &mut UnaryExpression) -> impl Visitor;
    fn visit_variable_expr(&mut self, expression: &mut VariableExpression) -> impl Visitor;

    fn visit_block_stmt(&mut self, statement: &mut BlockStatement) -> impl Visitor;
    fn visit_class_stmt(&mut self, statement: &mut ClassStatement) -> impl Visitor;
    fn visit_expression_stmt(&mut self, statement: &mut ExpressionStatement) -> impl Visitor;
    fn visit_function_stmt(&mut self, statement: &mut FunctionStatement) -> impl Visitor;
    fn visit_if_stmt(&mut self, statement: &mut IfStatement) -> impl Visitor;
    fn visit_return_stmt(&mut self, statement: &mut ReturnStatement) -> impl Visitor;
    fn visit_var_stmt(&mut self, statement: &mut VarStatement) -> impl Visitor;
    fn visit_while_stmt(&mut self, statement: &mut WhileStatement) -> impl Visitor;
    fn visit_for_stmt(&mut self, statement: &mut ForStatement) -> impl Visitor;
}

pub trait Node {
    fn accept(&mut self, visitor: &mut impl Visitor) -> impl Visitor
    where
        Self: Sized;
}
