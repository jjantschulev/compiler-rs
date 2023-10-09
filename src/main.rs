use compiler_rs::{
    lexer::lexer::Lexer,
    parser::statements::parse_block,
    type_checker::{statements::check_block, Scope},
};
use std::fs::read_to_string;

fn main() {
    let source = read_to_string("tests/sample.jj").unwrap();

    let mut lexer = Lexer::new(&source);

    let program = parse_block(&mut lexer, false);
    dbg! { &program };
    let program = program.expect("Failed to parse program");

    let mut scope = Scope::new();
    check_block(&program, &mut scope).unwrap();

    for token in lexer {
        println!("{:?}", token);
    }

    dbg!("Global Scope:", scope);
}
