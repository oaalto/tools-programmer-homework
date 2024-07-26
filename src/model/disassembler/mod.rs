use serde::{Deserialize, Serialize};

mod dummy;
mod mos6502;

#[derive(Clone)]
pub struct DisassemblyResult(pub(crate) Vec<String>);

#[derive(Default, Debug, Deserialize, Serialize)]
pub enum Architecture {
    #[serde(rename = "mos6502")]
    #[default]
    Mos6502,

    #[serde(rename = "dummy")]
    Dummy,
}

pub fn disassemble(arch: Architecture, data: &[u8]) -> DisassemblyResult {
    match arch {
        Architecture::Mos6502 => mos6502::disassemble(data),
        Architecture::Dummy => dummy::disassemble(data),
    }
}
