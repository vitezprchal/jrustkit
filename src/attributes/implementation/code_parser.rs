use std::any::Any;
use crate::attributes::attribute_parser::{AttributeParser, AttributeType};
use crate::constants::attribute_constants::CODE;
use crate::constants::instructions::{*};
use crate::instructions::instruction::{*};
use crate::parse::Parser;
use crate::reader::Reader;
use crate::structures::constant_pool::ConstantPool;

pub struct CodeAttributeParser;

pub struct CodeAttribute {
    pub max_stack: u16,
    pub max_locals: u16,
    pub instructions: Vec<Box<dyn Instruction>>,
}

impl AttributeParser for CodeAttributeParser {
    fn parse_attribute(&self, reader: &mut Reader, constant_pool: &ConstantPool) -> AttributeType {
        let max_stack = reader.read_u2();
        let max_locals = reader.read_u2();
        let code_length = reader.read_u4();

        let mut i = 0;
        let mut instructions: Vec<Box<dyn Instruction>> = Vec::new();
        while i < code_length {
            let opcode = reader.read_u1();

            println!("i = {}", i);
            println!("code_length = {}", code_length);
            match opcode {
                ALOAD => {
                    let index = reader.read_u1();
                    instructions.push(Box::new(InstructionLoad {
                        opcode,
                        index,
                    }));
                    i += 1;
                }
                ALOAD_0 | ALOAD_1 | ALOAD_2 | ALOAD_3 => {
                    let index = opcode - ALOAD_0;
                    instructions.push(Box::new(InstructionLoad {
                        opcode: ALOAD,
                        index,
                    }));
                }
                ANEWARRAY => {
                    let index = reader.read_u2();
                    instructions.push(Box::new(InstructionAnewArray {
                        opcode,
                        index,
                    }));
                    i += 2;
                }
                ASTORE => {}
                ASTORE_0 | ASTORE_1 | ASTORE_2 | ASTORE_3 => {}
                BIPUSH => {}
                CHECKCAST => {}
                DLOAD => {}
                DLOAD_0 | DLOAD_1 | DLOAD_2 | DLOAD_3 => {}
                DSTORE => {}
                DSTORE_0 | DSTORE_1 | DSTORE_2 | DSTORE_3 => {}
                FLOAD => {}
                FLOAD_0 | FLOAD_1 | FLOAD_2 | FLOAD_3 => {}
                FSTORE => {}
                FSTORE_0 | FSTORE_1 | FSTORE_2 | FSTORE_3 => {}
                GETFIELD | GETSTATIC | PUTFIELD | PUTSTATIC => {}
                GOTO => {}
                GOTO_W => {}
                IF_ACMPEQ | IF_ACMPNE | IF_ICMPEQ | IF_ICMPGE | IF_ICMPGT
                | IF_ICMPLE | IF_ICMPLT | IF_ICMPNE | IFEQ | IFGE | IFGT
                | IFLE | IFLT | IFNE | IFNONNULL | IFNULL => {}
                ILOAD => {}
                ILOAD_0 | ILOAD_1 | ILOAD_2 | ILOAD_3 => {}
                INSTANCEOF => {}
                INVOKEDYNAMIC | INVOKEINTERFACE | INVOKESPECIAL
                | INVOKESTATIC | INVOKEVIRTUAL => {
                    i += 2;
                }
                ISTORE => {}
                ISTORE_0 | ISTORE_1 | ISTORE_2 | ISTORE_3 => {}
                JSR => {}
                JSR_W => {}
                LDC => {}
                LDC_W => {}
                LDC2_W => {}
                LLOAD => {}
                LLOAD_0 | LLOAD_1 | LLOAD_2 | LLOAD_3 => {}
                LOOKUPSWITCH => {}
                LSTORE => {}
                LSTORE_0 | LSTORE_1 | LSTORE_2 | LSTORE_3 => {}
                MULTIANEWARRAY => {}
                NEW => {}
                NEWARRAY => {}
                RET => {}
                SIPUSH => {}
                TABLESWITCH => {}
                WIDE => {}

                _ => {
                    instructions.push(Box::new(InstructionByte {
                        opcode,
                    }));
                    //println!("Unknown opcode")
                }
            }

            println!("opcode = {}", opcode);
            i += 1;
        }


        let exception_table_length = reader.read_u2();
        for _ in 0..exception_table_length {
            let start_pc = reader.read_u2();
            let end_pc = reader.read_u2();
            let handler_pc = reader.read_u2();
            let catch_type = reader.read_u2();
        }

        self.parse_attribute(reader, constant_pool);

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