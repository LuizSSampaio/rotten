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
    fn visit_assign_expr<T>(&mut self, expression: &mut AssignExpression) -> T;
    fn visit_binary_expr<T>(&mut self, expression: &mut BinaryExpression) -> T;
    fn visit_call_expr<T>(&mut self, expression: &mut CallExpression) -> T;
    fn visit_get_expr<T>(&mut self, expression: &mut GetExpression) -> T;
    fn visit_grouping_expr<T>(&mut self, expression: &mut GroupingExpression) -> T;
    fn visit_literal_expr<T>(&mut self, expression: &mut LiteralExpression) -> T;
    fn visit_logical_expr<T>(&mut self, expression: &mut LogicalExpression) -> T;
    fn visit_set_expr<T>(&mut self, expression: &mut SetExpression) -> T;
    fn visit_super_expr<T>(&mut self, expression: &mut SuperExpression) -> T;
    fn visit_this_expr<T>(&mut self, expression: &mut ThisExpression) -> T;
    fn visit_unary_expr<T>(&mut self, expression: &mut UnaryExpression) -> T;
    fn visit_variable_expr<T>(&mut self, expression: &mut VariableExpression) -> T;

    fn visit_block_stmt<T>(&mut self, statement: &mut BlockStatement) -> T;
    fn visit_class_stmt<T>(&mut self, statement: &mut ClassStatement) -> T;
    fn visit_expression_stmt<T>(&mut self, statement: &mut ExpressionStatement) -> T;
    fn visit_function_stmt<T>(&mut self, statement: &mut FunctionStatement) -> T;
    fn visit_if_stmt<T>(&mut self, statement: &mut IfStatement) -> T;
    fn visit_return_stmt<T>(&mut self, statement: &mut ReturnStatement) -> T;
    fn visit_var_stmt<T>(&mut self, statement: &mut VarStatement) -> T;
    fn visit_while_stmt<T>(&mut self, statement: &mut WhileStatement) -> T;
    fn visit_for_stmt<T>(&mut self, statement: &mut ForStatement) -> T;
}

pub trait Node {
    fn accept<T>(&mut self, visitor: &mut impl Visitor) -> T
    where
        Self: Sized;
}
