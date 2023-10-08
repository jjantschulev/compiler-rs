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

    pub fn parse_token(&mut self, token: &TokenType) -> Result<TokenType, ParseError> {
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

    pub fn parse_ident(&mut self) -> Result<String, ParseError> {
        match self.raw.peek() {
            Some(TokenType::Identifier(_)) => match self.raw.next() {
                Some(TokenType::Identifier(ident)) => Ok(ident),
                _ => unreachable!(),
            },
            Some(tok) => Err(ParseError::UnexpectedToken(tok.clone())),
            None => Err(ParseError::UnexpectedEOF()),
        }
    }
    pub fn parse_string(&mut self) -> Result<String, ParseError> {
        match self.raw.peek() {
            Some(TokenType::String(_)) => match self.raw.next() {
                Some(TokenType::String(val)) => Ok(val),
                _ => unreachable!(),
            },
            Some(tok) => Err(ParseError::UnexpectedToken(tok.clone())),
            None => Err(ParseError::UnexpectedEOF()),
        }
    }
    pub fn parse_int(&mut self) -> Result<i64, ParseError> {
        match self.raw.peek() {
            Some(TokenType::Integer(_)) => match self.raw.next() {
                Some(TokenType::Integer(val)) => Ok(val),
                _ => unreachable!(),
            },
            Some(tok) => Err(ParseError::UnexpectedToken(tok.clone())),
            None => Err(ParseError::UnexpectedEOF()),
        }
    }
    pub fn parse_float(&mut self) -> Result<f64, ParseError> {
        match self.raw.peek() {
            Some(TokenType::Float(_)) => match self.raw.next() {
                Some(TokenType::Float(val)) => Ok(val),
                _ => unreachable!(),
            },
            Some(tok) => Err(ParseError::UnexpectedToken(tok.clone())),
            None => Err(ParseError::UnexpectedEOF()),
        }
    }
    pub fn parse_char(&mut self) -> Result<char, ParseError> {
        match self.raw.peek() {
            Some(TokenType::Char(_)) => match self.raw.next() {
                Some(TokenType::Char(val)) => Ok(val),
                _ => unreachable!(),
            },
            Some(tok) => Err(ParseError::UnexpectedToken(tok.clone())),
            None => Err(ParseError::UnexpectedEOF()),
        }
    }
    pub fn parse_bool(&mut self) -> Result<bool, ParseError> {
        match self.raw.peek() {
            Some(TokenType::Bool(_)) => match self.raw.next() {
                Some(TokenType::Bool(val)) => Ok(val),
                _ => unreachable!(),
            },
            Some(tok) => Err(ParseError::UnexpectedToken(tok.clone())),
            None => Err(ParseError::UnexpectedEOF()),
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = TokenType;
    fn next(&mut self) -> Option<Self::Item> {
        self.raw.next()
    }
}
