use super::literal::{
    parse_char_literal, parse_float_literal, parse_int_literal, parse_string_literal,
};

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Identifier(String),

    // Literals
    Integer(i64),
    Float(f64),
    String(String),
    Bool(bool),
    Char(char),

    // Misc Keywords
    Import,
    From,
    Let,

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

impl TokenType {
    pub fn parse_from_str(input: &str) -> Option<(TokenType, usize)> {
        TokenType::parse_identifier(input)
            .or_else(|| TokenType::parse_literal(input))
            .or_else(|| TokenType::parse_basic_token(input))
    }

    pub fn parse_identifier(input: &str) -> Option<(TokenType, usize)> {
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
        if let Some(keyword) = TokenType::match_basic_token(slice) {
            return Some((keyword, len));
        }

        Some((TokenType::Identifier(slice.to_string()), len))
    }

    pub fn parse_literal(input: &str) -> Option<(TokenType, usize)> {
        if let Some((number, len)) = parse_int_literal(input) {
            return Some((TokenType::Integer(number), len));
        }

        if let Some((number, len)) = parse_float_literal(input) {
            return Some((TokenType::Float(number), len));
        }

        if let Some((string, len)) = parse_string_literal(input) {
            return Some((TokenType::String(string), len));
        }

        if let Some((c, len)) = parse_char_literal(input) {
            return Some((TokenType::Char(c), len));
        }

        if input.starts_with("true") {
            return Some((TokenType::Bool(true), 4));
        }

        if input.starts_with("false") {
            return Some((TokenType::Bool(false), 5));
        }

        None
    }

    fn parse_basic_token(input: &str) -> Option<(TokenType, usize)> {
        let mut max_basic_token_len = std::cmp::min(8, input.len());
        if max_basic_token_len == 0 {
            return None;
        }
        loop {
            let slice = &input[..max_basic_token_len];
            if let Some(token) = TokenType::match_basic_token(slice) {
                break Some((token, max_basic_token_len));
            }
            // We go backwards to match the longest token first
            max_basic_token_len -= 1;
            if max_basic_token_len == 0 {
                break None;
            }
        }
    }

    fn match_basic_token(s: &str) -> Option<TokenType> {
        match s {
            // Misc Keywords
            "import" => Some(TokenType::Import),
            "from" => Some(TokenType::From),
            "let" => Some(TokenType::Let),

            // Control Flow
            "if" => Some(TokenType::If),
            "else" => Some(TokenType::Else),
            "while" => Some(TokenType::While),
            "for" => Some(TokenType::For),
            "return" => Some(TokenType::Return),
            "break" => Some(TokenType::Break),
            "continue" => Some(TokenType::Continue),
            "loop" => Some(TokenType::Loop),

            // Built in types
            "type" => Some(TokenType::Type),
            "int" => Some(TokenType::IntType),
            "float" => Some(TokenType::FloatType),
            "string" => Some(TokenType::StringType),
            "bool" => Some(TokenType::BoolType),
            "char" => Some(TokenType::CharType),
            "void" => Some(TokenType::VoidType),

            // Operators
            "=" => Some(TokenType::Assign),

            // Math
            "+" => Some(TokenType::Add),
            "-" => Some(TokenType::Sub),
            "/" => Some(TokenType::Div),
            "*" => Some(TokenType::Mul),
            "%" => Some(TokenType::Mod),

            // Comparison
            "==" => Some(TokenType::Equal),
            ">=" => Some(TokenType::GreaterEqual),
            ">" => Some(TokenType::GreaterThan),
            "<=" => Some(TokenType::LessEqual),
            "<" => Some(TokenType::LessThan),

            // Logical
            "&&" => Some(TokenType::And),
            "and" => Some(TokenType::And),
            "||" => Some(TokenType::Or),
            "or" => Some(TokenType::Or),
            "!" => Some(TokenType::Not),
            "not" => Some(TokenType::Not),

            // Special Characters
            "(" => Some(TokenType::LParen),
            ")" => Some(TokenType::RParen),
            "[" => Some(TokenType::LBracket),
            "]" => Some(TokenType::RBracket),
            "{" => Some(TokenType::LBrace),
            "}" => Some(TokenType::RBrace),
            "," => Some(TokenType::Comma),
            ";" => Some(TokenType::Semicolon),
            ":" => Some(TokenType::Colon),
            "." => Some(TokenType::Dot),
            "&" => Some(TokenType::Ref),
            "=>" => Some(TokenType::FuncArrow),

            _ => None,
        }
    }
}
