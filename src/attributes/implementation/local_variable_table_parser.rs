use crate::attributes::attribute_parser::{AttributeParser, AttributeType};
use crate::constants::attribute_constants::LOCAL_VARIABLE_TABLE;
use crate::reader::Reader;
use crate::structures::constant_pool::ConstantPool;

pub struct LocalVariableTableParser;

pub struct LocalVariableTableEntry {
    start_pc: u16,
    length: u16,
    name_index: u16,
    descriptor_index: u16,
    index: u16,
}

pub struct LocalVariableTableAttribute {
    local_variable_table: Vec<LocalVariableTableEntry>,
}

impl AttributeParser for LocalVariableTableParser {
    fn parse_attribute(&self, reader: &mut Reader, _: &ConstantPool) -> AttributeType {
        let length = reader.read_u4();
        let mut local_variable_table = Vec::new();
        for _ in 0..length {
            let start_pc = reader.read_u2();
            let length = reader.read_u2();
            let name_index = reader.read_u2();
            let descriptor_index = reader.read_u2();
            let index = reader.read_u2();
            local_variable_table.push(LocalVariableTableEntry {
                start_pc,
                length,
                name_index,
                descriptor_index,
                index,
            });
        }
        AttributeType::LocalVariableTable(LocalVariableTableAttribute { local_variable_table })
    }

    fn get_name(&self) -> &str {
        LOCAL_VARIABLE_TABLE
    }
}
