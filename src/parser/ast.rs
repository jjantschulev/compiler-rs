use std::{iter::Peekable, rc::Rc};

use crate::lexer::{
    keyword::Keyword,
    lexer::Lexer,
    literal::Literal,
    special_char::SpecialChar,
    token_type::{AbstractTokenType, Token, TokenType},
};

pub enum Type {
    Int,
    Uint,
    Float,
    Char,
    Bool,
    Void,
    Ptr(Rc<Type>),
    Array { element: Rc<Type>, len: usize },
    Struct { fields: Vec<(String, Rc<Type>)> },
    Function { args: Vec<Rc<Type>>, ret: Rc<Type> },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    ImportStatement { path: String, imports: Vec<String> },
}

impl Parsable for Statement {
    fn parse(lexer: &mut Peekable<Lexer>) -> Result<Self, ParseError> {
        match lexer.next().map(|t| t.token) {
            None => return Err(ParseError::new("Unexpected EOF".to_string())),
            Some(TokenType::Keyword(Keyword::Import)) => {
                let idents = parse_ident_list(lexer)?;
                expect_token(lexer, AbstractTokenType::Keyword(Keyword::From))?;

                match lexer.next() {
                    Some(Token {
                        token: TokenType::Literal(Literal::String(path)),
                        ..
                    }) => Ok(Statement::ImportStatement {
                        path,
                        imports: idents,
                    }),
                    tok => Err(ParseError::new(format!(
                        "Unexpected token {:?}, expected string literal",
                        tok
                    ))),
                }
            }
            tok => {
                return Err(ParseError::new(format!(
                    "Unexpected token {:?}, expected import statement",
                    tok
                )))
            }
        }
    }
}

fn expect_token(
    lexer: &mut Peekable<Lexer>,
    token: AbstractTokenType,
) -> Result<Token, ParseError> {
    match lexer.next() {
        None => Err(ParseError::new("Unexpected EOF".to_string())),
        Some(tok) => {
            if tok.token.is_of_type(&token) {
                Ok(tok)
            } else {
                Err(ParseError::new(format!(
                    "Unexpected token {:?}, expected {:?}",
                    tok.token, token
                )))
            }
        }
    }
}

fn parse_ident_list(lexer: &mut Peekable<Lexer>) -> Result<Vec<String>, ParseError> {
    let mut idents = Vec::new();

    loop {
        match lexer.peek() {
            None => return Err(ParseError::new("Unexpected EOF".to_string())),
            Some(tok) => match &tok.token {
                TokenType::Ident(ident) => {
                    idents.push(ident.clone());
                    lexer.next();
                }
                TokenType::SpecialChar(SpecialChar::Comma) => {
                    lexer.next();
                }
                _ => break,
            },
        }
    }

    Ok(idents)
}

// enum Statement {
//     Expr(Expr)
//     VarDecl
// }

// struct Block {
//     statements: Vec<Statement>,
// }

// enum Ast {
//     Block(Block),
// }

pub trait Parsable {
    fn parse(lexer: &mut Peekable<Lexer>) -> Result<Self, ParseError>
    where
        Self: Sized;
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParseError {
    message: String,
}

impl ParseError {
    pub fn new(message: String) -> Self {
        Self { message }
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}
