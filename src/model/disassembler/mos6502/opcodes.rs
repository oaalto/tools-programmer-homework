use std::collections::HashMap;
use std::sync::OnceLock;

#[derive(Clone)]
pub struct OpCode {
    format_fn: fn(&[u8]) -> String,
    pub num_bytes: u8,
}

impl Default for OpCode {
    fn default() -> Self {
        unknown_op_code().clone()
    }
}

impl OpCode {
    pub fn format(&self, bytes: &[u8]) -> String {
        (self.format_fn)(bytes)
    }
}

pub fn opcodes() -> &'static HashMap<u8, OpCode> {
    static HASHMAP: OnceLock<HashMap<u8, OpCode>> = OnceLock::new();
    HASHMAP.get_or_init(|| {
        let mut m = HashMap::new();
        m.insert(
            0x00,
            OpCode {
                format_fn: |_: &[u8]| -> String { "BRK".to_string() },
                num_bytes: 0,
            },
        );
        m.insert(
            0x06,
            OpCode {
                format_fn: |bytes: &[u8]| -> String { format!("ASL ${:02x}", bytes[0]) },
                num_bytes: 1,
            },
        );
        m.insert(
            0x16,
            OpCode {
                format_fn: |bytes: &[u8]| -> String { format!("ASL ${:02x},X", bytes[0]) },
                num_bytes: 1,
            },
        );
        m.insert(
            0x18,
            OpCode {
                format_fn: |_: &[u8]| -> String { "CLC".to_string() },
                num_bytes: 0,
            },
        );
        m.insert(
            0x1E,
            OpCode {
                format_fn: |bytes: &[u8]| -> String {
                    format!("ASL ${:02x}{:02x},X", bytes[1], bytes[0])
                },
                num_bytes: 2,
            },
        );
        m.insert(
            0x20,
            OpCode {
                format_fn: |bytes: &[u8]| -> String {
                    format!("JSR ${:02x}{:02x}", bytes[1], bytes[0])
                },
                num_bytes: 2,
            },
        );
        m.insert(
            0x21,
            OpCode {
                format_fn: |bytes: &[u8]| -> String { format!("AND (${:02x},X)", bytes[0]) },
                num_bytes: 1,
            },
        );
        m.insert(
            0x41,
            OpCode {
                format_fn: |bytes: &[u8]| -> String { format!("EOR (${:02x},X)", bytes[0]) },
                num_bytes: 1,
            },
        );
        m.insert(
            0x45,
            OpCode {
                format_fn: |bytes: &[u8]| -> String { format!("EOR ${:02x}", bytes[0]) },
                num_bytes: 1,
            },
        );
        m.insert(
            0x48,
            OpCode {
                format_fn: |_: &[u8]| -> String { "PHA".to_string() },
                num_bytes: 0,
            },
        );
        m.insert(
            0x55,
            OpCode {
                format_fn: |bytes: &[u8]| -> String { format!("EOR ${:02x},X", bytes[0]) },
                num_bytes: 1,
            },
        );
        m.insert(
            0x61,
            OpCode {
                format_fn: |bytes: &[u8]| -> String { format!("ADC (${:02x},X)", bytes[0]) },
                num_bytes: 1,
            },
        );
        m.insert(
            0x6E,
            OpCode {
                format_fn: |bytes: &[u8]| -> String {
                    format!("ROR ${:02x}{:02x}", bytes[1], bytes[0])
                },
                num_bytes: 2,
            },
        );
        m.insert(
            0xA0,
            OpCode {
                format_fn: |bytes: &[u8]| -> String { format!("LDY #${:02x}", bytes[0]) },
                num_bytes: 1,
            },
        );
        m.insert(
            0xA9,
            OpCode {
                format_fn: |bytes: &[u8]| -> String { format!("LDA #${:02x}", bytes[0]) },
                num_bytes: 1,
            },
        );
        m.insert(
            0xEA,
            OpCode {
                format_fn: |_: &[u8]| -> String { "NOP".to_string() },
                num_bytes: 0,
            },
        );
        m.insert(
            0xEE,
            OpCode {
                format_fn: |bytes: &[u8]| -> String {
                    format!("INC ${:02x}{:02x}", bytes[1], bytes[0])
                },
                num_bytes: 2,
            },
        );
        m.insert(
            0xF8,
            OpCode {
                format_fn: |_: &[u8]| -> String { "SED".to_string() },
                num_bytes: 0,
            },
        );
        m
    })
}

pub fn unknown_op_code() -> &'static OpCode {
    static OP_CODE: OnceLock<OpCode> = OnceLock::new();
    OP_CODE.get_or_init(|| OpCode {
        format_fn: |_bytes: &[u8]| -> String { "???".to_string() },
        num_bytes: 0,
    })
}
