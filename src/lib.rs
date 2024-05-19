pub mod opcode;

pub type Value = isize;
pub type Pointer = usize;

use thiserror::Error;

#[derive(Debug, Clone, Error)]
#[rustfmt::skip]
pub enum BytecodeErrorKind {
    #[error("Invalid opcode: `{0}`.")] InvalidOpcode(u8),
}

#[derive(Debug, Clone)]
pub struct BytecodeError {
    pub kind: BytecodeErrorKind,
    pub ptr: Option<Pointer>,
}

impl BytecodeError {
    pub fn new(kind: BytecodeErrorKind) -> Self {
        Self { kind, ptr: None }
    }

    pub fn new_with_ptr(kind: BytecodeErrorKind, ptr: Option<Pointer>) -> Self {
        Self { kind, ptr }
    }
}
