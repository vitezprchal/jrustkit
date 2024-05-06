use crate::attributes::attribute_parser::{AttributeParser, AttributeType};
use crate::constants::attribute_constants::STACK_MAP_TABLE;
use crate::parse::Parser;
use crate::reader::Reader;
use crate::structures::constant_pool::ConstantPool;

pub struct StackMapTableAttributeParser;

pub struct StackMapFrame {
    pub frame_type: u8,
}

pub struct StackMapTableAttribute {
    pub entries: Vec<StackMapFrame>,
}

impl AttributeParser for StackMapTableAttributeParser {
    fn parse_attribute(&self, reader: &mut Reader, _: &ConstantPool) -> AttributeType {
        println!("parse");
        let stack_map_table_length = reader.read_u2();
        let mut entries: Vec<StackMapFrame> = Vec::new();
        println!("StackMapTable length: {}", stack_map_table_length);
        for _ in 0..stack_map_table_length {
            let frame_type = reader.read_u1();
            entries.push(StackMapFrame { frame_type });
        }

        AttributeType::StackMapTable(StackMapTableAttribute { entries })
    }

    fn get_name(&self) -> &str {
        "STACK_MAP_TABLE"
    }
}
