use super::{
    indentifier::Ident,
    keyword::Keyword,
    lexer::{ParseFromConstStr, ParseFromStr},
    literal::Literal,
    operator::Operator,
    special_char::SpecialChar,
};

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Literal(Literal),
    Keyword(Keyword),
    Ident(Ident),
    Operator(Operator),
    SpecialChar(SpecialChar),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub index: usize,
    pub raw: String,
    pub token: TokenType,
}

impl TokenType {
    pub fn next_token(input: &str) -> Option<(Self, usize)> {
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

        if let Some((tok, len)) = Ident::parse_from_str(input) {
            return Some((TokenType::Ident(tok), len));
        }

        None
    }
}
