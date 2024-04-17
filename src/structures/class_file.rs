use crate::structures::field_info::FieldInfo;
use crate::structures::method_info::MethodInfo;

pub struct ClassFile {
    minor_version: u16,
    major_version: u16,
    access_flags: u16,
    this_class: String,
    super_class: String,
    interfaces: Vec<String>,
    fields_count: u16,
    fields: Vec<FieldInfo>,
    methods_count: u16,
    methods: Vec<MethodInfo>,
    //attributes_count: u16,
    //attributes: Vec<AttributeInfo>,
}
