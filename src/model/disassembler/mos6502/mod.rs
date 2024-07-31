mod opcodes;

use crate::model::disassembler::line::Line;
use crate::model::disassembler::DisassemblyResult;
use std::mem;

/// Disassemble the given data as a MOS 6502 instruction set.
pub fn disassemble(data: &[u8]) -> DisassemblyResult {
    let disassembler = data
        .iter()
        .fold(Mos6502DisassemblerData::new(), |mut disassembler, b| {
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

    let lines: Vec<String> = disassembler.lines.iter().map(ToString::to_string).collect();

    DisassemblyResult(lines)
}

enum State {
    ReadOpCode,
    ReadByte,
}

struct Mos6502DisassemblerData {
    current_memory_location: u64,
    lines: Vec<Line>,
    current_line: Line,
    state: State,
}

impl Mos6502DisassemblerData {
    fn new() -> Self {
        Self {
            current_memory_location: 0,
            lines: vec![],
            current_line: Line::new(0),
            state: State::ReadOpCode,
        }
    }

    fn current_line_finished(&self) -> bool {
        self.current_line.get_bytes().len() == self.current_line.get_opcode().num_bytes() as usize
    }

    fn set_opcode_for_current_line(&mut self, opcode: u8) {
        self.current_line.set_op_code(opcodes::get_opcode(opcode));
        self.current_line.set_op_code_byte(opcode);
    }

    fn save_current_line(&mut self) {
        self.lines.push(mem::replace(
            &mut self.current_line,
            Line::new(self.current_memory_location),
        ));
    }
}
