use crate::attributes::attribute_parser::{AttributeParser, AttributeType};
use crate::reader::Reader;
use crate::structures::constant_pool::ConstantPool;

pub struct ModuleMainClassParser;

pub struct ModuleMainClassAttribute {}

impl AttributeParser for ModuleMainClassParser {
    fn parse_attribute(&self, parser: &mut Reader, _constant_pool: &ConstantPool) -> AttributeType {
        todo!()
    }

    fn get_name(&self) -> &str {
        "ModuleMainClass"
    }
}