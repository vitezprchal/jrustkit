use crate::attributes::attribute_parser::{AttributeParser, AttributeType};
use crate::constants::attribute_constants::CONSTANT_VALUE;
use crate::parse::Parser;
use crate::reader::Reader;
use crate::structures::constant_pool::ConstantPool;

pub struct ConstantValueParser;

pub struct ConstantValueAttribute {
    pub constant_value_index: u16,
}

impl AttributeParser for ConstantValueParser {
    fn parse_attribute(&self, reader: &mut Reader, _: &ConstantPool) -> AttributeType {
        let constant_value_index = reader.read_u2();

        AttributeType::ConstantValue(ConstantValueAttribute {
            constant_value_index,
        })
    }

    fn get_name(&self) -> &str {
        CONSTANT_VALUE
    }
}