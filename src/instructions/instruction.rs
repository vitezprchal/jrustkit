use crate::instructions::implementation::instruction_load::InstructionVariable;

pub enum InstructionType {
    Variable(InstructionVariable)
}

pub trait Instruction {
    fn get_opcode(&self) -> u8;
    fn get_type(&self) -> InstructionType;
}
