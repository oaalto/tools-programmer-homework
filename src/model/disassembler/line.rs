use crate::model::disassembler::opcodes::{unknown_op_code, OpCode};
use std::fmt::{Display, Formatter};

/// Line keeps track of a single instruction line.
#[derive(Default)]
pub struct Line {
    memory_location: u64,
    bytes: Vec<u8>,
    op_code_byte: u8,
    op_code: OpCode,
}

impl Line {
    /// Create a new Line starting at the given memory location.
    pub fn new(memory_location: u64) -> Self {
        Self {
            memory_location,
            bytes: vec![],
            op_code_byte: 0,
            op_code: unknown_op_code(),
        }
    }

    pub fn get_bytes(&self) -> &Vec<u8> {
        &self.bytes
    }

    pub fn get_opcode(&self) -> &OpCode {
        &self.op_code
    }

    pub fn add_byte(&mut self, b: u8) {
        self.bytes.push(b);
    }

    pub fn set_op_code_byte(&mut self, op_code_byte: u8) {
        self.op_code_byte = op_code_byte;
    }

    pub fn set_op_code(&mut self, op_code: OpCode) {
        self.op_code = op_code;
    }

    fn format_bytes(&self) -> String {
        self.bytes
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect::<Vec<String>>()
            .join(" ")
    }
}

impl Display for Line {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:#06X} {:02x} {:9} {}",
            self.memory_location,
            self.op_code_byte,
            self.format_bytes(),
            self.op_code.format(&self.bytes)
        )
    }
}
