use std::fmt::Display;

use crate::Pointer;

#[derive(Debug)]
pub enum DebugMessage<T: Display> {
    InstructionTrace(T, Pointer),
    DbgInstruction(T),
}

pub trait Debugger: Default {
    fn debug<T: Display>(&self, msg: DebugMessage<T>);
}

#[derive(Default)]
pub struct DebugPrint;

impl Debugger for DebugPrint {
    fn debug<T: Display>(&self, msg: DebugMessage<T>) {
        match msg {
            DebugMessage::InstructionTrace(msg, ptr) => println!("Executing {msg}, Pointer: {ptr}"),
            DebugMessage::DbgInstruction(msg) => println!("DBG instruction: {msg}"),
        }
    }
}

#[derive(Default)]
pub struct DebugDisable;

impl Debugger for DebugDisable {
    fn debug<T: Display>(&self, _: DebugMessage<T>) {}
}
