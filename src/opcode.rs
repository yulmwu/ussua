use crate::{BytecodeError, BytecodeErrorKind};

macro_rules! opcodes {
    ($($variant:ident = $value:expr),* $(,)?) => {
        #[derive(Debug, Clone, Copy, PartialEq)]
        #[repr(u8)]
        #[non_exhaustive]
        pub enum Opcode {
            $($variant = $value,)*
        }

        impl TryFrom<u8> for Opcode {
            type Error = BytecodeError;

            fn try_from(value: u8) -> Result<Self, Self::Error> {
                match value {
                    $($value => Ok(Opcode::$variant),)*
                    _ => return Err(BytecodeError::new(BytecodeErrorKind::InvalidOpcode(value))),
                }
            }
        }
    }
}

opcodes! {
    // 0x0-
    NOOP = 0x00,
    PUSH = 0x01,
    STORE = 0x02,
    LOAD = 0x03,

    // 0x1-
    ADD = 0x10,
    SUB = 0x11,
    MUL = 0x12,
    DIV = 0x13,
    MOD = 0x14,

    AND = 0x18,
    OR = 0x19,
    XOR = 0x1A,
    NOT = 0x1B,
    LSF = 0x1C,
    RSF = 0x1D,

    // 0x2-
    EQ = 0x20,
    GT = 0x21,
    LT = 0x22,
    GTE = 0x23,
    LTE = 0x24,

    PROC = 0x28,
    CALL = 0x29,
    RET = 0x2A,

    JMP = 0x2C,
    JIF = 0x2D,

    // 0xF-
    DBG = 0xFE,
    EXIT = 0xFF,
}
