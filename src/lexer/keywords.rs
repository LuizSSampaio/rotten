use std::collections::HashMap;

use crate::token::kind::TokenType;

pub fn create_keywords() -> HashMap<&'static str, TokenType> {
    let mut keywords = HashMap::new();
    keywords.insert("and", TokenType::And);
    keywords.insert("class", TokenType::Class);
    keywords.insert("else", TokenType::Else);
    keywords.insert("false", TokenType::False);
    keywords.insert("for", TokenType::For);
    keywords.insert("fun", TokenType::Fun);
    keywords.insert("if", TokenType::If);
    keywords.insert("nil", TokenType::Nil);
    keywords.insert("or", TokenType::Or);
    keywords.insert("return", TokenType::Return);
    keywords.insert("super", TokenType::Super);
    keywords.insert("this", TokenType::This);
    keywords.insert("true", TokenType::True);
    keywords.insert("var", TokenType::Var);
    keywords.insert("while", TokenType::While);
    keywords
}
