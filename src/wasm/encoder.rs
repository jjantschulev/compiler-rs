use super::little_endian_base_128::EncodesToLeb128;
use super::*;

pub trait EncodesToWasm {
    fn encode_to_wasm(&self, output: &mut Vec<u8>);
}

impl EncodesToWasm for WasmModule {
    fn encode_to_wasm(&self, output: &mut Vec<u8>) {
        output.extend([0x00, 0x61, 0x73, 0x6d]); // magic number
        output.extend([0x01, 0x00, 0x00, 0x00]); // version

        WasmModule::encode_section(output, 0x01, &self.types);
        WasmModule::encode_section(output, 0x02, &self.imports);
        WasmModule::encode_section(
            output,
            0x03,
            &self
                .functions
                .iter()
                .map(|f| f.type_idx)
                .collect::<Vec<_>>(),
        );
        WasmModule::encode_section(output, 0x07, &self.exports);

        if let Some(start) = self.start {
            output.push(0x08); // start section
            output.push(0x01); // start section size
            start.encode_to_leb128(output);
        }

        WasmModule::encode_section(output, 0x0a, &self.functions);
    }
}

impl WasmModule {
    fn encode_section<T: EncodesToWasm>(output: &mut Vec<u8>, code: u8, section: &[T]) {
        output.push(code);
        let mut section_bytes = Vec::new();
        section.len().encode_to_leb128(&mut section_bytes);
        for item in section {
            item.encode_to_wasm(&mut section_bytes);
        }
        section_bytes.len().encode_to_leb128(output);
        output.extend(section_bytes);
    }
}

impl EncodesToWasm for u32 {
    fn encode_to_wasm(&self, output: &mut Vec<u8>) {
        self.encode_to_leb128(output);
    }
}

impl EncodesToWasm for FunctionType {
    fn encode_to_wasm(&self, output: &mut Vec<u8>) {
        output.push(0x60); // function type
        self.args.len().encode_to_leb128(output);
        for param in self.args.iter() {
            param.encode_to_wasm(output);
        }
        self.ret.len().encode_to_leb128(output);
        for result in self.ret.iter() {
            result.encode_to_wasm(output);
        }
    }
}

impl EncodesToWasm for NumType {
    fn encode_to_wasm(&self, output: &mut Vec<u8>) {
        output.push(match self {
            NumType::I32 => 0x7f,
            NumType::I64 => 0x7e,
            NumType::F32 => 0x7d,
            NumType::F64 => 0x7c,
        });
    }
}

impl EncodesToWasm for Import {
    fn encode_to_wasm(&self, output: &mut Vec<u8>) {
        self.module.len().encode_to_leb128(output);
        output.extend(self.module.as_bytes());
        self.name.len().encode_to_leb128(output);
        output.extend(self.name.as_bytes());
        self.typ.encode_to_wasm(output);
    }
}

impl EncodesToWasm for ImportType {
    fn encode_to_wasm(&self, output: &mut Vec<u8>) {
        match self {
            ImportType::Func(index) => {
                output.push(0x00); // func type magic number
                index.encode_to_leb128(output);
            }
            ImportType::Memory { min, max } => {
                output.push(0x02); // memory type magic number
                let flags = if max.is_some() { 0x01 } else { 0x00 };
                output.push(flags);
                min.encode_to_leb128(output);
                if let Some(max) = max {
                    max.encode_to_leb128(output);
                }
            }
        }
    }
}

impl EncodesToWasm for Export {
    fn encode_to_wasm(&self, output: &mut Vec<u8>) {
        self.name.len().encode_to_leb128(output);
        output.extend(self.name.as_bytes());
        self.typ.encode_to_wasm(output);
    }
}

impl EncodesToWasm for ExportType {
    fn encode_to_wasm(&self, output: &mut Vec<u8>) {
        match self {
            ExportType::Func(index) => {
                output.push(0x00); // func type magic number
                index.encode_to_leb128(output);
            }
        }
    }
}

impl EncodesToWasm for Function {
    fn encode_to_wasm(&self, output: &mut Vec<u8>) {
        let mut body_bytes = Vec::new();

        self.locals.len().encode_to_leb128(&mut body_bytes);
        for local in self.locals.iter() {
            body_bytes.push(0x01); // a single local
            local.encode_to_wasm(&mut body_bytes);
        }

        self.body.encode_to_wasm(&mut body_bytes);

        body_bytes.len().encode_to_leb128(output);
        output.extend(body_bytes);
    }
}

