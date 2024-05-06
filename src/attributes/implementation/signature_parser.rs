use crate::attributes::attribute_parser::{AttributeParser, AttributeType};
use crate::constants::attribute_constants::SIGNATURE;
use crate::parse::Parser;
use crate::reader::Reader;
use crate::structures::constant_pool::ConstantPool;

pub struct SignatureParser;

pub struct SignatureAttribute {
    signature_index: u16,
}

impl AttributeParser for SignatureParser {
    fn parse_attribute(&self, reader: &mut Reader, _: &ConstantPool) -> AttributeType {
        let signature_index = reader.read_u2();
        AttributeType::Signature(SignatureAttribute { signature_index })
    }

    fn get_name(&self) -> &str {
        SIGNATURE
    }
}
