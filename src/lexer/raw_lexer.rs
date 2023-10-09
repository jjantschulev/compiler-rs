use super::token::Token;

#[derive(Debug, Clone)]
pub struct RawLexer<'a> {
    input: &'a str,
    index: usize,
}

impl<'a> RawLexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input, index: 0 }
    }

    // helper function to consume until the pattern is matched
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

    // helper function to consume all whitespace and comments
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

impl<'a> Iterator for RawLexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.consume_till_next_token();

        let view = &self.input[self.index..];

        let (next_tok, len) = Token::parse_from_str(view)?;

        self.index += len;

        Some(next_tok)
    }
}
