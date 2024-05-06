use crate::attributes::attribute_parser::AttributeParsers;
use crate::constants::attribute_constants::*;
use crate::constants::instructions::*;
use crate::constants::pool_constants::*;
use crate::instructions::instruction::*;
use crate::reader::Reader;
use crate::structures::constant_pool::ConstantPool;
use crate::structures::constant_pool_entry::ConstantPoolEntry;
use std::any::Any;

pub struct Parser<'a> {
    pub reader: Reader<'a>,
    pub attribute_parsers: AttributeParsers,
}

impl<'a> Parser<'a> {
    pub fn new(data: &'a Vec<u8>) -> Self {
        let reader = Reader::new(data);
        Parser {
            reader,
            attribute_parsers: AttributeParsers::new(),
        }
    }

    pub fn parse_constant_pool(&mut self) -> ConstantPool {
        let constant_pool_count = self.reader.read_u2();
        let mut constant_pool = ConstantPool::new(constant_pool_count);

        for _ in 0..constant_pool_count - 1 {
            let tag = self.reader.read_u1();
            match tag {
                CONSTANT_UTF8 => {
                    let length = self.reader.read_u2();
                    let bytes = self.reader.read_bytes(length as usize);
                    constant_pool.add(ConstantPoolEntry::Utf8 { length, bytes });
                }
                CONSTANT_INTEGER => {
                    let bytes = self.reader.read_u4();
                    constant_pool.add(ConstantPoolEntry::Integer { bytes });
                }
                CONSTANT_FLOAT => {
                    let bytes = self.reader.read_u4();
                    constant_pool.add(ConstantPoolEntry::Float { bytes });
                }
                CONSTANT_LONG => {
                    let high_bytes = self.reader.read_u4();
                    let low_bytes = self.reader.read_u4();
                    constant_pool.add(ConstantPoolEntry::Long {
                        high_bytes,
                        low_bytes,
                    });
                }
                CONSTANT_DOUBLE => {
                    let high_bytes = self.reader.read_u4();
                    let low_bytes = self.reader.read_u4();
                    constant_pool.add(ConstantPoolEntry::Double {
                        high_bytes,
                        low_bytes,
                    });
                }
                CONSTANT_CLASS => {
                    let name_index = self.reader.read_u2();
                    constant_pool.add(ConstantPoolEntry::Class { name_index });
                }
                CONSTANT_STRING => {
                    let string_index = self.reader.read_u2();
                    constant_pool.add(ConstantPoolEntry::String { string_index });
                }
                CONSTANT_FIELD_REF => {
                    let class_index = self.reader.read_u2();
                    let name_and_type_index = self.reader.read_u2();
                    constant_pool.add(ConstantPoolEntry::Fieldref {
                        class_index,
                        name_and_type_index,
                    });
                }
                CONSTANT_METHOD_REF => {
                    let class_index = self.reader.read_u2();
                    let name_and_type_index = self.reader.read_u2();
                    constant_pool.add(ConstantPoolEntry::Methodref {
                        class_index,
                        name_and_type_index,
                    });
                }
                CONSTANT_INTERFACE_METHOD_REF => {
                    let class_index = self.reader.read_u2();
                    let name_and_type_index = self.reader.read_u2();
                    constant_pool.add(ConstantPoolEntry::InterfaceMethodref {
                        class_index,
                        name_and_type_index,
                    });
                }
                CONSTANT_NAME_AND_TYPE => {
                    let name_index = self.reader.read_u2();
                    let descriptor_index = self.reader.read_u2();
                    constant_pool.add(ConstantPoolEntry::NameAndType {
                        name_index,
                        descriptor_index,
                    });
                }
                CONSTANT_METHOD_HANDLE => {
                    let reference_kind = self.reader.read_u1();
                    let reference_index = self.reader.read_u2();
                    constant_pool.add(ConstantPoolEntry::MethodHandle {
                        reference_kind,
                        reference_index,
                    });
                }
                CONSTANT_METHOD_TYPE => {
                    let descriptor_index = self.reader.read_u2();
                    constant_pool.add(ConstantPoolEntry::MethodType { descriptor_index });
                }
                CONSTANT_DYNAMIC => {
                    let bootstrap_method_attr_index = self.reader.read_u2();
                    let name_and_type_index = self.reader.read_u2();
                    constant_pool.add(ConstantPoolEntry::Dynamic {
                        bootstrap_method_attr_index,
                        name_and_type_index,
                    });
                }
                CONSTANT_INVOKE_DYNAMIC => {
                    let bootstrap_method_attr_index = self.reader.read_u2();
                    let name_and_type_index = self.reader.read_u2();
                    constant_pool.add(ConstantPoolEntry::InvokeDynamic {
                        bootstrap_method_attr_index,
                        name_and_type_index,
                    });
                }
                CONSTANT_MODULE => {
                    let name_index = self.reader.read_u2();
                    constant_pool.add(ConstantPoolEntry::Module { name_index });
                }
                CONSTANT_PACKAGE => {
                    let name_index = self.reader.read_u2();
                    constant_pool.add(ConstantPoolEntry::Package { name_index });
                }
                _ => {
                    println!("Unknown tag: {}", tag);
                }
            }
        }

        constant_pool
    }

    pub fn parse(&mut self) {
        let magic = self.reader.read_u4();

        if magic != 0xCAFEBABE {
            println!("Invalid magic");
        }

        let minor = self.reader.read_u2();
        let major = self.reader.read_u2();
        let constant_pool = self.parse_constant_pool();
        let access_flags = self.reader.read_u2();
        let this_class = constant_pool.get_class_name(self.reader.read_u2()).unwrap();
        let super_class = constant_pool.get_class_name(self.reader.read_u2()).unwrap();

        let interfaces_count = self.reader.read_u2();

        for _ in 0..interfaces_count {
            let interface = self.reader.read_u2();
            println!("interface = {}", interface);
        }

        let fields_count = self.reader.read_u2();

        for _ in 0..fields_count {
            let access_flags = self.reader.read_u2();
            let name_index = constant_pool.get_utf8(self.reader.read_u2()).unwrap();
            let descriptor_index = constant_pool.get_utf8(self.reader.read_u2()).unwrap();

            println!("field = {} {}", name_index, descriptor_index);

            self.parse_attributes(&constant_pool)
        }

        let methods_count = self.reader.read_u2();

        for _ in 0..methods_count {
            let access_flags = self.reader.read_u2();
            let name_index = constant_pool.get_utf8(self.reader.read_u2()).unwrap();
            let descriptor_index = constant_pool.get_utf8(self.reader.read_u2()).unwrap();

            println!("method = {} {}", name_index, descriptor_index);

            self.parse_attributes(&constant_pool);
        }

        self.parse_attributes(&constant_pool);
    }

    pub fn parse_attributes(&mut self, constant_pool: &ConstantPool) {
        let attributes_count = self.reader.read_u2();

        for _ in 0..attributes_count {
            let attribute_name = constant_pool.get_utf8(self.reader.read_u2());
            let attribute_length = self.reader.read_u4();

            println!("{}", attribute_name.unwrap());
            println!("{}", attribute_length);

            match attribute_name {
                Some(name) => {
                    self.attribute_parsers
                        .parse_attribute(name, &mut self.reader, &constant_pool);

                    if name == CODE {
                        self.parse_attributes(&constant_pool);
                    }
                }

                None => {
                    let info = self.reader.read_bytes(attribute_length as usize);
                    println!(
                        "class attribute = {} {}",
                        attribute_name.unwrap(),
                        attribute_length
                    );
                }
            }
        }
    }
}
