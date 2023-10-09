use std::collections::HashMap;

use crate::lexer::{lexer::Lexer, token::Token};

use super::{
    helpers::{build_hashmap_from_entries, parse_list, ParseError},
    statements::{parse_block, Block},
    types::{parse_type, Type},
};

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Int(i64),
    Float(f64),
    String(String),
    Char(char),
    Bool(bool),

    StructLiteral(HashMap<String, Expression>),
    ArrayLiteral(Vec<Expression>),
    FunctionLiteral {
        args: Vec<(String, Type)>,
        ret: Type,
        body: Block,
    },
    TupleLiteral(Vec<Expression>),

    Identifier(String),

    Add(Box<Expression>, Box<Expression>),
    Sub(Box<Expression>, Box<Expression>),
    Mul(Box<Expression>, Box<Expression>),
    Div(Box<Expression>, Box<Expression>),
    Mod(Box<Expression>, Box<Expression>),

    Neg(Box<Expression>),

    Equal(Box<Expression>, Box<Expression>),
    GreaterEqual(Box<Expression>, Box<Expression>),
    GreaterThan(Box<Expression>, Box<Expression>),
    LessEqual(Box<Expression>, Box<Expression>),
    LessThan(Box<Expression>, Box<Expression>),

    And(Box<Expression>, Box<Expression>),
    Or(Box<Expression>, Box<Expression>),
    Not(Box<Expression>),

    Ref(Box<Expression>),
    Deref(Box<Expression>),

    Null,

    // Block(Vec<Statement>, Box<Expression>),
    Call {
        expr: Box<Expression>,
        args: Vec<Expression>,
    },
    Index {
        expr: Box<Expression>,
        index: Box<Expression>,
    },
    Dot {
        expr: Box<Expression>,
        field: String,
    },
    // Cast {
    //     expr: Box<Expression>,
    //     typ: Type,
    // },
}

pub fn parse_expression(lexer: &mut Lexer) -> Result<Expression, ParseError> {
    parse_e1(lexer)
}

// Or
fn parse_e1(lexer: &mut Lexer) -> Result<Expression, ParseError> {
    let lhs = parse_e2(lexer)?;
    if lexer.parse_token(&Token::Or).is_ok() {
        let rhs = parse_e1(lexer)?;
        Ok(Expression::Or(Box::new(lhs), Box::new(rhs)))
    } else {
        Ok(lhs)
    }
}

// And
fn parse_e2(lexer: &mut Lexer) -> Result<Expression, ParseError> {
    let lhs = parse_e3(lexer)?;
    if lexer.parse_token(&Token::And).is_ok() {
        let rhs = parse_e2(lexer)?;
        Ok(Expression::And(Box::new(lhs), Box::new(rhs)))
    } else {
        Ok(lhs)
    }
}

// Not
fn parse_e3(lexer: &mut Lexer) -> Result<Expression, ParseError> {
    if lexer.parse_token(&Token::Not).is_ok() {
        let expr = parse_e3(lexer)?;
        Ok(Expression::Not(Box::new(expr)))
    } else {
        parse_e4(lexer)
    }
}

// Comparison
fn parse_e4(lexer: &mut Lexer) -> Result<Expression, ParseError> {
    let lhs = parse_e5(lexer)?;
    match lexer.expect_peek()? {
        Token::Equal => {
            lexer.next();
            let rhs = parse_e4(lexer)?;
            Ok(Expression::Equal(Box::new(lhs), Box::new(rhs)))
        }
        Token::GreaterThan => {
            lexer.next();
            let rhs = parse_e4(lexer)?;
            Ok(Expression::GreaterThan(Box::new(lhs), Box::new(rhs)))
        }
        Token::GreaterEqual => {
            lexer.next();
            let rhs = parse_e4(lexer)?;
            Ok(Expression::GreaterEqual(Box::new(lhs), Box::new(rhs)))
        }
        Token::LessThan => {
            lexer.next();
            let rhs = parse_e4(lexer)?;
            Ok(Expression::LessThan(Box::new(lhs), Box::new(rhs)))
        }
        Token::LessEqual => {
            lexer.next();
            let rhs = parse_e4(lexer)?;
            Ok(Expression::LessEqual(Box::new(lhs), Box::new(rhs)))
        }
        _ => Ok(lhs),
    }
}

// Add/Sub
fn parse_e5(lexer: &mut Lexer) -> Result<Expression, ParseError> {
    let mut lhs = parse_e6(lexer)?;
    loop {
        match lexer.peek() {
            Some(Token::Add) => {
                lexer.next();
                let rhs = parse_e6(lexer)?;
                lhs = Expression::Add(Box::new(lhs), Box::new(rhs));
            }
            Some(Token::Sub) => {
                lexer.next();
                let rhs = parse_e6(lexer)?;
                lhs = Expression::Sub(Box::new(lhs), Box::new(rhs));
            }
            _ => break,
        }
    }
    Ok(lhs)
}

// Mul/Div/Mod
fn parse_e6(lexer: &mut Lexer) -> Result<Expression, ParseError> {
    let mut lhs = parse_e7(lexer)?;
    loop {
        match lexer.peek() {
            Some(Token::Mul) => {
                lexer.next();
                let rhs = parse_e7(lexer)?;
                lhs = Expression::Mul(Box::new(lhs), Box::new(rhs));
            }
            Some(Token::Div) => {
                lexer.next();
                let rhs = parse_e7(lexer)?;
                lhs = Expression::Div(Box::new(lhs), Box::new(rhs));
            }
            Some(Token::Mod) => {
                lexer.next();
                let rhs = parse_e7(lexer)?;
                lhs = Expression::Mod(Box::new(lhs), Box::new(rhs));
            }
            _ => break,
        }
    }
    Ok(lhs)
}

