/// Encapsulates how a single instruction can be read and printed.
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
    /// Creates a new OpCode with the given formatter and the number of bytes this instruction needs to read.
    pub fn new(format_fn: fn(&[u8]) -> String, num_bytes: u8) -> Self {
        Self {
            format_fn,
            num_bytes,
        }
    }

    /// Format the given bytes according to this OpCode's formatter.
    pub fn format(&self, bytes: &[u8]) -> String {
        (self.format_fn)(bytes)
    }

    /// Get the number of bytes this OpCode needs.
    pub fn num_bytes(&self) -> u8 {
        self.num_bytes
    }
}

/// Create an unknown OpCode.
pub fn unknown_op_code() -> OpCode {
    OpCode {
        format_fn: |_bytes: &[u8]| -> String { "???".to_string() },
        num_bytes: 0,
    }
}
