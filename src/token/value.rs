use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum TokenValue {
    Bool(bool),
    Number(f64),
    String(String),
    Nil,
}

impl From<TokenValue> for bool {
    fn from(value: TokenValue) -> Self {
        match value {
            TokenValue::Bool(val) => val,
            TokenValue::Number(val) => val != 0.0,
            TokenValue::String(val) => !val.is_empty(),
            TokenValue::Nil => false,
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
        };
        write!(f, "{}", text)
    }
}
