use std::any::{self, Any};

use crate::token::Token;
use crate::token_type::TokenType;

pub struct Scanner<'a> {
    source: &'a str,
    tokens: Vec<Token>,

    start: usize,
    current: usize,
    line: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Scanner {
            source: source,
            tokens: vec![],
            start: 0,
            current: 0,
            line: 0,
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while self.current < self.source.len() {
            self.start = self.current;
            // Scan token
        }

        self.tokens.push(Token::new(TokenType::Eof, String::new(), None, self.line));
        &self.tokens
    }
}