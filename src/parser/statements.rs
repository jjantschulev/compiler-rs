use crate::lexer::{lexer::Lexer, token::Token};

use super::{
    expressions::{parse_expression, Expression},
    helpers::{parse_list, ParseError},
    types::{parse_type, Type},
};

pub type Block = Vec<Statement>;

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Import {
        path: String,
        imports: Vec<ImportIdentifier>,
    },
    TypeDef {
        name: String,
        typ: Type,
    },
    VarDef {
        name: String,
        typ: Option<Type>,
        expr: Expression,
    },
    If {
        cond: Expression,
        body: Block,
        else_stmt: ElseStatement,
    },
    While {
        cond: Expression,
        body: Block,
    },
    Assign {
        lhs: Expression,
        rhs: Expression,
    },

    Return(Option<Expression>),
    Break,
    Continue,
    Loop(Block),

    Expr(Expression),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ImportIdentifier {
    name: String,
    alias: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ElseStatement {
    If(Box<Statement>),
    Block(Block),
    None,
}

pub fn parse_block(lexer: &mut Lexer, enclosed_by_brackets: bool) -> Result<Block, ParseError> {
    let mut statements = Vec::new();

    if enclosed_by_brackets {
        lexer.parse_token(&Token::LBrace)?;
    }

    loop {
        if enclosed_by_brackets && lexer.parse_token(&Token::RBrace).is_ok() {
            break;
        }
        match lexer.peek() {
            Some(_) => statements.push(parse_statement(lexer)?),
            None => break,
        }
    }

    Ok(statements)
}

pub fn parse_statement(lexer: &mut Lexer) -> Result<Statement, ParseError> {
    match lexer.expect_peek()? {
        Token::Type => {
            lexer.next();
            let name = lexer.parse_ident()?;
            lexer.parse_token(&Token::Assign)?;
            let typ = parse_type(lexer)?;
            lexer.parse_token(&Token::Semicolon)?;
            Ok(Statement::TypeDef { name, typ })
        }

        Token::Let => {
            lexer.next();
            let name = lexer.parse_ident()?;

            let typ = match lexer.expect_peek()? {
                Token::Colon => {
                    lexer.next();
                    Some(parse_type(lexer)?)
                }
                _ => None,
            };

            lexer.parse_token(&Token::Assign)?;

            let expr = parse_expression(lexer)?;

            lexer.parse_token(&Token::Semicolon)?;

            Ok(Statement::VarDef { name, typ, expr })
        }

        Token::Import => {
            let idents = parse_list(
                lexer,
                &Token::Import,
                &Token::Comma,
                &Token::From,
                |lexer| {
                    let name = lexer.parse_ident()?;

                    match lexer.expect_peek()? {
                        Token::As => {
                            lexer.next();
                            let alias = lexer.parse_ident()?;
                            Ok(ImportIdentifier { name, alias })
                        }
                        _ => Ok(ImportIdentifier {
                            name: name.clone(),
                            alias: name,
                        }),
                    }
                },
            )?;

            let path = lexer.parse_string()?;

            lexer.parse_token(&Token::Semicolon)?;

            Ok(Statement::Import {
                path,
                imports: idents,
            })
        }

        Token::While => {
            lexer.next();
            let cond = parse_expression(lexer)?;
            let body = parse_block(lexer, true)?;
            Ok(Statement::While { cond, body })
        }

        Token::Loop => {
            lexer.next();
            let body = parse_block(lexer, true)?;
            Ok(Statement::Loop(body))
        }

        Token::Return => {
            lexer.next();
            let expr = if lexer.parse_token(&Token::Semicolon).is_ok() {
                None
            } else {
                Some(parse_expression(lexer)?)
            };
            lexer.parse_token(&Token::Semicolon)?;
            Ok(Statement::Return(expr))
        }

        _ => {
            let expr = parse_expression(lexer)?;

            if lexer.parse_token(&Token::Assign).is_ok() {
                let rhs = parse_expression(lexer)?;
                lexer.parse_token(&Token::Semicolon)?;
                Ok(Statement::Assign { lhs: expr, rhs })
            } else if lexer.parse_token(&Token::AddAssign).is_ok() {
                let rhs = parse_expression(lexer)?;
                lexer.parse_token(&Token::Semicolon)?;
                Ok(Statement::Assign {
                    lhs: expr.clone(),
                    rhs: Expression::Add(Box::new(expr), Box::new(rhs)),
                })
            } else if lexer.parse_token(&Token::SubAssign).is_ok() {
                let rhs = parse_expression(lexer)?;
                lexer.parse_token(&Token::Semicolon)?;
                Ok(Statement::Assign {
                    lhs: expr.clone(),
                    rhs: Expression::Sub(Box::new(expr), Box::new(rhs)),
                })
            } else if lexer.parse_token(&Token::MulAssign).is_ok() {
                let rhs = parse_expression(lexer)?;
                lexer.parse_token(&Token::Semicolon)?;
                Ok(Statement::Assign {
                    lhs: expr.clone(),
                    rhs: Expression::Mul(Box::new(expr), Box::new(rhs)),
                })
            } else if lexer.parse_token(&Token::DivAssign).is_ok() {
                let rhs = parse_expression(lexer)?;
                lexer.parse_token(&Token::Semicolon)?;
                Ok(Statement::Assign {
                    lhs: expr.clone(),
                    rhs: Expression::Div(Box::new(expr), Box::new(rhs)),
                })
            } else {
                lexer.parse_token(&Token::Semicolon)?;
                Ok(Statement::Expr(expr))
            }
        }
    }
}
