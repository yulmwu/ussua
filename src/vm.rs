use std::collections::HashMap;

use crate::{BytecodeError, BytecodeErrorKind, Instructions, Pointer, Value};

#[derive(Debug, Clone, Default)]
pub struct Stack(pub Vec<Value>);

impl Stack {
    pub fn push(&mut self, value: Value) {
        self.0.push(value);
    }

    pub fn pop(&mut self) -> Result<Value, BytecodeError> {
        self.0.pop().ok_or(BytecodeError::new(BytecodeErrorKind::EmptyStack))
    }
}

#[derive(Debug, Default)]
pub struct Vm<'a> {
    pub instructions: Vec<Instructions<'a>>,
    stack: Stack,
    call_stack: CallStack,
    heap_store: Store,
}

#[derive(Debug, Clone, Copy)]
struct CallStackFrame {
    pointer: Pointer,
}

type CallStack = Vec<CallStackFrame>;

type Store = HashMap<usize, Value>;
