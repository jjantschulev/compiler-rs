pub mod encoder;
pub mod little_endian_base_128;
pub struct WasmModule {
    pub types: Vec<FunctionType>,
    pub imports: Vec<Import>,
    pub exports: Vec<Export>,
    pub functions: Vec<Function>,
    pub start: Option<u32>,
}

pub enum NumType {
    I32,
    I64,
    F32,
    F64,
}

pub struct FunctionType {
    pub args: Vec<NumType>,
    pub ret: Vec<NumType>,
}

pub struct Import {
    pub module: String,
    pub name: String,
    pub typ: ImportType,
}

pub enum ImportType {
    Func(u32),
    Memory { min: u32, max: Option<u32> },
}

pub struct Export {
    pub name: String,
    pub typ: ExportType,
}

pub enum ExportType {
    Func(u32),
}

pub struct Function {
    pub type_idx: u32,
    pub locals: Vec<NumType>,
    pub body: Expression,
}

pub struct Expression {
    pub instructions: Vec<Instruction>,
}

pub enum Instruction {
    I32Const(i32),
    I64Const(i64),
    F32Const(f32),
    F64Const(f64),
    IntegerOp(IntegerOp),
    FloatOp(FloatOp),
    ConvertOp(ConvertOp),
    // RefOp(RefOp),
    VariableOp(VariableOp),
    ControlOp(ControlOp),
}

pub enum RefOp {
    Func(u32),
    IsNull,
    Null(u32),
}

pub enum VariableOp {
    LocalGet(u32),
    LocalSet(u32),
    LocalTee(u32),
    GlobalGet(u32),
    GlobalSet(u32),
}

pub enum ControlOp {
    Call(u32),
    Return,
    Nop,
}

pub enum IntegerType {
    I32,
    I64,
}

pub enum FloatType {
    F32,
    F64,
}

pub struct IntegerOp {
    pub op: IntegerOpType,
    pub typ: IntegerType,
}

pub struct FloatOp {
    pub op: FloatOpType,
    pub typ: FloatType,
}

pub enum IntegerOpType {
    // Unary Ops
    Clz,
    Ctz,
    Popcnt,

    // Test Ops
    Eqz,

    // Relational Ops
    Eq,
    Ne,
    LtS,
    LtU,
    GtS,
    GtU,
    LeS,
    LeU,
    GeS,
    GeU,

    // Bin Ops
    Add,
    Sub,
    Mul,
    DivS,
    DivU,
    RemS,
    RemU,
    And,
    Or,
    Xor,
    Shl,
    ShrU,
    ShrS,
    Rotl,
    Rotr,
}

pub enum FloatOpType {
    // Unary Ops
    Abs,
    Neg,
    Sqrt,
    Ceil,
    Floor,
    Trunc,
    Nearest,

    // Bin Ops
    Add,
    Sub,
    Mul,
    Div,
    Min,
    Max,
    Copysign,

    // Compare Ops
    Eq,
    Ne,
    Lt,
    Le,
    Ge,
    Gt,
}

pub enum ConvertOp {
    I32Extend8S,
    I32Extend16S,
    I64Extend8S,
    I64Extend16S,
    I64Extend32S,
    I32WrapI64,
    I64ExtendI32S,
    I64ExtendI32U,

    I32TruncF32S,
    I32TruncF32U,
    I32TruncF64S,
    I32TruncF64U,
    I64TruncF32S,
    I64TruncF32U,
    I64TruncF64S,
    I64TruncF64U,
    I32TruncSatF32S,
    I32TruncSatF32U,
    I32TruncSatF64S,
    I32TruncSatF64U,
    I64TruncSatF32S,
    I64TruncSatF32U,
    I64TruncSatF64S,
    I64TruncSatF64U,
    F32DemoteF64,
    F64PromoteF32,
    F32ConvertI32S,
    F32ConvertI32U,
    F32ConvertI64S,
    F32ConvertI64U,
    F64ConvertI32S,
    F64ConvertI32U,
    F64ConvertI64S,
    F64ConvertI64U,

    I32ReinterpretF32,
    I64ReinterpretF64,
    F32ReinterpretI32,
    F64ReinterpretI64,
}
