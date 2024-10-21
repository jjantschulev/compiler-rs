use compiler_rs::{
    lexer::lexer::Lexer,
    parser::statements::parse_block,
    type_checker::{statements::check_block, Scope},
    wasm::{self, encoder::EncodesToWasm, WasmModule},
};
use std::fs::read_to_string;

fn main() {
    let source = read_to_string("tests/tree.jj").unwrap();

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

    let wasm_module = make_wasm_module();

    let mut bytes = vec![];
    wasm_module.encode_to_wasm(&mut bytes);
    std::fs::write("tests/program.wasm", bytes).unwrap();
}

fn make_wasm_module() -> WasmModule {
    use wasm::*;
    WasmModule {
        start: Some(2),
        types: vec![
            FunctionType {
                args: vec![NumType::I32],
                ret: vec![],
            },
            FunctionType {
                args: vec![NumType::I32, NumType::I32],
                ret: vec![NumType::I32],
            },
            FunctionType {
                args: vec![],
                ret: vec![],
            },
        ],
        imports: vec![Import {
            module: "env".to_string(),
            name: "print_int".to_string(),
            typ: ImportType::Func(0),
        }],
        exports: vec![Export {
            name: "add_two_int_32".to_string(),
            typ: ExportType::Func(1),
        }],
        functions: vec![
            Function {
                type_idx: 1,
                locals: vec![NumType::I32],
                body: Expression {
                    instructions: vec![
                        Instruction::VariableOp(VariableOp::LocalGet(0)),
                        Instruction::VariableOp(VariableOp::LocalGet(1)),
                        Instruction::IntegerOp(IntegerOp {
                            op: IntegerOpType::Add,
                            typ: IntegerType::I32,
                        }),
                        Instruction::VariableOp(VariableOp::LocalTee(2)),
                        Instruction::ControlOp(ControlOp::Call(0)),
                        Instruction::VariableOp(VariableOp::LocalGet(2)),
                    ],
                },
            },
            Function {
                type_idx: 2,
                locals: vec![],
                body: Expression {
                    instructions: vec![
                        Instruction::I32Const(42),
                        Instruction::ControlOp(ControlOp::Call(0)),
                    ],
                },
            },
        ],
    }
}