impl EncodesToWasm for Expression {
    fn encode_to_wasm(&self, output: &mut Vec<u8>) {
        for instruction in self.instructions.iter() {
            instruction.encode_to_wasm(output);
        }
        output.push(0x0b); // end
    }
}

impl EncodesToWasm for Instruction {
    fn encode_to_wasm(&self, output: &mut Vec<u8>) {
        match self {
            Instruction::I32Const(constant) => {
                output.push(0x41);
                (*constant).encode_to_leb128(output);
            }
            Instruction::I64Const(constant) => {
                output.push(0x42);
                (*constant).encode_to_leb128(output);
            }
            Instruction::F32Const(constant) => {
                output.push(0x43);
                output.extend_from_slice(&constant.to_le_bytes());
            }
            Instruction::F64Const(constant) => {
                output.push(0x44);
                output.extend_from_slice(&constant.to_le_bytes());
            }
            Instruction::IntegerOp(op) => op.encode_to_wasm(output),
            Instruction::FloatOp(op) => op.encode_to_wasm(output),
            Instruction::ConvertOp(op) => op.encode_to_wasm(output),
            Instruction::VariableOp(op) => op.encode_to_wasm(output),
            Instruction::ControlOp(op) => op.encode_to_wasm(output),
        }
    }
}

impl EncodesToWasm for ControlOp {
    fn encode_to_wasm(&self, output: &mut Vec<u8>) {
        match self {
            ControlOp::Call(index) => {
                output.push(0x10);
                index.encode_to_leb128(output);
            }
            ControlOp::Return => output.push(0x0F),
            ControlOp::Nop => output.push(0x01),
        }
    }
}

impl EncodesToWasm for VariableOp {
    fn encode_to_wasm(&self, output: &mut Vec<u8>) {
        let (opcode, index) = match self {
            VariableOp::LocalGet(index) => (0x20, index),
            VariableOp::LocalSet(index) => (0x21, index),
            VariableOp::LocalTee(index) => (0x22, index),
            VariableOp::GlobalGet(index) => (0x23, index),
            VariableOp::GlobalSet(index) => (0x24, index),
        };
        output.push(opcode);
        index.encode_to_leb128(output);
    }
}

impl EncodesToWasm for IntegerOp {
    fn encode_to_wasm(&self, output: &mut Vec<u8>) {
        use IntegerOpType::*;
        use IntegerType::*;
        let byte = match (&self.op, &self.typ) {
            // Unary Ops
            (Clz, I32) => 0x67,
            (Ctz, I32) => 0x68,
            (Popcnt, I32) => 0x69,
            (Clz, I64) => 0x79,
            (Ctz, I64) => 0x7A,
            (Popcnt, I64) => 0x7B,

            // Test Ops
            (Eqz, I32) => 0x45,
            (Eqz, I64) => 0x50,

            // Relational Ops
            (Eq, I32) => 0x46,
            (Ne, I32) => 0x47,
            (LtS, I32) => 0x48,
            (LtU, I32) => 0x49,
            (GtS, I32) => 0x4A,
            (GtU, I32) => 0x4B,
            (LeS, I32) => 0x4C,
            (LeU, I32) => 0x4D,
            (GeS, I32) => 0x4E,
            (GeU, I32) => 0x4F,

            (Eq, I64) => 0x51,
            (Ne, I64) => 0x52,
            (LtS, I64) => 0x53,
            (LtU, I64) => 0x54,
            (GtS, I64) => 0x55,
            (GtU, I64) => 0x56,
            (LeS, I64) => 0x57,
            (LeU, I64) => 0x58,
            (GeS, I64) => 0x59,
            (GeU, I64) => 0x5A,

            // Binary Ops
            (Add, I32) => 0x6A,
            (Sub, I32) => 0x6B,
            (Mul, I32) => 0x6C,
            (DivS, I32) => 0x6D,
            (DivU, I32) => 0x6E,
            (RemS, I32) => 0x6F,
            (RemU, I32) => 0x70,
            (And, I32) => 0x71,
            (Or, I32) => 0x72,
            (Xor, I32) => 0x73,
            (Shl, I32) => 0x74,
            (ShrS, I32) => 0x75,
            (ShrU, I32) => 0x76,
            (Rotl, I32) => 0x77,
            (Rotr, I32) => 0x78,

            (Add, I64) => 0x7C,
            (Sub, I64) => 0x7D,
            (Mul, I64) => 0x7E,
            (DivS, I64) => 0x7F,
            (DivU, I64) => 0x80,
            (RemS, I64) => 0x81,
            (RemU, I64) => 0x82,
            (And, I64) => 0x83,
            (Or, I64) => 0x84,
            (Xor, I64) => 0x85,
            (Shl, I64) => 0x86,
            (ShrS, I64) => 0x87,
            (ShrU, I64) => 0x88,
            (Rotl, I64) => 0x89,
            (Rotr, I64) => 0x8A,
        };
        output.push(byte);
    }
}

