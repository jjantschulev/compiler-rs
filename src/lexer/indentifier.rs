use super::lexer::ParseFromStr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ident(String);

impl ParseFromStr for Ident {
    fn parse_from_str(input: &str) -> Option<(Self, usize)> {
        if let Some((ident, len)) = parse_identifier(input) {
            return Some((Self(ident), len));
        }

        None
    }
}

fn parse_identifier(input: &str) -> Option<(String, usize)> {
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

    Some((input[..len].to_string(), len))
}
