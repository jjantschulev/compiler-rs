use super::{keyword, lexer::ParseFromConstStr};

pub fn parse_identifier(input: &str) -> Option<(String, usize)> {
    let mut iterator = input.chars();

    let mut len = 1;
    let first = iterator.next()?;
    if !first.is_alphabetic() && first != '_' {
        return None;
    }

    while iterator
        .next()
        .is_some_and(|c| c.is_alphanumeric() || c == '_')
    {
        len += 1;
    }

    // Make sure that this is not a keyword
    for keyword in keyword::Keyword::enumarate().iter() {
        if keyword.to_str() == &input[..len] {
            return None;
        }
    }

    Some((input[..len].to_string(), len))
}
