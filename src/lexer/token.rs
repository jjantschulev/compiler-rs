use super::literal::{
    parse_char_literal, parse_float_literal, parse_int_literal, parse_string_literal,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Identifier(String),

    // Literals
    Integer(i64),
    Float(f64),
    String(String),
    Bool(bool),
    Char(char),

    // Misc Keywords
    Import,
    As,
    From,
    Let,
    Yeet,

    // Control Flow
    If,
    Else,
    While,
    For,
    Return,
    Break,
    Continue,
    Loop,

    // Built in types
    Type,
    IntType,
    FloatType,
    StringType,
    BoolType,
    CharType,
    VoidType,

    // Operators
    Assign,

    // Math
    Add,
    Sub,
    Div,
    Mul,
    AddAssign,
    SubAssign,
    DivAssign,
    MulAssign,
    Mod,

    // Comparison
    Equal,
    GreaterEqual,
    GreaterThan,
    LessEqual,
    LessThan,

    // Logical
    And,
    Or,
    Not,

    // Special Characters
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
    FuncArrow,
}

impl Token {
    pub fn parse_from_str(input: &str) -> Option<(Token, usize)> {
        Token::parse_identifier(input)
            .or_else(|| Token::parse_literal(input))
            .or_else(|| Token::parse_basic_token(input))
    }

    pub fn parse_identifier(input: &str) -> Option<(Token, usize)> {
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

        let slice = &input[..len];

        // Check if the identifier is a keyword, if so return None
        if let Some(keyword) = Token::match_basic_token(slice) {
            return Some((keyword, len));
        }

        Some((Token::Identifier(slice.to_string()), len))
    }

    pub fn parse_literal(input: &str) -> Option<(Token, usize)> {
        if let Some((number, len)) = parse_float_literal(input) {
            return Some((Token::Float(number), len));
        }

        if let Some((number, len)) = parse_int_literal(input) {
            return Some((Token::Integer(number), len));
        }

        if let Some((string, len)) = parse_string_literal(input) {
            return Some((Token::String(string), len));
        }

        if let Some((c, len)) = parse_char_literal(input) {
            return Some((Token::Char(c), len));
        }

        if input.starts_with("true") {
            return Some((Token::Bool(true), 4));
        }

        if input.starts_with("false") {
            return Some((Token::Bool(false), 5));
        }

        None
    }

    fn parse_basic_token(input: &str) -> Option<(Token, usize)> {
        let mut max_basic_token_len = std::cmp::min(8, input.len());
        if max_basic_token_len == 0 {
            return None;
        }
        loop {
            let slice = &input[..max_basic_token_len];
            if let Some(token) = Token::match_basic_token(slice) {
                break Some((token, max_basic_token_len));
            }
            // We go backwards to match the longest token first
            max_basic_token_len -= 1;
            if max_basic_token_len == 0 {
                break None;
            }
        }
    }

    fn match_basic_token(s: &str) -> Option<Token> {
        match s {
            // Misc Keywords
            "import" => Some(Token::Import),
            "from" => Some(Token::From),
            "let" => Some(Token::Let),
            "as" => Some(Token::As),
            "yeet" => Some(Token::Yeet),

            // Control Flow
            "if" => Some(Token::If),
            "else" => Some(Token::Else),
            "while" => Some(Token::While),
            "for" => Some(Token::For),
            "return" => Some(Token::Return),
            "break" => Some(Token::Break),
            "continue" => Some(Token::Continue),
            "loop" => Some(Token::Loop),

            // Built in types
            "type" => Some(Token::Type),
            "int" => Some(Token::IntType),
            "float" => Some(Token::FloatType),
            "string" => Some(Token::StringType),
            "bool" => Some(Token::BoolType),
            "char" => Some(Token::CharType),
            "void" => Some(Token::VoidType),

            // Operators
            "=" => Some(Token::Assign),

            // Math
            "+" => Some(Token::Add),
            "-" => Some(Token::Sub),
            "/" => Some(Token::Div),
            "*" => Some(Token::Mul),
            "%" => Some(Token::Mod),

            "+=" => Some(Token::AddAssign),
            "-=" => Some(Token::SubAssign),
            "/=" => Some(Token::DivAssign),
            "*=" => Some(Token::MulAssign),

            // Comparison
            "==" => Some(Token::Equal),
            ">=" => Some(Token::GreaterEqual),
            ">" => Some(Token::GreaterThan),
            "<=" => Some(Token::LessEqual),
            "<" => Some(Token::LessThan),

            // Logical
            "&&" => Some(Token::And),
            "and" => Some(Token::And),
            "||" => Some(Token::Or),
            "or" => Some(Token::Or),
            "!" => Some(Token::Not),
            "not" => Some(Token::Not),

            // Special Characters
            "(" => Some(Token::LParen),
            ")" => Some(Token::RParen),
            "[" => Some(Token::LBracket),
            "]" => Some(Token::RBracket),
            "{" => Some(Token::LBrace),
            "}" => Some(Token::RBrace),
            "," => Some(Token::Comma),
            ";" => Some(Token::Semicolon),
            ":" => Some(Token::Colon),
            "." => Some(Token::Dot),
            "&" => Some(Token::Ref),
            "=>" => Some(Token::FuncArrow),

            _ => None,
        }
    }
}
