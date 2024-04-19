use crate::attributes::attribute_parser::{AttributeParser, AttributeType};
use crate::constants::attribute_constants::SOURCE_DEBUG_EXTENSION;
use crate::reader::Reader;
use crate::structures::constant_pool::ConstantPool;

pub struct SourceDebugExtensionParser;

pub struct SourceDebugExtensionAttribute {
    debug_extension: Vec<u8>,
}

impl AttributeParser for SourceDebugExtensionParser {
    fn parse_attribute(&self, reader: &mut Reader, _: &ConstantPool) -> AttributeType {
        let length = reader.read_u4();
        let debug_extension = reader.read_bytes(length as usize);
        AttributeType::SourceDebugExtension(SourceDebugExtensionAttribute { debug_extension })
    }

    fn get_name(&self) -> &str {
        SOURCE_DEBUG_EXTENSION
    }
}