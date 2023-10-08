use crate::lexer::{lexer::Lexer, token_type::TokenType};

use super::{
    expressions::{parse_expression, Expression},
    helpers::{parse_list, ParseError},
    types::{parse_type, Type},
};

type Block = Vec<Statement>;

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
    For {
        init: Option<Box<Statement>>,
        cond: Option<Expression>,
        step: Option<Expression>,
        body: Block,
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
        lexer.expect_token(&TokenType::LBrace)?;
    }

    loop {
        if enclosed_by_brackets && lexer.expect_token(&TokenType::RBrace).is_ok() {
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
        TokenType::Type => {
            lexer.next();
            let name = match lexer.expect_next()? {
                TokenType::Identifier(name) => name,
                tok => return Err(ParseError::UnexpectedToken(tok.clone())),
            };
            lexer.expect_token(&TokenType::Assign)?;
            let typ = parse_type(lexer)?;
            lexer.expect_token(&TokenType::Semicolon)?;
            Ok(Statement::TypeDef { name, typ })
        }

        TokenType::Let => {
            lexer.next();
            let name = match lexer.expect_next()? {
                TokenType::Identifier(name) => name,
                tok => return Err(ParseError::UnexpectedToken(tok.clone())),
            };

            let typ = match lexer.expect_peek()? {
                TokenType::Colon => {
                    lexer.next();
                    Some(parse_type(lexer)?)
                }
                _ => None,
            };

            lexer.expect_token(&TokenType::Assign)?;

            let expr = parse_expression(lexer)?;

            lexer.expect_token(&TokenType::Semicolon)?;

            Ok(Statement::VarDef { name, typ, expr })
        }

        TokenType::Import => {
            let idents = parse_list(
                lexer,
                &TokenType::Import,
                &TokenType::Comma,
                &TokenType::From,
                |lexer| {
                    let name = match lexer.expect_next()? {
                        TokenType::Identifier(name) => Ok(name),
                        tok => Err(ParseError::UnexpectedToken(tok.clone())),
                    }?;

                    match lexer.expect_peek()? {
                        TokenType::As => {
                            lexer.next();
                            let alias = match lexer.expect_next()? {
                                TokenType::Identifier(alias) => Ok(alias),
                                tok => Err(ParseError::UnexpectedToken(tok.clone())),
                            }?;
                            Ok(ImportIdentifier { name, alias })
                        }
                        _ => Ok(ImportIdentifier {
                            name: name.clone(),
                            alias: name,
                        }),
                    }
                },
            )?;

            let path = match lexer.expect_next()? {
                TokenType::String(string) => string,
                tok => return Err(ParseError::UnexpectedToken(tok.clone())),
            };

            lexer.expect_token(&TokenType::Semicolon)?;

            Ok(Statement::Import {
                path,
                imports: idents,
            })
        }

        TokenType::While => {
            lexer.next();
            let cond = parse_expression(lexer)?;
            let body = parse_block(lexer, true)?;
            Ok(Statement::While { cond, body })
        }

        TokenType::Loop => {
            lexer.next();
            let body = parse_block(lexer, true)?;
            Ok(Statement::Loop(body))
        }

        _ => {
            let expr = parse_expression(lexer)?;
            lexer.expect_token(&TokenType::Semicolon)?;
            Ok(Statement::Expr(expr))
        }
    }
}
