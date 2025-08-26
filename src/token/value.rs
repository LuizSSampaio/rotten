use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum TokenValue {
    Bool(bool),
    Number(f64),
    String(String),
    Nil,
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
