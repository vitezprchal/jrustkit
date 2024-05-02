use crate::attributes::attribute_parser::{AttributeParser, AttributeType};
use crate::constants::attribute_constants::LINE_NUMBER_TABLE;
use crate::reader::Reader;
use crate::structures::constant_pool::ConstantPool;

pub struct LineNumberTableParser;

pub struct LineNumberTableEntry {
    start_pc: u16,
    line_number: u16,
}

pub struct LineNumberTableAttribute {
    line_number_table: Vec<LineNumberTableEntry>,
}

impl AttributeParser for LineNumberTableParser {
    fn parse_attribute(&self, reader: &mut Reader, _: &ConstantPool) -> AttributeType {
        let length = reader.read_u2();
        let mut line_number_table = Vec::new();
        for _ in 0..length {
            let start_pc = reader.read_u2();
            let line_number = reader.read_u2();
            line_number_table.push(LineNumberTableEntry { start_pc, line_number });
        }
        AttributeType::LineNumberTable(LineNumberTableAttribute { line_number_table })
    }

    fn get_name(&self) -> &str {
        LINE_NUMBER_TABLE
    }
}