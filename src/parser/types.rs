use std::rc::Rc;

use crate::lexer::{lexer::Lexer, token_type::TokenType};

use super::helpers::{parse_list, ParseError};

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int,
    Float,
    String,
    Char,
    Bool,
    Void,
    Named(String),
    Ptr(Rc<Type>),
    SizedArray { element: Rc<Type>, len: i64 },
    Array { element: Rc<Type> },
    Struct { fields: Vec<(String, Rc<Type>)> },
    Tuple { fields: Vec<Rc<Type>> },
    Function { args: Vec<Rc<Type>>, ret: Rc<Type> },
}

pub fn parse_type(lexer: &mut Lexer) -> Result<Type, ParseError> {
    let mut ty = parse_type_without_array(lexer)?;

    loop {
        if let Ok(len) = parse_array_type(lexer) {
            ty = match len {
                Some(len) => Type::SizedArray {
                    element: Rc::new(ty),
                    len,
                },
                None => Type::Array {
                    element: Rc::new(ty),
                },
            };
        } else {
            break;
        }
    }

    Ok(ty)
}

fn parse_array_type(lexer: &mut Lexer) -> Result<Option<i64>, ParseError> {
    match lexer.expect_peek()? {
        TokenType::LBracket => {
            lexer.next();
            let len = match lexer.expect_peek()? {
                TokenType::RBracket => None,
                TokenType::Integer(len) => {
                    let len = *len;
                    lexer.next();
                    Some(len)
                }
                tok => return Err(ParseError::UnexpectedToken(tok.clone())),
            };
            lexer.expect_token(&TokenType::RBracket)?;
            Ok(len)
        }
        tok => Err(ParseError::UnexpectedToken(tok.clone())),
    }
}

fn parse_type_without_array(lexer: &mut Lexer) -> Result<Type, ParseError> {
    match lexer.expect_peek()? {
        TokenType::IntType => {
            lexer.next();
            Ok(Type::Int)
        }
        TokenType::FloatType => {
            lexer.next();
            Ok(Type::Float)
        }
        TokenType::StringType => {
            lexer.next();
            Ok(Type::String)
        }
        TokenType::CharType => {
            lexer.next();
            Ok(Type::Char)
        }
        TokenType::BoolType => {
            lexer.next();
            Ok(Type::Bool)
        }
        TokenType::VoidType => {
            lexer.next();
            Ok(Type::Void)
        }
        TokenType::Identifier(_) => {
            if let Some(TokenType::Identifier(name)) = lexer.next() {
                Ok(Type::Named(name))
            } else {
                unreachable!()
            }
        }
        TokenType::Ref => {
            lexer.next();
            let ty = parse_type(lexer)?;
            Ok(Type::Ptr(Rc::new(ty)))
        }
        TokenType::LParen => {
            let fields = parse_list(
                lexer,
                &TokenType::LParen,
                &TokenType::Comma,
                &TokenType::RParen,
                |l| parse_type(l).map(Rc::new),
            )?;

            if lexer.expect_token(&TokenType::FuncArrow).is_ok() {
                let ret = parse_type(lexer)?;
                Ok(Type::Function {
                    args: fields,
                    ret: Rc::new(ret),
                })
            } else {
                Ok(Type::Tuple { fields })
            }
        }

        TokenType::LBrace => {
            let fields = parse_list(
                lexer,
                &TokenType::LBrace,
                &TokenType::Comma,
                &TokenType::RBrace,
                |l| {
                    let name = match l.expect_next()? {
                        TokenType::Identifier(name) => name,
                        tok => return Err(ParseError::UnexpectedToken(tok.clone())),
                    };
                    l.expect_token(&TokenType::Colon)?;
                    let ty = parse_type(l)?;
                    Ok((name, Rc::new(ty)))
                },
            )?;
            Ok(Type::Struct { fields })
        }

        tok => Err(ParseError::UnexpectedToken(tok.clone())),
    }
}
