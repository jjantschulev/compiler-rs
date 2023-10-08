use compiler_rs::{lexer::lexer::Lexer, parser::types::parse_type};
use std::fs::read_to_string;

fn main() {
    let source = read_to_string("tests/sample.jj").unwrap();

    let mut lexer = Lexer::new(&source);

    dbg! { parse_type(&mut lexer) };
}
