use std::fmt::Display;

#[derive(Debug)]
pub enum DebugKind {
    Info,
    Error,
}

pub trait Debugger: Default {
    fn debug<T: Display>(&self, kind: DebugKind, msg: T);
}

#[derive(Default)]
pub struct DebugPrint;

impl Debugger for DebugPrint {
    fn debug<T: Display>(&self, kind: DebugKind, msg: T) {
        match kind {
            DebugKind::Info => println!("{msg}"),
            DebugKind::Error => eprintln!("{msg}"),
        }
    }
}
