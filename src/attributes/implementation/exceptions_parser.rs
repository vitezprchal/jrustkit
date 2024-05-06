use crate::attributes::attribute_parser::{AttributeParser, AttributeType};
use crate::constants::attribute_constants::{CODE, EXCEPTIONS};
use crate::constants::instructions::{
    ALOAD, ALOAD_0, ALOAD_1, ALOAD_2, ALOAD_3, ANEWARRAY, ASTORE, ASTORE_0, ASTORE_1, ASTORE_2,
    ASTORE_3, BIPUSH, CHECKCAST, DLOAD, DLOAD_0, DLOAD_1, DLOAD_2, DLOAD_3, DSTORE, DSTORE_0,
    DSTORE_1, DSTORE_2, DSTORE_3, FLOAD, FLOAD_0, FLOAD_1, FLOAD_2, FLOAD_3, FSTORE, FSTORE_0,
    FSTORE_1, FSTORE_2, FSTORE_3, GETFIELD, GETSTATIC, GOTO, GOTO_W, IFEQ, IFGE, IFGT, IFLE, IFLT,
    IFNE, IFNONNULL, IFNULL, IF_ACMPEQ, IF_ACMPNE, IF_ICMPEQ, IF_ICMPGE, IF_ICMPGT, IF_ICMPLE,
    IF_ICMPLT, IF_ICMPNE, ILOAD, ILOAD_0, ILOAD_1, ILOAD_2, ILOAD_3, INSTANCEOF, INVOKEDYNAMIC,
    INVOKEINTERFACE, INVOKESPECIAL, INVOKESTATIC, INVOKEVIRTUAL, ISTORE, ISTORE_0, ISTORE_1,
    ISTORE_2, ISTORE_3, JSR, JSR_W, LDC, LDC2_W, LDC_W, LLOAD, LLOAD_0, LLOAD_1, LLOAD_2, LLOAD_3,
    LOOKUPSWITCH, LSTORE, LSTORE_0, LSTORE_1, LSTORE_2, LSTORE_3, MULTIANEWARRAY, NEW, NEWARRAY,
    PUTFIELD, PUTSTATIC, RET, SIPUSH, TABLESWITCH, WIDE,
};
use crate::instructions::instruction::*;
use crate::parse::Parser;
use crate::reader::Reader;
use crate::structures::constant_pool::ConstantPool;
use std::any::Any;

pub struct ExceptionsParser;

pub struct ExceptionsAttribute {
    pub exceptions: Vec<u16>,
}

impl AttributeParser for ExceptionsParser {
    fn parse_attribute(&self, reader: &mut Reader, _: &ConstantPool) -> AttributeType {
        let number_of_exceptions = reader.read_u2();
        let mut exceptions = Vec::with_capacity(number_of_exceptions as usize);
        for _ in 0..number_of_exceptions {
            exceptions.push(reader.read_u2());
        }

        AttributeType::Exceptions(ExceptionsAttribute { exceptions })
    }

    fn get_name(&self) -> &str {
        EXCEPTIONS
    }
}
