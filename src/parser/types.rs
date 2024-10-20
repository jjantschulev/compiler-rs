use std::{boxed::Box, collections::HashMap};

use crate::lexer::{lexer::Lexer, token::Token};

use super::{
    expressions::{parse_expression, Expression},
    helpers::{build_hashmap_from_entries, parse_list, ParseError},
};

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int,
    Float,
    String,
    Char,
    Bool,
    Void,
    Named(String),
    Ptr(Box<Type>),
    SizedArray {
        element: Box<Type>,
        len: i64,
    },
    Array(Box<Type>),
    Struct(HashMap<String, Type>),
    Tuple(Vec<Box<Type>>),
    Function {
        args: Vec<Box<Type>>,
        ret: Box<Type>,
    },
    TypeOf(Box<Expression>),
}

pub fn parse_type(lexer: &mut Lexer) -> Result<Type, ParseError> {
    let mut ty = parse_type_without_array(lexer)?;

    loop {
        if let Ok(len) = parse_array_type(lexer) {
            ty = match len {
                Some(len) => Type::SizedArray {
                    element: Box::new(ty),
                    len,
                },
                None => Type::Array(Box::new(ty)),
            };
        } else {
            break;
        }
    }

    Ok(ty)
}

fn parse_array_type(lexer: &mut Lexer) -> Result<Option<i64>, ParseError> {
    match lexer.expect_peek()? {
        Token::LBracket => {
            lexer.next();
            let len = lexer.parse_int();
            lexer.parse_token(&Token::RBracket)?;
            Ok(len.ok())
        }
        tok => Err(ParseError::UnexpectedToken(tok.clone())),
    }
}

fn parse_type_without_array(lexer: &mut Lexer) -> Result<Type, ParseError> {
    match lexer.expect_peek()? {
        Token::IntType => {
            lexer.next();
            Ok(Type::Int)
        }
        Token::FloatType => {
            lexer.next();
            Ok(Type::Float)
        }
        Token::StringType => {
            lexer.next();
            Ok(Type::String)
        }
        Token::CharType => {
            lexer.next();
            Ok(Type::Char)
        }
        Token::BoolType => {
            lexer.next();
            Ok(Type::Bool)
        }
        Token::VoidType => {
            lexer.next();
            Ok(Type::Void)
        }
        Token::Identifier(_) => {
            let name = lexer.parse_ident()?;
            Ok(Type::Named(name))
        }
        Token::Ref => {
            lexer.next();
            let ty = parse_type(lexer)?;
            Ok(Type::Ptr(Box::new(ty)))
        }
        Token::LParen => {
            let fields = parse_list(lexer, &Token::LParen, &Token::Comma, &Token::RParen, |l| {
                parse_type(l).map(Box::new)
            })?;

            if lexer.parse_token(&Token::FuncArrow).is_ok() {
                let ret = parse_type(lexer)?;
                Ok(Type::Function {
                    args: fields,
                    ret: Box::new(ret),
                })
            } else {
                Ok(Type::Tuple(fields))
            }
        }

        Token::LBrace => {
            let fields = parse_list(lexer, &Token::LBrace, &Token::Comma, &Token::RBrace, |l| {
                let name = l.parse_ident()?;
                l.parse_token(&Token::Colon)?;
                let ty = parse_type(l)?;
                Ok((name, ty))
            })?;

            let fields = build_hashmap_from_entries(fields)?;

            Ok(Type::Struct(fields))
        }

        Token::TypeOf => {
            lexer.next();
            lexer.parse_token(&Token::LParen)?;
            let expr = parse_expression(lexer)?;
            lexer.parse_token(&Token::RParen)?;
            Ok(Type::TypeOf(Box::new(expr)))
        }

        tok => Err(ParseError::UnexpectedToken(tok.clone())),
    }
}
