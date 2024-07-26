use serde::{Deserialize, Serialize};

mod mos6502;

#[derive(Clone)]
pub struct DisassemblyResult(pub(crate) Vec<String>);

pub trait Disassembler {
    fn disassemble(&self, data: &[u8]) -> DisassemblyResult;
}

#[derive(Default, Debug, Deserialize, Serialize)]
pub enum Architecture {
    #[serde(rename = "mos6502")]
    #[default]
    Mos6502,
}

pub fn get_disassembler_for_architecture(arch: Architecture) -> impl Disassembler {
    match arch {
        Architecture::Mos6502 => mos6502::Mos6502Disassembler,
    }
}
