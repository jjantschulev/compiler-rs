use compiler_rs::{lexer::lexer::Lexer, parser::statements::parse_block};
use std::fs::read_to_string;

fn main() {
    let source = read_to_string("tests/sample.jj").unwrap();

    let mut lexer = Lexer::new(&source);

    dbg! { parse_block(&mut lexer, false).unwrap() };
}
