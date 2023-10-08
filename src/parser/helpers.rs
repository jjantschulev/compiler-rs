use crate::lexer::{lexer::Lexer, token_type::TokenType};

pub fn parse_list<T>(
    lexer: &mut Lexer,
    start: &TokenType,
    separator: &TokenType,
    end: &TokenType,
    parse_item: impl Fn(&mut Lexer) -> Result<T, ParseError>,
) -> Result<Vec<T>, ParseError> {
    let mut items = Vec::new();

    lexer.expect_token(start)?;

    loop {
        if lexer.expect_token(end).is_ok() {
            break;
        }

        items.push(parse_item(lexer)?);

        if lexer.expect_token(end).is_ok() {
            break;
        }

        lexer.expect_token(separator)?;
    }

    Ok(items)
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
    UnexpectedToken(TokenType),
    UnexpectedEOF(),
}
