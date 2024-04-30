use crate::attributes::attribute_parser::{AttributeParser, AttributeType};
use crate::constants::attribute_constants::INNER_CLASSES;
use crate::reader::Reader;
use crate::structures::constant_pool::ConstantPool;

pub struct InnerClassesParser;

pub struct InnerClass {
    pub inner_class_info_index: u16,
    pub outer_class_info_index: u16,
    pub inner_name_index: u16,
    pub inner_class_access_flags: u16,
}

pub struct InnerClassesAttribute {
    pub classes: Vec<InnerClass>,
}

impl AttributeParser for InnerClassesParser {
    fn parse_attribute(&self, reader: &mut Reader, constant_pool: &ConstantPool) -> AttributeType {
        let number_of_classes = reader.read_u2();
        let mut classes = Vec::new();
        for _ in 0..number_of_classes {
            let inner_class_info_index = reader.read_u2();
            let outer_class_info_index = reader.read_u2();
            let inner_name_index = reader.read_u2();
            let inner_class_access_flags = reader.read_u2();
            classes.push(InnerClass {
                inner_class_info_index,
                outer_class_info_index,
                inner_name_index,
                inner_class_access_flags,
            });
        }
        AttributeType::InnerClasses(InnerClassesAttribute { classes })
    }

    fn get_name(&self) -> &str {
        INNER_CLASSES
    }
}
