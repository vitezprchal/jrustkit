use crate::attributes::attribute_parser::{AttributeParser, AttributeType};
use crate::constants::attribute_constants::CODE;
use crate::constants::instructions::*;
use crate::instructions::instruction::*;
use crate::reader::Reader;
use crate::structures::constant_pool::ConstantPool;

pub struct CodeAttributeParser;

pub struct CodeAttribute {
    pub max_stack: u16,
    pub max_locals: u16,
    pub instructions: Vec<Instruction>,
}

impl AttributeParser for CodeAttributeParser {
    fn parse_attribute(&self, reader: &mut Reader, constant_pool: &ConstantPool) -> AttributeType {
        let max_stack = reader.read_u2();
        let max_locals = reader.read_u2();
        let code_length = reader.read_u4();

        let mut i = 0;
        let mut instructions = Vec::new();
        println!("code_length = {}", code_length);
        while i < code_length {
            let opcode = reader.read_u1();

            println!("{} {}", opcode, i);
            match opcode {
                ALOAD | DLOAD | FLOAD | ILOAD | LLOAD | ASTORE | DSTORE | FSTORE | ISTORE
                | LSTORE => {
                    let index = reader.read_u1();
                    instructions.push(Instruction::new(
                        opcode,
                        InstructionType::Variable { index },
                    ));

                    i += 1;
                }
                ALOAD_0 | ALOAD_1 | ALOAD_2 | ALOAD_3 => {
                    let index = opcode - ALOAD_0;
                    instructions.push(Instruction::new(ALOAD, InstructionType::Variable { index }));
                }

                DLOAD_0 | DLOAD_1 | DLOAD_2 | DLOAD_3 => {
                    let index = opcode - DLOAD_0;
                    instructions.push(Instruction::new(DLOAD, InstructionType::Variable { index }));
                }
                FLOAD_0 | FLOAD_1 | FLOAD_2 | FLOAD_3 => {
                    let index = opcode - FLOAD_0;
                    instructions.push(Instruction::new(FLOAD, InstructionType::Variable { index }));
                }
                ILOAD_0 | ILOAD_1 | ILOAD_2 | ILOAD_3 => {
                    let index = opcode - ILOAD_0;
                    instructions.push(Instruction::new(ILOAD, InstructionType::Variable { index }));
                }
                LLOAD_0 | LLOAD_1 | LLOAD_2 | LLOAD_3 => {
                    let index = opcode - LLOAD_0;
                    instructions.push(Instruction::new(LLOAD, InstructionType::Variable { index }));
                }

                DSTORE_0 | DSTORE_1 | DSTORE_2 | DSTORE_3 => {
                    let index = opcode - DSTORE_0;
                    instructions.push(Instruction::new(
                        DSTORE,
                        InstructionType::Variable { index },
                    ));
                }
                FSTORE_0 | FSTORE_1 | FSTORE_2 | FSTORE_3 => {
                    let index = opcode - FSTORE_0;
                    instructions.push(Instruction::new(
                        FSTORE,
                        InstructionType::Variable { index },
                    ));
                }
                ISTORE_0 | ISTORE_1 | ISTORE_2 | ISTORE_3 => {
                    let index = opcode - ISTORE_0;
                    instructions.push(Instruction::new(
                        ISTORE,
                        InstructionType::Variable { index },
                    ));
                }
                LSTORE_0 | LSTORE_1 | LSTORE_2 | LSTORE_3 => {
                    let index = opcode - LSTORE_0;
                    instructions.push(Instruction::new(
                        LSTORE,
                        InstructionType::Variable { index },
                    ));
                }
                ASTORE_0 | ASTORE_1 | ASTORE_2 | ASTORE_3 => {
                    let index = opcode - ASTORE_0;
                    instructions.push(Instruction::new(
                        ASTORE,
                        InstructionType::Variable { index },
                    ));
                }
                BIPUSH => {
                    let byte = reader.read_u1();
                    instructions.push(Instruction::new(BIPUSH, InstructionType::Bipush { byte }));
                    i += 1;
                }
                CHECKCAST => {
                    let index = reader.read_u2();
                    instructions.push(Instruction::new(
                        CHECKCAST,
                        InstructionType::Checkcast { index },
                    ));
                    i += 2;
                }
                GETFIELD | GETSTATIC | PUTFIELD | PUTSTATIC => {
                    let index = reader.read_u2();
                    instructions.push(Instruction::new(opcode, InstructionType::Field { index }));

                    i += 2;
                }
                GOTO => {}
                GOTO_W => {}
                IF_ACMPEQ | IF_ACMPNE | IF_ICMPEQ | IF_ICMPGE | IF_ICMPGT | IF_ICMPLE
                | IF_ICMPLT | IF_ICMPNE | IFEQ | IFGE | IFGT | IFLE | IFLT | IFNE | IFNONNULL
                | IFNULL => {}
                INSTANCEOF => {}
                INVOKEDYNAMIC | INVOKEINTERFACE | INVOKESPECIAL | INVOKESTATIC | INVOKEVIRTUAL => {
                    let index = reader.read_u2();
                    instructions.push(Instruction::new(opcode, InstructionType::Invoke { index }));

                    i += 2;
                }
                JSR => {}
                JSR_W => {}
                LDC => {
                    let index = reader.read_u1();
                    instructions.push(Instruction::new(LDC, InstructionType::Ldc { index }));

                    i += 1;
                }
                LDC_W => {}
                LDC2_W => {}
                LOOKUPSWITCH => {}
                MULTIANEWARRAY => {}
                NEW => {}
                NEWARRAY => {}
                RET => {}
                SIPUSH => {}
                TABLESWITCH => {}
                ANEWARRAY => {
                    let index = reader.read_u2();
                    instructions.push(Instruction::new(
                        ALOAD,
                        InstructionType::Anewarray { index },
                    ));

                    i += 2;
                }
                WIDE => {}

                _ => {
                    instructions.push(Instruction::new(opcode, InstructionType::None {}));
                }
            }

            i += 1;
        }

        let exception_table_length = reader.read_u2();
        for _ in 0..exception_table_length {
            let start_pc = reader.read_u2();
            let end_pc = reader.read_u2();
            let handler_pc = reader.read_u2();
            let catch_type = reader.read_u2();
        }

        AttributeType::Code(CodeAttribute {
            max_stack,
            max_locals,
            instructions,
        })
    }

    fn get_name(&self) -> &str {
        CODE
    }
}
