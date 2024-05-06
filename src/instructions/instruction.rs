use crate::instructions::implementation::instruction_load::*;

pub enum InstructionType {
    None {},
    Variable { index: u8 },
    Anewarray { index: u16 },
    Bipush { byte: u8 },
    Checkcast { index: u16 },
    Field { index: u16 },
    Ldc { index: u8 },
    Invoke { index: u16 },
    Offset { offset: u16 },
}

pub struct Instruction {
    pub instruction_type: InstructionType,
    pub opcode: u8,
}

impl Instruction {
    pub fn new(opcode: u8, instruction_type: InstructionType) -> Instruction {
        Instruction {
            instruction_type,
            opcode,
        }
    }
}
