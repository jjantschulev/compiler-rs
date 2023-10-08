use std::iter::Peekable;

use crate::parser::helpers::ParseError;

use super::{raw_lexer::RawLexer, token_type::TokenType};

#[derive(Debug, Clone)]
pub struct Lexer<'a> {
    raw: Peekable<RawLexer<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            raw: RawLexer::new(input).peekable(),
        }
    }

    pub fn peek(&mut self) -> Option<&TokenType> {
        self.raw.peek()
    }

    pub fn expect_token(&mut self, token: &TokenType) -> Result<TokenType, ParseError> {
        match self.raw.peek() {
            Some(tok) if tok == token => {
                let tok = tok.clone();
                self.raw
                    .next()
                    .ok_or_else(|| ParseError::UnexpectedToken(tok))
            }
            Some(tok) => Err(ParseError::UnexpectedToken(tok.clone())),
            None => Err(ParseError::UnexpectedEOF()),
        }
    }

    pub fn expect_next(&mut self) -> Result<TokenType, ParseError> {
        self.raw.next().ok_or(ParseError::UnexpectedEOF())
    }
    pub fn expect_peek(&mut self) -> Result<&TokenType, ParseError> {
        self.raw.peek().ok_or(ParseError::UnexpectedEOF())
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = TokenType;
    fn next(&mut self) -> Option<Self::Item> {
        self.raw.next()
    }
}
