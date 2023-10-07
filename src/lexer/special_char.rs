use super::lexer::ParseFromConstStr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpecialChar {
    LParen,
    RParen,
    LBracket,
    RBracket,
    LBrace,
    RBrace,
    Comma,
    Semicolon,
    Colon,
    Dot,
    Ref,
}

impl ParseFromConstStr for SpecialChar {
    fn to_str(&self) -> &'static str {
        match self {
            SpecialChar::LParen => "(",
            SpecialChar::RParen => ")",
            SpecialChar::LBracket => "[",
            SpecialChar::RBracket => "]",
            SpecialChar::LBrace => "{",
            SpecialChar::RBrace => "}",
            SpecialChar::Comma => ",",
            SpecialChar::Semicolon => ";",
            SpecialChar::Colon => ":",
            SpecialChar::Dot => ".",
            SpecialChar::Ref => "&",
        }
    }

    fn enumarate<'a>() -> &'a [Self]
    where
        Self: Sized,
    {
        &[
            SpecialChar::LParen,
            SpecialChar::RParen,
            SpecialChar::LBracket,
            SpecialChar::RBracket,
            SpecialChar::LBrace,
            SpecialChar::RBrace,
            SpecialChar::Comma,
            SpecialChar::Semicolon,
            SpecialChar::Colon,
            SpecialChar::Dot,
            SpecialChar::Ref,
        ]
    }
}
