use crate::structures::constant_pool_entry::ConstantPoolEntry;

pub struct ConstantPool(Vec<ConstantPoolEntry>);

impl ConstantPool {
    pub fn new(size: u16) -> Self {
        ConstantPool(Vec::with_capacity(size as usize))
    }

    pub fn add(&mut self, entry: ConstantPoolEntry) {
        self.0.push(entry);
    }

    pub fn get(&self, index: u16) -> Option<&ConstantPoolEntry> {
        self.0.get(index as usize)
    }

    pub fn get_utf8(&self, index: u16) -> Option<&str> {
        match self.get(index - 1) {
            Some(ConstantPoolEntry::Utf8 { bytes, .. }) => {
                Some(std::str::from_utf8(bytes).unwrap())
            }
            _ => None,
        }
    }

    pub fn get_class_name(&self, index: u16) -> Option<&str> {
        match self.get(index - 1) {
            Some(ConstantPoolEntry::Class { name_index }) => {
                self.get_utf8(*name_index)
            }
            _ => None,
        }
    }
}