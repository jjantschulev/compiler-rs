use std::rc::Rc;

pub enum Type {
    Int,
    Uint,
    Float,
    Char,
    Bool,
    Void,
    Ptr(Rc<Type>),
    Array { element: Rc<Type>, len: usize },
    Struct { fields: Vec<(String, Rc<Type>)> },
}

// pub struct ImportStatement {
//     path: Token,
// }

// enum Statement {
//     Expr(Expr)
//     VarDecl
// }

// struct Block {
//     statements: Vec<Statement>,
// }

// enum Ast {
//     Block(Block),
// }
