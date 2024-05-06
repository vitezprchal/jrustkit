use crate::attributes::attribute_parser::{AttributeParser, AttributeType};
use crate::constants::attribute_constants::ENCLOSING_METHOD;
use crate::reader::Reader;
use crate::structures::constant_pool::ConstantPool;

pub struct EnclosingMethodParser;

pub struct EnclosingMethodAttribute {
    pub class_index: u16,
    pub method_index: u16,
}

impl AttributeParser for EnclosingMethodParser {
    fn parse_attribute(&self, parser: &mut Reader, constant_pool: &ConstantPool) -> AttributeType {
        let class_index = parser.read_u2();
        let method_index = parser.read_u2();
        AttributeType::EnclosingMethod(EnclosingMethodAttribute {
            class_index,
            method_index,
        })
    }

    fn get_name(&self) -> &str {
        ENCLOSING_METHOD
    }
}
