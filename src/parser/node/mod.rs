use crate::parser::node::expression::assign::AssignExpression;

pub mod expression;
pub mod statement;

pub trait Visitor {
    fn visit_assign_expr(&mut self, expression: &mut AssignExpression) -> impl Visitor;
    fn visit_binary_expr(&mut self) -> impl Visitor;
    fn visit_call_expr(&mut self) -> impl Visitor;
    fn visit_get_expr(&mut self) -> impl Visitor;
    fn visit_grouping_expr(&mut self) -> impl Visitor;
    fn visit_literal_expr(&mut self) -> impl Visitor;
    fn visit_logical_expr(&mut self) -> impl Visitor;
    fn visit_set_expr(&mut self) -> impl Visitor;
    fn visit_super_expr(&mut self) -> impl Visitor;
    fn visit_this_expr(&mut self) -> impl Visitor;
    fn visit_unary_expr(&mut self) -> impl Visitor;
    fn visit_variable_expr(&mut self) -> impl Visitor;

    fn visit_block_stmt(&mut self) -> impl Visitor;
    fn visit_class_stmt(&mut self) -> impl Visitor;
    fn visit_expression_stmt(&mut self) -> impl Visitor;
    fn visit_function_stmt(&mut self) -> impl Visitor;
    fn visit_if_stmt(&mut self) -> impl Visitor;
    fn visit_print_stmt(&mut self) -> impl Visitor;
    fn visit_return_stmt(&mut self) -> impl Visitor;
    fn visit_var_stmt(&mut self) -> impl Visitor;
    fn visit_while_stmt(&mut self) -> impl Visitor;
}

pub trait Node {
    fn accept(&mut self, visitor: &mut impl Visitor) -> impl Visitor
    where
        Self: Sized;
}
