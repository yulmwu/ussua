pub mod opcode;
pub mod vm;

pub type Value = isize;
pub type Pointer = usize;

use opcode::Op;
use thiserror::Error;

#[derive(Debug, Clone, Copy, Default)]
pub struct Instructions<'a>(pub &'a [Op]);

#[derive(Debug, Clone, Error)]
#[rustfmt::skip]
pub enum BytecodeErrorKind {
    #[error("Invalid opcode: `{0}`.")] InvalidOpcode(u8),
    #[error("Empty Stack")] EmptyStack,
    #[error("Empty Operand")] EmptyOperand,
    #[error("Empty Call Stack")] EmptyCallStack,
    #[error("Address not found in heap store: `{0}`.")] NotFoundAddressInStore(Pointer)
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

    pub fn new_with_ptr(kind: BytecodeErrorKind, ptr: Pointer) -> Self {
        Self {
            kind,
            ptr: Some(ptr),
        }
    }
}
