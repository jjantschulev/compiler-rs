use super::token_type::{Token, TokenType};

pub struct Lexer<'a> {
    input: &'a str,
    index: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input, index: 0 }
    }

    fn consume_until_match(&mut self, pattern: &str) {
        loop {
            let view = &self.input[self.index..];
            if view.starts_with(pattern) {
                self.index += pattern.len();
                break;
            }
            self.index += 1;
        }
    }

    fn consume_till_next_token(&mut self) {
        loop {
            let view = &self.input[self.index..];

            if view.starts_with("//") {
                self.index += 2;
                self.consume_until_match("\n");
                continue;
            }

            if view.starts_with("/*") {
                self.index += 2;
                self.consume_until_match("*/");
                continue;
            }

            match view.chars().next() {
                Some(c) if c.is_whitespace() => {
                    self.index += 1;
                    continue;
                }
                _ => break,
            }
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.consume_till_next_token();
        let view = &self.input[self.index..];

        let (next_tok, len) = TokenType::next_token(view)?;

        self.index += len;

        Some(Token {
            token: next_tok,
            raw: view[..len].to_string(),
            index: self.index - len,
        })
    }
}

pub trait ParseFromStr {
    fn parse_from_str(input: &str) -> Option<(Self, usize)>
    where
        Self: Sized;
}

pub trait ParseFromConstStr {
    fn to_str(&self) -> &'static str;
    fn enumarate<'a>() -> &'a [Self]
    where
        Self: Sized;

    fn parse_from_str(input: &str) -> Option<(Self, usize)>
    where
        Self: Sized + Clone,
    {
        for item in Self::enumarate() {
            let item_str = item.to_str();
            if input.starts_with(item_str) {
                return Some((item.clone(), item_str.len()));
            }
        }

        None
    }
}
