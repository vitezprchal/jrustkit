use crate::attributes::attribute_parser::{AttributeParser, AttributeType};
use crate::constants::attribute_constants::MODULE;
use crate::reader::Reader;
use crate::structures::constant_pool::ConstantPool;

pub struct ModuleParser;

pub struct ModuleAttribute {}

impl AttributeParser for ModuleParser {
    fn parse_attribute(&self, parser: &mut Reader, constant_pool: &ConstantPool) -> AttributeType {
        todo!()
    }

    fn get_name(&self) -> &str {
        MODULE
    }
}
