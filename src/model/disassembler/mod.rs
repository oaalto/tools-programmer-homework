use crate::model::disassembler::line::Line;
use crate::model::disassembler::opcodes::{opcodes, unknown_op_code};
use std::mem;

mod line;
mod opcodes;

enum State {
    ReadOpCode,
    ReadByte,
}

pub struct Disassembler {
    current_memory_location: u64,
    lines: Vec<Line>,
    current_line: Line,
    state: State,
}

impl Disassembler {
    fn new() -> Self {
        Self {
            current_memory_location: 0,
            lines: vec![],
            current_line: Line::new(0),
            state: State::ReadOpCode,
        }
    }

    fn current_line_finished(&self) -> bool {
        self.current_line.bytes.len() == self.current_line.op_code.num_bytes as usize
    }

    fn set_opcode_for_current_line(&mut self, opcode: u8) {
        self.current_line
            .set_op_code(opcodes().get(&opcode).unwrap_or(unknown_op_code()).clone());
        self.current_line.set_op_code_byte(opcode);
    }

    fn save_current_line(&mut self) {
        self.lines.push(mem::take(&mut self.current_line));
        self.current_line.memory_location = self.current_memory_location;
    }

    pub fn disassemble(data: &[u8]) -> Vec<String> {
        let disassembler = data
            .iter()
            .fold(Disassembler::new(), |mut disassembler, b| {
                match disassembler.state {
                    State::ReadOpCode => {
                        disassembler.set_opcode_for_current_line(*b);

                        disassembler.state = State::ReadByte;
                    }
                    State::ReadByte => disassembler.current_line.add_byte(*b),
                }

                disassembler.current_memory_location += 1;

                if disassembler.current_line_finished() {
                    disassembler.save_current_line();
                    disassembler.state = State::ReadOpCode;
                }

                disassembler
            });

        disassembler.lines.iter().map(ToString::to_string).collect()
    }
}
