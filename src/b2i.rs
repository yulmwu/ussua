/* Bytecode bytes [u8] to Instruction */

use crate::{
    opcode::{Op, Opcode},
    BytecodeError, Instructions,
};

impl<'a> TryFrom<&'a [u8]> for Instructions<'a> {
    type Error = BytecodeError;

    fn try_from(bytes: &'a [u8]) -> Result<Self, Self::Error> {
        let mut instructions = Vec::new();

        let mut index = 0;

        while index < bytes.len() {
            use Opcode::*;

            let opcode = Opcode::try_from(bytes[index])?;

            match opcode {
                // with operand
                PUSH | STORE | LOAD | PROC | CALL | JMP | JIF | DBG => {
                    instructions.push(Op::new(opcode, Some(bytes[index + 1] as isize)));
                    index += 1;
                }
                _ => {
                    instructions.push(Op::new(opcode, None));
                }
            }

            index += 1;
        }

        Ok(Self(Box::leak(instructions.into_boxed_slice())))
    }
}
