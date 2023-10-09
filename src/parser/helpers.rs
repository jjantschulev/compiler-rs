use std::collections::HashMap;

use crate::lexer::{lexer::Lexer, token::Token};

pub fn parse_list<T>(
    lexer: &mut Lexer,
    start: &Token,
    separator: &Token,
    end: &Token,
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

pub fn build_hashmap_from_entries<T>(
    entries: Vec<(String, T)>,
) -> Result<HashMap<String, T>, ParseError> {
    let mut map = HashMap::new();

    for (key, value) in entries {
        if map.contains_key(&key) {
            return Err(ParseError::Unknown("Duplicate key".to_string()));
        }
        map.insert(key, value);
    }

    Ok(map)
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
    UnexpectedToken(Token),
    UnexpectedEOF(),
    Unknown(String),
}
