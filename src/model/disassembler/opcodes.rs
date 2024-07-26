use crate::model::disassembler::{dummy, mos6502, Architecture};

#[derive(Clone)]
pub struct OpCode {
    format_fn: fn(&[u8]) -> String,
    num_bytes: u8,
}

impl Default for OpCode {
    fn default() -> Self {
        unknown_op_code()
    }
}

impl OpCode {
    pub fn new(format_fn: fn(&[u8]) -> String, num_bytes: u8) -> Self {
        Self {
            format_fn,
            num_bytes,
        }
    }

    pub fn format(&self, bytes: &[u8]) -> String {
        (self.format_fn)(bytes)
    }

    pub fn num_bytes(&self) -> u8 {
        self.num_bytes
    }
}

pub fn get_opcode(architecture: &Architecture, code: u8) -> OpCode {
    match architecture {
        Architecture::Mos6502 => mos6502::get_opcode(code),
        Architecture::Dummy => dummy::get_opcode(code),
    }
}

pub fn unknown_op_code() -> OpCode {
    OpCode {
        format_fn: |_bytes: &[u8]| -> String { "???".to_string() },
        num_bytes: 0,
    }
}
