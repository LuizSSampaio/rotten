use std::sync::{Arc, RwLock};

use crate::{
    interpreter::Interpreter,
    parser::node::statement::Statement,
    token::value::{TokenValue, instance::Instance},
};

pub type NativeFn =
    fn(&mut Interpreter, &mut FunctionData, &[TokenValue]) -> anyhow::Result<TokenValue>;

#[derive(Debug, Clone)]
pub struct Function {
    pub data: FunctionData,
    pub call: NativeFn,
}

#[derive(Debug, Clone)]
pub struct FunctionData {
    pub body: Option<Vec<Statement>>,
    pub params: Vec<String>,
    pub this: Option<Arc<RwLock<Instance>>>,
}
