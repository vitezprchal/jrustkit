use std::fmt;

pub enum ConstantPoolEntry {
    Class {
        name_index: u16,
    },
    Fieldref {
        class_index: u16,
        name_and_type_index: u16,
    },
    Methodref {
        class_index: u16,
        name_and_type_index: u16,
    },
    InterfaceMethodref {
        class_index: u16,
        name_and_type_index: u16,
    },
    String {
        string_index: u16,
    },
    Integer {
        bytes: u32,
    },
    Float {
        bytes: u32,
    },
    Long {
        high_bytes: u32,
        low_bytes: u32,
    },
    Double {
        high_bytes: u32,
        low_bytes: u32,
    },
    NameAndType {
        name_index: u16,
        descriptor_index: u16,
    },
    Utf8 {
        length: u16,
        bytes: Vec<u8>,
    },
    MethodHandle {
        reference_kind: u8,
        reference_index: u16,
    },
    MethodType {
        descriptor_index: u16,
    },
    Dynamic {
        bootstrap_method_attr_index: u16,
        name_and_type_index: u16,
    },
    InvokeDynamic {
        bootstrap_method_attr_index: u16,
        name_and_type_index: u16,
    },
    Module {
        name_index: u16,
    },
    Package {
        name_index: u16,
    },
}

impl fmt::Display for ConstantPoolEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ConstantPoolEntry::Class { name_index } => {
                write!(f, "Class with name index: {}", name_index)
            }
            ConstantPoolEntry::Fieldref {
                class_index,
                name_and_type_index,
            } => write!(
                f,
                "Fieldref with class index: {} and name and type index: {}",
                class_index, name_and_type_index
            ),
            ConstantPoolEntry::Methodref {
                class_index,
                name_and_type_index,
            } => write!(
                f,
                "Methodref with class index: {} and name and type index: {}",
                class_index, name_and_type_index
            ),
            ConstantPoolEntry::InterfaceMethodref {
                class_index,
                name_and_type_index,
            } => write!(
                f,
                "InterfaceMethodref with class index: {} and name and type index: {}",
                class_index, name_and_type_index
            ),
            ConstantPoolEntry::String { string_index } => {
                write!(f, "String with string index: {}", string_index)
            }
            ConstantPoolEntry::Integer { bytes } => write!(f, "Integer with bytes: {}", bytes),
            ConstantPoolEntry::Float { bytes } => write!(f, "Float with bytes: {}", bytes),
            ConstantPoolEntry::Long {
                high_bytes,
                low_bytes,
            } => write!(
                f,
                "Long with high bytes: {} and low bytes: {}",
                high_bytes, low_bytes
            ),
            ConstantPoolEntry::Double {
                high_bytes,
                low_bytes,
            } => write!(
                f,
                "Double with high bytes: {} and low bytes: {}",
                high_bytes, low_bytes
            ),
            ConstantPoolEntry::NameAndType {
                name_index,
                descriptor_index,
            } => write!(
                f,
                "NameAndType with name index: {} and descriptor index: {}",
                name_index, descriptor_index
            ),
            ConstantPoolEntry::Utf8 { length, bytes } => {
                write!(f, "Utf8 with length: {} and bytes: {:?}", length, bytes)
            }
            ConstantPoolEntry::MethodHandle {
                reference_kind,
                reference_index,
            } => write!(
                f,
                "MethodHandle with reference kind: {} and reference index: {}",
                reference_kind, reference_index
            ),
            ConstantPoolEntry::MethodType { descriptor_index } => {
                write!(f, "MethodType with descriptor index: {}", descriptor_index)
            }
            ConstantPoolEntry::Dynamic {
                bootstrap_method_attr_index,
                name_and_type_index,
            } => write!(
                f,
                "Dynamic with bootstrap method attr index: {} and name and type index: {}",
                bootstrap_method_attr_index, name_and_type_index
            ),
            ConstantPoolEntry::InvokeDynamic {
                bootstrap_method_attr_index,
                name_and_type_index,
            } => write!(
                f,
                "InvokeDynamic with bootstrap method attr index: {} and name and type index: {}",
                bootstrap_method_attr_index, name_and_type_index
            ),
            ConstantPoolEntry::Module { name_index } => {
                write!(f, "Module with name index: {}", name_index)
            }
            ConstantPoolEntry::Package { name_index } => {
                write!(f, "Package with name index: {}", name_index)
            }
        }
    }
}
