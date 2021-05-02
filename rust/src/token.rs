use crate::token_type::TokenType;
use std::any::Any;

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,

    // Consider using custom type instead of Any
    literal: Option<Box<dyn Any>>,
    line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: Option<Box<dyn Any>>, line: usize) -> Self {
        Token { token_type, lexeme, literal, line}
    }
}
