pub struct Instruction {
    opcode: u8,
}

// Implementation

impl Instruction {
    pub fn new(opcode: u8) -> Instruction {
        Instruction { opcode }
    }

    pub fn opcode(&self) -> u8 {
        self.opcode
    }
}