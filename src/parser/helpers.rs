use crate::lexer::{lexer::Lexer, token_type::TokenType};

pub fn parse_list<T>(
    lexer: &mut Lexer,
    start: &TokenType,
    separator: &TokenType,
    end: &TokenType,
    parse_item: impl Fn(&mut Lexer) -> Result<T, ParseError>,
) -> Result<Vec<T>, ParseError> {
    let mut items = Vec::new();

    lexer.parse_token(start)?;

    loop {
        if lexer.parse_token(end).is_ok() {
            break;
        }

        items.push(parse_item(lexer)?);

        if lexer.parse_token(end).is_ok() {
            break;
        }

        lexer.parse_token(separator)?;
    }

    Ok(items)
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
    UnexpectedToken(TokenType),
    UnexpectedEOF(),
}
