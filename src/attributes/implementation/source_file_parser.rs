use crate::attributes::attribute_parser::{AttributeParser, AttributeType};
use crate::constants::attribute_constants::SOURCE_FILE;
use crate::parse::Parser;
use crate::reader::Reader;
use crate::structures::constant_pool::ConstantPool;

pub struct SourceFileAttributeParser;

pub struct SourceFileAttribute {
    pub source_file: u16,
}

impl AttributeParser for SourceFileAttributeParser {
    fn parse_attribute(&self, reader: &mut Reader, constant_pool: &ConstantPool) -> AttributeType {
        let source_file_index = reader.read_u2();

        AttributeType::SourceFile(SourceFileAttribute {
            source_file: source_file_index,
        })
    }

    fn get_name(&self) -> &str {
        SOURCE_FILE
    }
}
