use crate::attributes::attribute_parser::{AttributeParser, AttributeType};
use crate::constants::attribute_constants::RUNTIME_VISIBLE_PARAMETER_ANNOTATIONS;
use crate::reader::Reader;
use crate::structures::constant_pool::ConstantPool;

pub struct RuntimeVisibleParameterAnnotationsParser;

impl AttributeParser for RuntimeVisibleParameterAnnotationsParser {
    fn parse_attribute(&self, reader: &mut Reader, constant_pool: &ConstantPool) -> AttributeType {
        todo!()
    }

    fn get_name(&self) -> &str {
        RUNTIME_VISIBLE_PARAMETER_ANNOTATIONS
    }
}
