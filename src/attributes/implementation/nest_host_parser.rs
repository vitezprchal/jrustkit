use crate::attributes::attribute_parser::{AttributeParser, AttributeType};
use crate::constants::attribute_constants::NEST_HOST;
use crate::reader::Reader;
use crate::structures::constant_pool::ConstantPool;

pub struct NestHostParser;

pub struct NestHostAttribute {}

impl AttributeParser for NestHostParser {
    fn parse_attribute(&self, parser: &mut Reader, _constant_pool: &ConstantPool) -> AttributeType {
        todo!()
    }

    fn get_name(&self) -> &str {
        NEST_HOST
    }
}
