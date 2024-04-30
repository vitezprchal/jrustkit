use crate::attributes::attribute_parser::{AttributeParser, AttributeType};
use crate::constants::attribute_constants::DEPRECATED;
use crate::reader::Reader;
use crate::structures::constant_pool::ConstantPool;

pub struct DeprecatedParser;

pub struct DeprecatedAttribute;

impl AttributeParser for DeprecatedParser {
    fn parse_attribute(&self, reader: &mut Reader, _: &ConstantPool) -> AttributeType {
        AttributeType::Deprecated(DeprecatedAttribute {})
    }

    fn get_name(&self) -> &str {
        DEPRECATED
    }
}