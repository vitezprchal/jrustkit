use crate::attributes::attribute_parser::{AttributeParser, AttributeType};
use crate::constants::attribute_constants::NEST_MEMBERS;
use crate::reader::Reader;
use crate::structures::constant_pool::ConstantPool;

pub struct NestMembersParser;

pub struct NestMembersAttribute {}

impl AttributeParser for NestMembersParser {
    fn parse_attribute(&self, parser: &mut Reader, _constant_pool: &ConstantPool) -> AttributeType {
        todo!()
    }

    fn get_name(&self) -> &str {
        NEST_MEMBERS
    }
}