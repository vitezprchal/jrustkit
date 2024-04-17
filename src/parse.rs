use crate::reader::Reader;
use crate::constants::attribute_constants::*;
use crate::constants::pool_constants::*;
use crate::structures::constant_pool::ConstantPool;
use crate::structures::constant_pool_entry::ConstantPoolEntry;
use crate::constants::instructions::{*};

pub struct Parser<'a> {
    reader: Reader<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(data: &'a Vec<u8>) -> Self {
        let reader = Reader::new(data);
        Parser { reader }
    }

    pub fn parse(&mut self) {
        let magic = self.reader.read_u4();

        if magic != 0xCAFEBABE {
            println!("Invalid magic");
        }

        let minor = self.reader.read_u2();
        let major = self.reader.read_u2();

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
                    constant_pool.add(ConstantPoolEntry::Long
                    { high_bytes, low_bytes });
                }
                CONSTANT_DOUBLE => {
                    let high_bytes = self.reader.read_u4();
                    let low_bytes = self.reader.read_u4();
                    constant_pool.add(ConstantPoolEntry::Double
                    { high_bytes, low_bytes });
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
                    constant_pool.add(ConstantPoolEntry::Fieldref
                    { class_index, name_and_type_index });
                }
                CONSTANT_METHOD_REF => {
                    let class_index = self.reader.read_u2();
                    let name_and_type_index = self.reader.read_u2();
                    constant_pool.add(ConstantPoolEntry::Methodref
                    { class_index, name_and_type_index });
                }
                CONSTANT_INTERFACE_METHOD_REF => {
                    let class_index = self.reader.read_u2();
                    let name_and_type_index = self.reader.read_u2();
                    constant_pool.add(ConstantPoolEntry::InterfaceMethodref
                    { class_index, name_and_type_index });
                }
                CONSTANT_NAME_AND_TYPE => {
                    let name_index = self.reader.read_u2();
                    let descriptor_index = self.reader.read_u2();
                    constant_pool.add(ConstantPoolEntry::NameAndType
                    { name_index, descriptor_index });
                }
                CONSTANT_METHOD_HANDLE => {
                    let reference_kind = self.reader.read_u1();
                    let reference_index = self.reader.read_u2();
                    constant_pool.add(ConstantPoolEntry::MethodHandle
                    { reference_kind, reference_index });
                }
                CONSTANT_METHOD_TYPE => {
                    let descriptor_index = self.reader.read_u2();
                    constant_pool.add(ConstantPoolEntry::MethodType
                    { descriptor_index });
                }
                CONSTANT_DYNAMIC => {
                    let bootstrap_method_attr_index = self.reader.read_u2();
                    let name_and_type_index = self.reader.read_u2();
                    constant_pool.add(ConstantPoolEntry::Dynamic
                    { bootstrap_method_attr_index, name_and_type_index });
                }
                CONSTANT_INVOKE_DYNAMIC => {
                    let bootstrap_method_attr_index = self.reader.read_u2();
                    let name_and_type_index = self.reader.read_u2();
                    constant_pool.add(ConstantPoolEntry::InvokeDynamic
                    { bootstrap_method_attr_index, name_and_type_index });
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


        let access_flags = self.reader.read_u2();

        let this_class =
            constant_pool.get_class_name(self.reader.read_u2()).unwrap();

        let super_class =
            constant_pool.get_class_name(self.reader.read_u2()).unwrap();

        let interfaces_count = self.reader.read_u2();

        println!("interfaces = {}", interfaces_count);

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

    fn parse_attributes(&mut self, constant_pool: &ConstantPool) {
        let attributes_count = self.reader.read_u2();

        for _ in 0..attributes_count {
            let attribute_name = constant_pool.get_utf8(self.reader.read_u2());
            let attribute_length = self.reader.read_u4();

            match attribute_name {
                Some(name) => {
                    match name {
                        CONSTANT_VALUE => {
                            let value_index = self.reader.read_u2();
                            //println!("class attribute = {} {}", name, value_index);
                        }
                        CODE => {
                            let max_stack = self.reader.read_u2();
                            let max_locals = self.reader.read_u2();
                            let code_length = self.reader.read_u4();

                            let mut i = 0;
                            while i < code_length {
                                let opcode = self.reader.read_u1();

                                println!("i = {}", i);
                                println!("code_length = {}", code_length);
                                match opcode {
                                    _ => { println!("Unknown opcode") }
                                }

                                println!("opcode = {}", opcode);
                                i += 1;
                            }
                            let exception_table_length = self.reader.read_u2();
                            for _ in 0..exception_table_length {
                                let start_pc = self.reader.read_u2();
                                let end_pc = self.reader.read_u2();
                                let handler_pc = self.reader.read_u2();
                                let catch_type = self.reader.read_u2();
                            }
                            self.parse_attributes(&constant_pool);
                        }

                        STACK_MAP_TABLE => {
                            let number_of_entries = self.reader.read_u2();
                            for _ in 0..number_of_entries {
                                let frame_type = self.reader.read_u1();
                            }
                        }

                        EXCEPTIONS => {
                            let number_of_exceptions = self.reader.read_u2();
                            for _ in 0..number_of_exceptions {
                                let exception_index = self.reader.read_u2();
                            }
                        }

                        INNER_CLASSES => {
                            let number_of_classes = self.reader.read_u2();
                            for _ in 0..number_of_classes {
                                let inner_class_info_index = self.reader.read_u2();
                                let outer_class_info_index = self.reader.read_u2();
                                let inner_name_index = self.reader.read_u2();
                                let inner_class_access_flags = self.reader.read_u2();
                            }
                        }

                        //here
                        ENCLOSING_METHOD => {
                            let class_index = self.reader.read_u2();
                            let method_index = self.reader.read_u2();
                        }

                        SYNTHETIC => {}

                        SIGNATURE => {
                            let signature_index = self.reader.read_u2();
                        }

                        SOURCE_DEBUG_EXTENSION => {
                            let debug_extension = self.reader.read_bytes(attribute_length as usize);
                        }

                        LINE_NUMBER_TABLE => {
                            let line_number_table_length = self.reader.read_u2();
                            for _ in 0..line_number_table_length {
                                let start_pc = self.reader.read_u2();
                                let line_number = self.reader.read_u2();
                            }
                        }

                        LOCAL_VARIABLE_TABLE => {
                            let local_variable_table_length = self.reader.read_u2();
                            for _ in 0..local_variable_table_length {
                                let start_pc = self.reader.read_u2();
                                let length = self.reader.read_u2();
                                let name_index = self.reader.read_u2();
                                let descriptor_index = self.reader.read_u2();
                                let index = self.reader.read_u2();
                            }
                        }

                        LOCAL_VARIABLE_TYPE_TABLE => {
                            let local_variable_type_table_length = self.reader.read_u2();
                            for _ in 0..local_variable_type_table_length {
                                let start_pc = self.reader.read_u2();
                                let length = self.reader.read_u2();
                                let name_index = self.reader.read_u2();
                                let signature_index = self.reader.read_u2();
                                let index = self.reader.read_u2();
                            }
                        }

                        DEPRECATED => {}

                        RUNTIME_VISIBLE_ANNOTATIONS => {
                            let num_annotations = self.reader.read_u2();
                            for _ in 0..num_annotations {
                                let type_index = self.reader.read_u2();
                                let num_element_value_pairs = self.reader.read_u2();
                                for _ in 0..num_element_value_pairs {
                                    let element_name_index = self.reader.read_u2();
                                    // parse element value
                                }
                            }
                        }

                        RUNTIME_INVISIBLE_ANNOTATIONS => {
                            let num_annotations = self.reader.read_u2();
                            for _ in 0..num_annotations {
                                let type_index = self.reader.read_u2();
                                let num_element_value_pairs = self.reader.read_u2();
                                for _ in 0..num_element_value_pairs {
                                    let element_name_index = self.reader.read_u2();
                                    // parse element value
                                }
                            }
                        }

                        RUNTIME_VISIBLE_PARAMETER_ANNOTATIONS => {
                            let num_parameters = self.reader.read_u1();
                            for _ in 0..num_parameters {
                                let num_annotations = self.reader.read_u2();
                                for _ in 0..num_annotations {
                                    let type_index = self.reader.read_u2();
                                    let num_element_value_pairs = self.reader.read_u2();
                                    for _ in 0..num_element_value_pairs {
                                        let element_name_index = self.reader.read_u2();
                                        // parse element value
                                    }
                                }
                            }
                        }
                        RUNTIME_INVISIBLE_PARAMETER_ANNOTATIONS => {
                            let num_parameters = self.reader.read_u1();
                            for _ in 0..num_parameters {
                                let num_annotations = self.reader.read_u2();
                                for _ in 0..num_annotations {
                                    let type_index = self.reader.read_u2();
                                    let num_element_value_pairs = self.reader.read_u2();
                                    for _ in 0..num_element_value_pairs {
                                        let element_name_index = self.reader.read_u2();
                                        // parse element value
                                    }
                                }
                            }
                        }

                        ANNOTATION_DEFAULT => {
                            // parse element value
                        }

                        BOOTSTRAP_METHODS => {
                            let num_bootstrap_methods = self.reader.read_u2();
                            for _ in 0..num_bootstrap_methods {
                                let bootstrap_method_ref = self.reader.read_u2();
                                let num_bootstrap_arguments = self.reader.read_u2();
                                for _ in 0..num_bootstrap_arguments {
                                    let bootstrap_argument = self.reader.read_u2();
                                }
                            }
                        }

                        METHOD_PARAMETERS => {
                            let parameters_count = self.reader.read_u1();
                            for _ in 0..parameters_count {
                                let name_index = self.reader.read_u2();
                                let access_flags = self.reader.read_u2();
                            }
                        }

                        MODULE => {
                            let name_index = self.reader.read_u2();
                            let access_flags = self.reader.read_u2();
                            let version_index = self.reader.read_u2();
                        }

                        MODULE_PACKAGES => {
                            let num_packages = self.reader.read_u2();
                            for _ in 0..num_packages {
                                let package_index = self.reader.read_u2();
                            }
                        }

                        MODULE_MAIN_CLASS => {
                            let main_class_index = self.reader.read_u2();
                        }

                        NEST_HOST => {
                            let host_class_index = self.reader.read_u2();
                        }

                        NEST_MEMBERS => {
                            let num_members = self.reader.read_u2();
                            for _ in 0..num_members {
                                let member_class_index = self.reader.read_u2();
                            }
                        }

                        _ => {
                            let info = self.reader.read_bytes(attribute_length as usize);
                            println!("class attribute = {} {}", name, attribute_length);
                        }
                    }
                }
                None => {
                    let info = self.reader.read_bytes(attribute_length as usize);
                    println!("class attribute = {} {}", attribute_name.unwrap(), attribute_length);
                }
            }
        }
    }
}