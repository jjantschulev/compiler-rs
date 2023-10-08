use crate::lexer::{lexer::Lexer, token_type::TokenType};

use super::{helpers::ParseError, statements::Statement};

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Int(i64),
    Float(f64),
    String(String),
    Char(char),
    Bool(bool),

    Identifier(String),

    Add(Box<Expression>, Box<Expression>),
    Sub(Box<Expression>, Box<Expression>),
    Mul(Box<Expression>, Box<Expression>),
    Div(Box<Expression>, Box<Expression>),
    Mod(Box<Expression>, Box<Expression>),
    Neg(Box<Expression>),

    Block(Vec<Statement>, Box<Expression>),

    Call {
        expr: Box<Expression>,
        args: Vec<Expression>,
    },
    Index {
        expr: Box<Expression>,
        index: Box<Expression>,
    },
}

pub fn parse_expression(lexer: &mut Lexer) -> Result<Expression, ParseError> {
    parse_literal(lexer)
}

pub fn parse_literal(lexer: &mut Lexer) -> Result<Expression, ParseError> {
    lexer
        .parse_int()
        .map(|v| Expression::Int(v))
        .or_else(|_| lexer.parse_float().map(|v| Expression::Float(v)))
        .or_else(|_| lexer.parse_char().map(|v| Expression::Char(v)))
        .or_else(|_| lexer.parse_bool().map(|v| Expression::Bool(v)))
        .or_else(|_| lexer.parse_string().map(|v| Expression::String(v)))
        .or_else(|_| lexer.parse_ident().map(|v| Expression::Identifier(v)))
        .or_else(|e| {
            if lexer.parse_token(&TokenType::LParen).is_ok() {
                let expr = parse_expression(lexer)?;
                lexer.parse_token(&TokenType::RParen)?;
                Ok(expr)
            } else {
                Err(e)
            }
        })
}
