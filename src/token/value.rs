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
        let debug_str = format!("{:?}", self);
        let variant_name = debug_str
            .split_once('(')
            .map(|(name, _)| name)
            .unwrap_or(&debug_str);
        write!(f, "{}", variant_name.to_lowercase())
    }
}
