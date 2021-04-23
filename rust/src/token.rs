use crate::token_type::TokenType;
use std::any::Any;

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,

    // Consider using custom type instead of Any
    literal: Box<dyn Any>,
    line: usize,
}
