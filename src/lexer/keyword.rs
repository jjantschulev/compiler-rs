use super::lexer::ParseFromConstStr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Keyword {
    Import,
    From,
    Fn,
    Int,
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
}

impl ParseFromConstStr for Keyword {
    fn to_str(&self) -> &'static str {
        match self {
            Keyword::Import => "import",
            Keyword::From => "from",
            Keyword::Fn => "fn",
            Keyword::Int => "int",
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
        }
    }

    fn enumarate<'a>() -> &'a [Self]
    where
        Self: Sized,
    {
        &[
            Keyword::Import,
            Keyword::From,
            Keyword::Fn,
            Keyword::Int,
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
        ]
    }
}
