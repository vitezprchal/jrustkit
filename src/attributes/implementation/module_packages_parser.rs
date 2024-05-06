use crate::attributes::attribute_parser::{AttributeParser, AttributeType};
use crate::constants::attribute_constants::MODULE_PACKAGES;
use crate::reader::Reader;
use crate::structures::constant_pool::ConstantPool;

pub struct ModulePackagesParser;

pub struct ModulePackagesAttribute {}

impl AttributeParser for ModulePackagesParser {
    fn parse_attribute(&self, parser: &mut Reader, _constant_pool: &ConstantPool) -> AttributeType {
        todo!()
    }

    fn get_name(&self) -> &str {
        MODULE_PACKAGES
    }
}
