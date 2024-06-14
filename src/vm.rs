use crate::{
    debugger::{DebugKind, Debugger},
    opcode::{Op, Opcode},
    BytecodeError, BytecodeErrorKind, Instructions, Pointer, Value,
};
use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct Stack(pub Vec<Value>);

impl Stack {
    pub fn push(&mut self, value: Value) {
        self.0.push(value);
    }

    pub fn pop(&mut self) -> Result<Value, BytecodeError> {
        self.0
            .pop()
            .ok_or(BytecodeError::new(BytecodeErrorKind::EmptyStack))
    }
}

#[derive(Debug, Default)]
pub struct Vm<'a, T: Debugger> {
    pub instructions: Instructions<'a>,
    stack: Stack,
    call_stack: CallStack,
    heap_store: Store,
    debugger: T,
}

#[derive(Debug, Clone, Copy)]
struct CallStackFrame {
    pointer: Pointer,
}

type CallStack = Vec<CallStackFrame>;

type Store = HashMap<usize, Value>;

enum OpExecuted {
    Ok,
    Continue,
    Break,
}

impl<'a, T: Debugger> Vm<'a, T> {
    pub fn new(instructions: Instructions<'a>) -> Self {
        Vm {
            instructions,
            ..Default::default()
        }
    }

    pub fn execute(&mut self) -> Result<(), BytecodeError> {
        let mut pointer = 0;

        while let Some(op) = self.instructions.0.get(pointer) {
            match self.execute_op(op, &mut pointer)? {
                OpExecuted::Ok => {}
                OpExecuted::Continue => {
                    continue;
                }
                OpExecuted::Break => {
                    break;
                }
            }

            pointer += 1;
        }

        Ok(())
    }

    fn get_operand(&self, op: &Op, pointer: Pointer) -> Result<Value, BytecodeError> {
        op.operand.ok_or(BytecodeError::new_with_ptr(
            BytecodeErrorKind::EmptyOperand,
            pointer,
        ))
    }

    fn execute_op(&mut self, op: &Op, pointer: &mut Pointer) -> Result<OpExecuted, BytecodeError> {
        macro_rules! operator {
            (unary $op:tt) => {{
                let first = self.stack.pop()?;

                self.stack.push($op first);
            }};
            (binary $op:tt) => {{
                let first = self.stack.pop()?;
                let second = self.stack.pop()?;

                self.stack.push(second $op first);
            }};
            (eq $op:tt) => {{
                let first = self.stack.pop()?;
                let second = self.stack.pop()?;

                self.stack.push(if second $op first { 1 } else { 0 });
            }};
        }

        match op.opcode {
            Opcode::NOOP => {}
            Opcode::PUSH => {
                self.stack.push(self.get_operand(op, *pointer)?);
            }
            Opcode::STORE => {
                let value = self.stack.pop()?;
                let addr = self.get_operand(op, *pointer)? as usize;

                self.heap_store.insert(addr, value);
            }
            Opcode::LOAD => {
                let addr = self.get_operand(op, *pointer)? as usize;
                let value = self
                    .heap_store
                    .get(&addr)
                    .ok_or(BytecodeError::new_with_ptr(
                        BytecodeErrorKind::NotFoundAddressInStore(addr),
                        *pointer,
                    ))?;

                self.stack.push(*value);
            }
            Opcode::ADD => operator!(binary+),
            Opcode::SUB => operator!(binary -),
            Opcode::MUL => operator!(binary *),
            Opcode::DIV => operator!(binary /),
            Opcode::MOD => operator!(binary %),
            Opcode::AND => operator!(binary &),
            Opcode::OR => operator!(binary |),
            Opcode::XOR => operator!(binary ^),
            Opcode::NOT => operator!(unary !),
            Opcode::LSF => operator!(binary <<),
            Opcode::RSF => operator!(binary >>),
            Opcode::EQ => operator!(eq ==),
            Opcode::GT => operator!(eq >),
            Opcode::LT => operator!(eq <),
            Opcode::GTE => operator!(eq >=),
            Opcode::LTE => operator!(eq <=),
            Opcode::PROC => {
                *pointer = *pointer + (self.get_operand(op, *pointer)? as Pointer) + 1 /* proc */;
                return Ok(OpExecuted::Continue);
            }
            Opcode::CALL => {
                self.call_stack.push(CallStackFrame {
                    pointer: *pointer + 1,
                });

                *pointer = self.get_operand(op, *pointer)? as Pointer + 1 /* call [proc ptr] */;
                return Ok(OpExecuted::Continue);
            }
            Opcode::RET => {
                let frame = self.call_stack.pop().ok_or(BytecodeError::new_with_ptr(
                    BytecodeErrorKind::EmptyCallStack,
                    *pointer,
                ))?;
                *pointer = frame.pointer;

                return Ok(OpExecuted::Continue);
            }
            Opcode::JMP => {
                *pointer = self.get_operand(op, *pointer)? as Pointer;
                return Ok(OpExecuted::Continue);
            }
            Opcode::JIF => {
                if self.stack.pop()? == 0 {
                    *pointer = self.stack.pop()? as Pointer;
                    return Ok(OpExecuted::Continue);
                }
            }
            Opcode::DBG => self
                .debugger
                .debug(DebugKind::Info, self.stack.pop()?.to_string().as_str()),
            Opcode::EXIT => return Ok(OpExecuted::Break),
        }

        Ok(OpExecuted::Ok)
    }
}
