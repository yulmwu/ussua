use ussua::{
    debugger::DebugPrint,
    opcode::{Op, Opcode},
    vm::Vm,
    Instructions,
};

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

    println!(
        "{:?}",
        instructions
            .iter()
            .map(|v| format!(
                "0x{:02x}{}",
                v.opcode as u8,
                match v.operand {
                    Some(operand) => format!(" 0x{:02x}", operand as u8),
                    None => String::new(),
                }
            ))
            .collect::<Vec<_>>()
    );

    let instructions = Instructions(&instructions);

    let arr = [0x28, 0x02, 0x00, 0x2a, 0x29, 0x00, 0x00];
    let arr: &[u8] = &arr;
    let instructions_from_arr: Instructions = Instructions::try_from(arr).unwrap();

    assert_eq!(instructions, instructions_from_arr);

    let mut vm = Vm::<DebugPrint>::new(instructions);
    vm.execute().unwrap();
}
