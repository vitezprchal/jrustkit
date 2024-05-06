use crate::attributes::attribute_parser::{AttributeParser, AttributeType};
use crate::constants::attribute_constants::METHOD_PARAMETERS;
use crate::reader::Reader;
use crate::structures::constant_pool::ConstantPool;

pub struct MethodParametersParser;

pub struct MethodParametersAttribute {}

impl AttributeParser for MethodParametersParser {
    fn parse_attribute(&self, parser: &mut Reader, constant_pool: &ConstantPool) -> AttributeType {
        todo!()
    }

    fn get_name(&self) -> &str {
        METHOD_PARAMETERS
    }
}
