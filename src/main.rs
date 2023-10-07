use compiler_rs::{
    lexer::lexer::Lexer,
    parser::ast::{Parsable, Statement},
};
use std::fs::read_to_string;

fn main() {
    let source = read_to_string("tests/sample.jj").unwrap();

    let mut lexer = Lexer::new(&source).peekable();

    // for token in lexer.cloned() {
    //     println!("{:?}", token);
    // }

    let statement = Statement::parse(&mut lexer);

    println!("{:?}", statement);
}
