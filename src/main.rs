use compiler_rs::lexer::lexer::Lexer;
use std::fs::read_to_string;

fn main() {
    let source = read_to_string("tests/sample.jj").unwrap();

    let lexer = Lexer::new(&source);

    for token in lexer {
        println!("{:?}", token);
    }
}