// Neg/Pos
fn parse_e7(lexer: &mut Lexer) -> Result<Expression, ParseError> {
    if lexer.parse_token(&Token::Sub).is_ok() {
        let expr = parse_e7(lexer)?;
        Ok(Expression::Neg(Box::new(expr)))
    } else {
        parse_e8(lexer)
    }
}

fn parse_e8(lexer: &mut Lexer) -> Result<Expression, ParseError> {
    if lexer.parse_token(&Token::Ref).is_ok() {
        let expr = parse_e8(lexer)?;
        Ok(Expression::Ref(Box::new(expr)))
    } else if lexer.parse_token(&Token::Mul).is_ok() {
        let expr = parse_e8(lexer)?;
        Ok(Expression::Deref(Box::new(expr)))
    } else {
        parse_e9(lexer)
    }
}

// Call/Index/Dot
fn parse_e9(lexer: &mut Lexer) -> Result<Expression, ParseError> {
    let mut expr = parse_literal(lexer)?;
    loop {
        match lexer.expect_peek()? {
            Token::LParen => {
                let args = parse_list(
                    lexer,
                    &Token::LParen,
                    &Token::Comma,
                    &Token::RParen,
                    parse_expression,
                )?;
                expr = Expression::Call {
                    expr: Box::new(expr),
                    args,
                };
            }
            Token::LBracket => {
                lexer.next();
                let index = parse_expression(lexer)?;
                lexer.parse_token(&Token::RBracket)?;
                expr = Expression::Index {
                    expr: Box::new(expr),
                    index: Box::new(index),
                };
            }
            Token::Dot => {
                lexer.next();
                let field = lexer.parse_ident()?;
                expr = Expression::Dot {
                    expr: Box::new(expr),
                    field,
                };
            }
            _ => break,
        }
    }
    Ok(expr)
}

fn parse_literal(lexer: &mut Lexer) -> Result<Expression, ParseError> {
    lexer
        .parse_int()
        .map(|v| Expression::Int(v))
        .or_else(|_| lexer.parse_token(&Token::Null).map(|_| Expression::Null))
        .or_else(|_| lexer.parse_float().map(|v| Expression::Float(v)))
        .or_else(|_| lexer.parse_char().map(|v| Expression::Char(v)))
        .or_else(|_| lexer.parse_bool().map(|v| Expression::Bool(v)))
        .or_else(|_| lexer.parse_string().map(|v| Expression::String(v)))
        .or_else(|_| lexer.parse_ident().map(|v| Expression::Identifier(v)))
        .or_else(|_| parse_struct_literal(lexer))
        .or_else(|_| parse_array_literal(lexer))
        .or_else(|_| {
            if is_func_literal(lexer).is_ok() {
                parse_function_literal(lexer)
            } else {
                let items = parse_list(
                    lexer,
                    &Token::LParen,
                    &Token::Comma,
                    &Token::RParen,
                    parse_expression,
                )?;
                if items.len() == 1 {
                    Ok(items.into_iter().next().unwrap())
                } else {
                    Ok(Expression::TupleLiteral(items))
                }
            }
        })
}

fn parse_struct_literal(lexer: &mut Lexer) -> Result<Expression, ParseError> {
    let fields = parse_list(lexer, &Token::LBrace, &Token::Comma, &Token::RBrace, |l| {
        let name = l.parse_ident()?;
        l.parse_token(&Token::Colon)?;
        let expr = parse_expression(l)?;
        Ok((name, expr))
    })?;

    let fields = build_hashmap_from_entries(fields)?;

    Ok(Expression::StructLiteral(fields))
}

fn parse_array_literal(lexer: &mut Lexer) -> Result<Expression, ParseError> {
    let fields = parse_list(
        lexer,
        &Token::LBracket,
        &Token::Comma,
        &Token::RBracket,
        parse_expression,
    )?;

    Ok(Expression::ArrayLiteral(fields))
}

fn is_func_literal(lexer: &Lexer) -> Result<(), ParseError> {
    let mut lexer = lexer.clone();
    lexer.parse_token(&Token::LParen)?;
    if lexer.parse_token(&Token::RParen).is_ok() {
        return Ok(());
    }
    lexer.parse_ident()?;
    lexer.parse_token(&Token::Colon)?;
    Ok(())
}

fn parse_function_literal(lexer: &mut Lexer) -> Result<Expression, ParseError> {
    let args = parse_list(
        lexer,
        &Token::LParen,
        &Token::Comma,
        &Token::RParen,
        |lexer| {
            let name = lexer.parse_ident()?;
            lexer.parse_token(&Token::Colon)?;
            let typ = parse_type(lexer)?;
            Ok((name, typ))
        },
    )?;

    let ret = if lexer.parse_token(&Token::Colon).is_ok() {
        parse_type(lexer)?
    } else {
        Type::Void
    };

    lexer.parse_token(&Token::FuncArrow)?;

    let body = parse_block(lexer, true)?;

    Ok(Expression::FunctionLiteral { args, ret, body })
}
