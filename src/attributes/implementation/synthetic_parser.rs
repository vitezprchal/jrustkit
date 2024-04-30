use crate::attributes::attribute_parser::{AttributeParser, AttributeType};
use crate::reader::Reader;
use crate::structures::constant_pool::ConstantPool;

pub struct SyntheticParser;

pub struct SyntheticAttribute {}

impl AttributeParser for SyntheticParser {
    fn parse_attribute(&self, reader: &mut Reader, _constant_pool: &ConstantPool) -> AttributeType {
        AttributeType::Synthetic(SyntheticAttribute {})
    }

    fn get_name(&self) -> &str {
        "Synthetic"
    }
}