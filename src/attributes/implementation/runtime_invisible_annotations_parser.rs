use crate::attributes::attribute_parser::{AttributeParser, AttributeType};
use crate::constants::attribute_constants::RUNTIME_INVISIBLE_ANNOTATIONS;
use crate::reader::Reader;
use crate::structures::constant_pool::ConstantPool;

pub struct RuntimeInvisibleAnnotationsParser;

pub struct RuntimeInvisibleAnnotationsAttribute {
    annotations: Vec<()>,
}

impl AttributeParser for RuntimeInvisibleAnnotationsParser {
    fn parse_attribute(&self, reader: &mut Reader, constant_pool: &ConstantPool) -> AttributeType {
        let length = reader.read_u2();
        let mut annotations = Vec::new();
        for _ in 0..length {
            let element_name_index = reader.read_u2();
            todo!()
        }
        AttributeType::RuntimeInvisibleAnnotations(RuntimeInvisibleAnnotationsAttribute {
            annotations,
        })
    }

    fn get_name(&self) -> &str {
        RUNTIME_INVISIBLE_ANNOTATIONS
    }
}
