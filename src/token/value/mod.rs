use std::fmt::Display;

use crate::{
    interpreter::Interpreter, parser::node::statement::Statement, token::value::instance::Instance,
};

pub mod instance;

pub type NativeFn =
    fn(&mut Interpreter, &mut FunctionData, &[TokenValue]) -> anyhow::Result<TokenValue>;

#[derive(Debug, Clone)]
pub struct Function {
    pub data: FunctionData,
    pub call: NativeFn,
}

#[derive(Debug, Clone)]
pub struct Class {
    pub name: String,
}

#[derive(Debug, Clone)]
pub enum TokenValue {
    Bool(bool),
    Number(f64),
    String(String),
    Function(Function),
    Class(Class),
    Instance(Instance),
    Nil,
}

#[derive(Debug, Clone)]
pub struct FunctionData {
    pub body: Option<Vec<Statement>>,
    pub params: Vec<String>,
}

impl PartialEq for TokenValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (TokenValue::Bool(a), TokenValue::Bool(b)) => a == b,
            (TokenValue::Number(a), TokenValue::Number(b)) => a == b,
            (TokenValue::String(a), TokenValue::String(b)) => a == b,
            (TokenValue::Nil, TokenValue::Nil) => true,
            (TokenValue::Function(a), TokenValue::Function(b)) => a.data.params == b.data.params,
            _ => false,
        }
    }
}

impl PartialOrd for TokenValue {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (TokenValue::Bool(a), TokenValue::Bool(b)) => a.partial_cmp(b),
            (TokenValue::Number(a), TokenValue::Number(b)) => a.partial_cmp(b),
            (TokenValue::String(a), TokenValue::String(b)) => a.partial_cmp(b),
            (TokenValue::Nil, TokenValue::Nil) => Some(std::cmp::Ordering::Equal),
            (TokenValue::Function(a), TokenValue::Function(b)) => {
                a.data.params.len().partial_cmp(&b.data.params.len())
            }
            _ => None,
        }
    }
}

impl TryFrom<TokenValue> for bool {
    type Error = anyhow::Error;

    fn try_from(value: TokenValue) -> Result<Self, Self::Error> {
        match value {
            TokenValue::Bool(val) => Ok(val),
            TokenValue::Number(val) => Ok(val != 0.0),
            TokenValue::String(val) => Ok(!val.is_empty()),
            TokenValue::Nil => Ok(false),
            TokenValue::Function(_) => Err(anyhow::anyhow!("Cannot convert function to bool")),
            TokenValue::Class(_) => Err(anyhow::anyhow!("Cannot convert class to bool")),
            TokenValue::Instance(_) => Err(anyhow::anyhow!("Cannot convert Instance to bool")),
        }
    }
}

impl From<bool> for TokenValue {
    fn from(v: bool) -> Self {
        Self::Bool(v)
    }
}

macro_rules! impl_from_numeric {
    ($($t:ty),*) => {
        $(
            impl From<$t> for TokenValue {
                fn from(v: $t) -> Self {
                    Self::Number(v as f64)
                }
            }
        )*
    };
}

impl_from_numeric!(i8, i16, i32, i64, isize, u8, u16, u32, u64, usize, f32, f64);

impl From<String> for TokenValue {
    fn from(v: String) -> Self {
        Self::String(v)
    }
}

impl From<&str> for TokenValue {
    fn from(v: &str) -> Self {
        Self::String(v.to_owned())
    }
}

impl TryFrom<TokenValue> for f64 {
    type Error = anyhow::Error;

    fn try_from(value: TokenValue) -> Result<Self, Self::Error> {
        match value {
            TokenValue::Bool(val) => {
                if val {
                    return Ok(1.0);
                }
                Ok(0.0)
            }
            TokenValue::Number(val) => Ok(val),
            TokenValue::String(val) => val
                .parse()
                .map_err(|_| anyhow::anyhow!("Failed to parse string as f64: {:?}", val)),
            TokenValue::Nil => Err(anyhow::anyhow!("Cannot convert nil to f64")),
            TokenValue::Function(_) => Err(anyhow::anyhow!("Cannot convert function to f64")),
            TokenValue::Class(_) => Err(anyhow::anyhow!("Cannot convert class to f64")),
            TokenValue::Instance(_) => Err(anyhow::anyhow!("Cannot convert Instance to f64")),
        }
    }
}

impl Display for TokenValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            TokenValue::Bool(val) => val.to_string(),
            TokenValue::Number(val) => val.to_string(),
            TokenValue::String(val) => val.to_owned(),
            TokenValue::Nil => String::from("nil"),
            TokenValue::Function(_) => String::from("native function"),
            TokenValue::Class(val) => val.name.to_string(),
            TokenValue::Instance(val) => format!("{} instance", val.class.name),
        };
        write!(f, "{}", text)
    }
}