impl EncodesToWasm for FloatOp {
    fn encode_to_wasm(&self, output: &mut Vec<u8>) {
        use FloatOpType::*;
        use FloatType::*;
        let byte = match (&self.op, &self.typ) {
            // Unary Ops
            (Abs, F32) => 0x8B,
            (Neg, F32) => 0x8C,
            (Sqrt, F32) => 0x91,
            (Ceil, F32) => 0x8D,
            (Floor, F32) => 0x8E,
            (Trunc, F32) => 0x8F,
            (Nearest, F32) => 0x90,

            (Abs, F64) => 0x99,
            (Neg, F64) => 0x9A,
            (Sqrt, F64) => 0x9F,
            (Ceil, F64) => 0x9B,
            (Floor, F64) => 0x9C,
            (Trunc, F64) => 0x9D,
            (Nearest, F64) => 0x9E,

            // Binary Ops
            (Add, F32) => 0x92,
            (Sub, F32) => 0x93,
            (Mul, F32) => 0x94,
            (Div, F32) => 0x95,
            (Min, F32) => 0x96,
            (Max, F32) => 0x97,
            (Copysign, F32) => 0x98,

            (Add, F64) => 0xA0,
            (Sub, F64) => 0xA1,
            (Mul, F64) => 0xA2,
            (Div, F64) => 0xA3,
            (Min, F64) => 0xA4,
            (Max, F64) => 0xA5,
            (Copysign, F64) => 0xA6,

            // Compare Ops
            (Eq, F32) => 0x5B,
            (Ne, F32) => 0x5C,
            (Lt, F32) => 0x5D,
            (Le, F32) => 0x5F,
            (Ge, F32) => 0x60,
            (Gt, F32) => 0x5E,

            (Eq, F64) => 0x61,
            (Ne, F64) => 0x62,
            (Lt, F64) => 0x63,
            (Le, F64) => 0x65,
            (Ge, F64) => 0x66,
            (Gt, F64) => 0x64,
        };
        output.push(byte);
    }
}

impl EncodesToWasm for ConvertOp {
    fn encode_to_wasm(&self, output: &mut Vec<u8>) {
        use ConvertOp::*;
        let (byte, extra) = match self {
            I32Extend8S => (0xC0, None),
            I32Extend16S => (0xC1, None),
            I64Extend8S => (0xC2, None),
            I64Extend16S => (0xC3, None),
            I64Extend32S => (0xC4, None),

            I32WrapI64 => (0xA7, None),
            I64ExtendI32S => (0xAC, None),
            I64ExtendI32U => (0xAD, None),

            I32TruncF32S => (0xA8, None),
            I32TruncF32U => (0xA9, None),
            I32TruncF64S => (0xAA, None),
            I32TruncF64U => (0xAB, None),
            I64TruncF32S => (0xAE, None),
            I64TruncF32U => (0xAF, None),
            I64TruncF64S => (0xB0, None),
            I64TruncF64U => (0xB1, None),

            I32TruncSatF32S => (0xFC, Some(0u32)),
            I32TruncSatF32U => (0xFC, Some(1u32)),
            I32TruncSatF64S => (0xFC, Some(2u32)),
            I32TruncSatF64U => (0xFC, Some(3u32)),
            I64TruncSatF32S => (0xFC, Some(4u32)),
            I64TruncSatF32U => (0xFC, Some(5u32)),
            I64TruncSatF64S => (0xFC, Some(6u32)),
            I64TruncSatF64U => (0xFC, Some(7u32)),
            F32DemoteF64 => (0xB6, None),
            F64PromoteF32 => (0xBB, None),

            F32ConvertI32S => (0xB2, None),
            F32ConvertI32U => (0xB3, None),
            F32ConvertI64S => (0xB4, None),
            F32ConvertI64U => (0xB5, None),
            F64ConvertI32S => (0xB7, None),
            F64ConvertI32U => (0xB8, None),
            F64ConvertI64S => (0xB9, None),
            F64ConvertI64U => (0xBA, None),

            I32ReinterpretF32 => (0xBC, None),
            I64ReinterpretF64 => (0xBD, None),
            F32ReinterpretI32 => (0xBE, None),
            F64ReinterpretI64 => (0xBF, None),
        };
        output.push(byte);
        if let Some(extra) = extra {
            extra.encode_to_leb128(output);
        }
    }
}
