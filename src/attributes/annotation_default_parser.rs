use crate::attributes::attribute_parser::{AttributeParser, AttributeType};
use crate::constants::attribute_constants::ANNOTATION_DEFAULT;
use crate::reader::Reader;
use crate::structures::constant_pool::ConstantPool;

pub struct AnnotationDefaultParser;

pub struct AnnotationDefaultAttribute {}

impl AttributeParser for AnnotationDefaultParser {
    fn parse_attribute(&self, parser: &mut Reader, constant_pool: &ConstantPool) -> AttributeType {
        todo!()
    }

    fn get_name(&self) -> &str {
        ANNOTATION_DEFAULT
    }
}
