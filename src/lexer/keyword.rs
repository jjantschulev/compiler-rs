use super::lexer::ParseFromConstStr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Keyword {
    Import,
    From,
    Int,
    Uint,
    Float,
    Char,
    Bool,
    Void,
    Struct,
    If,
    Else,
    While,
    For,
    Return,
    Break,
    Continue,
    True,
    False,
    Loop,
    Type,
    Any,
    Let,
}

impl ParseFromConstStr for Keyword {
    fn to_str(&self) -> &'static str {
        match self {
            Keyword::Import => "import",
            Keyword::From => "from",
            Keyword::Int => "int",
            Keyword::Uint => "uint",
            Keyword::Float => "float",
            Keyword::Char => "char",
            Keyword::Bool => "bool",
            Keyword::Void => "void",
            Keyword::Struct => "struct",
            Keyword::If => "if",
            Keyword::Else => "else",
            Keyword::While => "while",
            Keyword::For => "for",
            Keyword::Return => "return",
            Keyword::Break => "break",
            Keyword::Continue => "continue",
            Keyword::True => "true",
            Keyword::False => "false",
            Keyword::Loop => "loop",
            Keyword::Type => "type",
            Keyword::Any => "any",
            Keyword::Let => "let",
        }
    }

    fn enumarate<'a>() -> &'a [Self]
    where
        Self: Sized,
    {
        &[
            Keyword::Import,
            Keyword::From,
            Keyword::Int,
            Keyword::Uint,
            Keyword::Float,
            Keyword::Char,
            Keyword::Bool,
            Keyword::Void,
            Keyword::Struct,
            Keyword::If,
            Keyword::Else,
            Keyword::While,
            Keyword::For,
            Keyword::Return,
            Keyword::Break,
            Keyword::Continue,
            Keyword::True,
            Keyword::False,
            Keyword::Loop,
            Keyword::Type,
            Keyword::Any,
            Keyword::Let,
        ]
    }
}
