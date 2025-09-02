use std::fmt::Display;

use crate::parser::node::statement::StatementVisitor;

pub type NativeFn = fn(
    &mut dyn StatementVisitor<anyhow::Result<Option<TokenValue>>>,
    &[TokenValue],
) -> anyhow::Result<Option<TokenValue>>;

#[derive(Debug, Clone)]
pub enum TokenValue {
    Bool(bool),
    Number(f64),
    String(String),
    Function { arity: u8, call: NativeFn },
    Nil,
}

impl PartialEq for TokenValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (TokenValue::Bool(a), TokenValue::Bool(b)) => a == b,
            (TokenValue::Number(a), TokenValue::Number(b)) => a == b,
            (TokenValue::String(a), TokenValue::String(b)) => a == b,
            (TokenValue::Nil, TokenValue::Nil) => true,
            (TokenValue::Function { arity: a, .. }, TokenValue::Function { arity: b, .. }) => {
                a == b
            }
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
            (TokenValue::Function { arity: a, .. }, TokenValue::Function { arity: b, .. }) => {
                a.partial_cmp(b)
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
            TokenValue::Function { .. } => Err(anyhow::anyhow!("Cannot convert function to bool")),
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
            TokenValue::Function { .. } => Err(anyhow::anyhow!("Cannot convert function to f64")),
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
            TokenValue::Function { .. } => String::from("native function"),
        };
        write!(f, "{}", text)
    }
}
