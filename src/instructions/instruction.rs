use std::any::Any;

pub trait Instruction: Any {
    fn get_opcode(&self) -> u8;
}

pub struct InstructionLoad {
    pub opcode: u8,
    pub index: u8,
}

impl Instruction for InstructionLoad {
    fn get_opcode(&self) -> u8 {
        self.opcode
    }
}


pub struct InstructionByte {
    pub opcode: u8,
}

impl Instruction for InstructionByte {
    fn get_opcode(&self) -> u8 {
        self.opcode
    }
}

pub struct InstructionInvoke {
    pub opcode: u8,
    pub method_description: u16,
}

impl Instruction for InstructionInvoke {
    fn get_opcode(&self) -> u8 {
        self.opcode
    }
}