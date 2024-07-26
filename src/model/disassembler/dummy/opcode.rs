use crate::model::disassembler::opcodes::{unknown_op_code, OpCode};

pub fn get_opcode(code: u8) -> OpCode {
    match code {
        0x00 => OpCode::new(|_: &[u8]| -> String { "BRK".to_string() }, 0),
        0x06 => OpCode::new(
            |bytes: &[u8]| -> String { format!("ASL ${:02x}", bytes[0]) },
            1,
        ),
        0x16 => OpCode::new(
            |bytes: &[u8]| -> String { format!("ASL ${:02x},X", bytes[0]) },
            1,
        ),
        0x18 => OpCode::new(|_: &[u8]| -> String { "CLC".to_string() }, 0),
        0x1E => OpCode::new(
            |bytes: &[u8]| -> String { format!("ASL ${:02x}{:02x},X", bytes[1], bytes[0]) },
            2,
        ),
        0x20 => OpCode::new(
            |bytes: &[u8]| -> String { format!("JSR ${:02x}{:02x}", bytes[1], bytes[0]) },
            2,
        ),
        0x21 => OpCode::new(
            |bytes: &[u8]| -> String { format!("AND (${:02x},X)", bytes[0]) },
            1,
        ),
        0x41 => OpCode::new(
            |bytes: &[u8]| -> String { format!("EOR (${:02x},X)", bytes[0]) },
            1,
        ),
        0x45 => OpCode::new(
            |bytes: &[u8]| -> String { format!("EOR ${:02x}", bytes[0]) },
            1,
        ),
        0x48 => OpCode::new(|_: &[u8]| -> String { "PHA".to_string() }, 0),
        0x55 => OpCode::new(
            |bytes: &[u8]| -> String { format!("EOR ${:02x},X", bytes[0]) },
            1,
        ),
        0x61 => OpCode::new(
            |bytes: &[u8]| -> String { format!("ADC (${:02x},X)", bytes[0]) },
            1,
        ),
        0x6E => OpCode::new(
            |bytes: &[u8]| -> String { format!("ROR ${:02x}{:02x}", bytes[1], bytes[0]) },
            2,
        ),
        0xA0 => OpCode::new(
            |bytes: &[u8]| -> String { format!("LDY #${:02x}", bytes[0]) },
            1,
        ),
        0xA9 => OpCode::new(
            |bytes: &[u8]| -> String { format!("LDA #${:02x}", bytes[0]) },
            1,
        ),
        0xEA => OpCode::new(|_: &[u8]| -> String { "NOP".to_string() }, 0),
        0xEE => OpCode::new(
            |bytes: &[u8]| -> String { format!("INC ${:02x}{:02x}", bytes[1], bytes[0]) },
            2,
        ),
        0xF8 => OpCode::new(|_: &[u8]| -> String { "SED".to_string() }, 0),
        _ => unknown_op_code(),
    }
}
