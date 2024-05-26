use std::collections::HashMap;

use crate::{
    opcode::{Op, Opcode},
    BytecodeError, BytecodeErrorKind, Instructions, Pointer, Value,
};

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
pub struct Vm<'a> {
    pub instructions: Instructions<'a>,
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

enum OpExecuted {
    Ok,
    Continue,
    Break,
}

impl<'a> Vm<'a> {
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
            Opcode::ADD => todo!(),
            Opcode::SUB => todo!(),
            Opcode::MUL => todo!(),
            Opcode::DIV => todo!(),
            Opcode::MOD => todo!(),
            Opcode::AND => todo!(),
            Opcode::OR => todo!(),
            Opcode::XOR => todo!(),
            Opcode::NOT => todo!(),
            Opcode::LSF => todo!(),
            Opcode::RSF => todo!(),
            Opcode::EQ => todo!(),
            Opcode::GT => todo!(),
            Opcode::LT => todo!(),
            Opcode::GTE => todo!(),
            Opcode::LTE => todo!(),
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
            Opcode::JMP => todo!(),
            Opcode::JIF => todo!(),
            Opcode::DBG => todo!(),
            Opcode::EXIT => return Ok(OpExecuted::Break),
        }

        Ok(OpExecuted::Ok)
    }
}
