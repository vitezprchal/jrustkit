use crate::attributes::attribute_parser::{AttributeParser, AttributeType};
use crate::constants::attribute_constants::BOOTSTRAP_METHODS;
use crate::reader::Reader;
use crate::structures::constant_pool::ConstantPool;

pub struct BootstrapMethodsParser;

pub struct BootstrapMethodsAttribute {}

impl AttributeParser for BootstrapMethodsParser {
    fn parse_attribute(&self, parser: &mut Reader, constant_pool: &ConstantPool) -> AttributeType {
        todo!()
    }

    fn get_name(&self) -> &str {
        BOOTSTRAP_METHODS
    }
}
