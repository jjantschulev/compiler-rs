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
    match lexer.expect_peek()? {
        TokenType::Integer(i) => {
            let num = *i;
            lexer.next();
            Ok(Expression::Int(num))
        }
        tok => return Err(ParseError::UnexpectedToken(tok.clone())),
    }
}
