use super::{
    indentifier::parse_identifier,
    keyword::Keyword,
    lexer::{ParseFromConstStr, ParseFromStr},
    literal::{AbstractLiteral, Literal},
    operator::Operator,
    special_char::SpecialChar,
};

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Literal(Literal),
    Keyword(Keyword),
    Ident(String),
    Operator(Operator),
    SpecialChar(SpecialChar),
}

#[derive(Debug, Clone, PartialEq)]
pub enum AbstractTokenType {
    Literal(AbstractLiteral),
    AnyLiteral,
    Keyword(Keyword),
    AnyKeyword,
    Ident,
    Operator(Operator),
    AnyOperator,
    SpecialChar(SpecialChar),
    AnySpecialChar,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub index: usize,
    pub raw: String,
    pub token: TokenType,
}

impl TokenType {
    pub fn next_token(input: &str) -> Option<(Self, usize)> {
        if let Some((tok, len)) = parse_identifier(input) {
            return Some((TokenType::Ident(tok), len));
        }

        if let Some((tok, len)) = Keyword::parse_from_str(input) {
            return Some((TokenType::Keyword(tok), len));
        }

        if let Some((tok, len)) = Operator::parse_from_str(input) {
            return Some((TokenType::Operator(tok), len));
        }

        if let Some((tok, len)) = SpecialChar::parse_from_str(input) {
            return Some((TokenType::SpecialChar(tok), len));
        }

        if let Some((tok, len)) = Literal::parse_from_str(input) {
            return Some((TokenType::Literal(tok), len));
        }

        None
    }

    pub fn is_of_type(&self, typ: &AbstractTokenType) -> bool {
        match typ {
            AbstractTokenType::AnyKeyword => match self {
                TokenType::Keyword(_) => true,
                _ => false,
            },
            AbstractTokenType::AnyLiteral => match self {
                TokenType::Literal(_) => true,
                _ => false,
            },
            AbstractTokenType::AnyOperator => match self {
                TokenType::Operator(_) => true,
                _ => false,
            },
            AbstractTokenType::AnySpecialChar => match self {
                TokenType::SpecialChar(_) => true,
                _ => false,
            },
            AbstractTokenType::Ident => match self {
                TokenType::Ident(_) => true,
                _ => false,
            },
            AbstractTokenType::Keyword(keyword) => match self {
                TokenType::Keyword(tok) => tok == keyword,
                _ => false,
            },
            AbstractTokenType::Literal(literal) => match self {
                TokenType::Literal(tok) => match (tok, literal) {
                    (Literal::Integer(_), AbstractLiteral::Integer) => true,
                    (Literal::Float(_), AbstractLiteral::Float) => true,
                    (Literal::String(_), AbstractLiteral::String) => true,
                    (Literal::Boolean(_), AbstractLiteral::Boolean) => true,
                    (Literal::Char(_), AbstractLiteral::Char) => true,
                    _ => false,
                },
                _ => false,
            },
            AbstractTokenType::Operator(operator) => match self {
                TokenType::Operator(tok) => tok == operator,
                _ => false,
            },
            AbstractTokenType::SpecialChar(special_char) => match self {
                TokenType::SpecialChar(tok) => tok == special_char,
                _ => false,
            },
        }
    }
}
