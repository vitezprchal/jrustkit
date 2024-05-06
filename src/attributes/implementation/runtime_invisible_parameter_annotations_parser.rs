use crate::attributes::attribute_parser::{AttributeParser, AttributeType};
use crate::constants::attribute_constants::RUNTIME_INVISIBLE_PARAMETER_ANNOTATIONS;
use crate::reader::Reader;
use crate::structures::constant_pool::ConstantPool;

pub struct RuntimeInvisibleParameterAnnotationsParser;

pub struct RuntimeInvisibleParameterAnnotationsAttribute {}

impl AttributeParser for RuntimeInvisibleParameterAnnotationsParser {
    fn parse_attribute(&self, reader: &mut Reader, constant_pool: &ConstantPool) -> AttributeType {
        todo!("RuntimeInvisibleParameterAnnotationsParser")
    }

    fn get_name(&self) -> &str {
        RUNTIME_INVISIBLE_PARAMETER_ANNOTATIONS
    }
}
