use ussua::{debugger::DebugPrint, opcode::{Op, Opcode}, vm::Vm, Instructions};

fn main() {
    let a = Opcode::try_from(0xFF).unwrap();
    println!("{:?}", a);

    let instructions = [
        Op::new(Opcode::PROC, Some(2)),
        Op::new(Opcode::NOOP, None),
        Op::new(Opcode::RET, None),
        Op::new(Opcode::CALL, Some(0)),
        Op::new(Opcode::NOOP, None),
    ];
    let instructions = Instructions(&instructions);

    let mut vm = Vm::<DebugPrint>::new(instructions);
    vm.execute().unwrap();
}
