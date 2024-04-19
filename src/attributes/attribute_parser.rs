use crate::attributes::{code_parser, constant_value_parser, exceptions_parser, signature_parser};
use crate::attributes::annotation_default_parser::{AnnotationDefaultAttribute, AnnotationDefaultParser};
use crate::attributes::bootstrap_methods_parser::{BootstrapMethodsAttribute, BootstrapMethodsParser};
use crate::attributes::code_parser::CodeAttributeParser;
use crate::attributes::constant_value_parser::ConstantValueParser;
use crate::attributes::deprecated_parser::{DeprecatedAttribute, DeprecatedParser};
use crate::attributes::enclosing_method_parser::{EnclosingMethodAttribute, EnclosingMethodParser};
use crate::attributes::exceptions_parser::ExceptionsParser;
use crate::attributes::inner_classes_parser::{InnerClassesAttribute, InnerClassesParser};
use crate::attributes::line_number_table_parser::{LineNumberTableAttribute, LineNumberTableParser};
use crate::attributes::local_variable_table_parser::{LocalVariableTableAttribute, LocalVariableTableParser};
use crate::attributes::method_parameters::{MethodParametersAttribute, MethodParametersParser};
use crate::attributes::module_main_class_parser::{ModuleMainClassAttribute, ModuleMainClassParser};
use crate::attributes::module_packages_parser::{ModulePackagesAttribute, ModulePackagesParser};
use crate::attributes::module_parser::{ModuleAttribute, ModuleParser};
use crate::attributes::nest_host_parser::{NestHostAttribute, NestHostParser};
use crate::attributes::nest_members_parser::{NestMembersAttribute, NestMembersParser};
use crate::attributes::runtime_invisible_annotations_parser::{RuntimeInvisibleAnnotationsAttribute, RuntimeInvisibleAnnotationsParser};
use crate::attributes::runtime_invisible_parameter_annotations_parser::{RuntimeInvisibleParameterAnnotationsAttribute, RuntimeInvisibleParameterAnnotationsParser};
use crate::attributes::runtime_visible_annotations::{RuntimeVisibleAnnotationsAttribute, RuntimeVisibleAnnotationsParser};
use crate::attributes::runtime_visible_parameter_annotations::RuntimeVisibleParameterAnnotationsParser;
use crate::attributes::signature_parser::SignatureParser;
use crate::attributes::source_debug_extension_parser::{SourceDebugExtensionAttribute, SourceDebugExtensionParser};
use crate::attributes::source_file_parser::{SourceFileAttribute, SourceFileAttributeParser};
use crate::attributes::stack_map_table_parser::{StackMapTableAttribute, StackMapTableAttributeParser};
use crate::attributes::synthetic_parser::{SyntheticAttribute, SyntheticParser};
use crate::reader::Reader;
use crate::structures::constant_pool::ConstantPool;

pub enum AttributeType {
    ConstantValue(constant_value_parser::ConstantValueAttribute),
    Code(code_parser::CodeAttribute),
    StackMapTable(StackMapTableAttribute),
    Signature(signature_parser::SignatureAttribute),
    Exceptions(exceptions_parser::ExceptionsAttribute),
    SourceFile(SourceFileAttribute),
    InnerClasses(InnerClassesAttribute),
    EnclosingMethod(EnclosingMethodAttribute),
    Synthetic(SyntheticAttribute),
    SourceDebugExtension(SourceDebugExtensionAttribute),
    LineNumberTable(LineNumberTableAttribute),
    LocalVariableTable(LocalVariableTableAttribute),
    Deprecated(DeprecatedAttribute),
    RuntimeInvisibleAnnotations(RuntimeInvisibleAnnotationsAttribute),
    RuntimeInvisibleParameterAnnotations(RuntimeInvisibleParameterAnnotationsAttribute),
    RuntimeVisibleAnnotations(RuntimeVisibleAnnotationsAttribute),
    AnnotationDefault(AnnotationDefaultAttribute),
    BootstrapMethods(BootstrapMethodsAttribute),
    MethodParameters(MethodParametersAttribute),
    Module(ModuleAttribute),
    ModulePackages(ModulePackagesAttribute),
    ModuleMainClass(ModuleMainClassAttribute),
    NestHost(NestHostAttribute),
    NestMembers(NestMembersAttribute),
}

pub trait AttributeParser {
    fn parse_attribute(&self, parser: &mut Reader, constant_pool: &ConstantPool) -> AttributeType;
    fn get_name(&self) -> &str;
}

pub struct AttributeParsers {
    parsers: Vec<Box<dyn AttributeParser>>,
}

impl AttributeParsers {
    pub fn new() -> Self {
        Self {
            parsers: vec![
                Box::new(ConstantValueParser),
                Box::new(CodeAttributeParser),
                Box::new(StackMapTableAttributeParser),
                Box::new(SignatureParser),
                Box::new(ExceptionsParser),
                Box::new(SourceFileAttributeParser),
                Box::new(InnerClassesParser),
                Box::new(EnclosingMethodParser),
                Box::new(SyntheticParser),
                Box::new(SourceDebugExtensionParser),
                Box::new(LineNumberTableParser),
                Box::new(LocalVariableTableParser),
                Box::new(DeprecatedParser),
                Box::new(RuntimeInvisibleAnnotationsParser),
                Box::new(RuntimeInvisibleParameterAnnotationsParser),
                Box::new(RuntimeVisibleAnnotationsParser),
                Box::new(RuntimeVisibleParameterAnnotationsParser),
                Box::new(AnnotationDefaultParser),
                Box::new(BootstrapMethodsParser),
                Box::new(MethodParametersParser),
                Box::new(ModuleParser),
                Box::new(ModulePackagesParser),
                Box::new(ModuleMainClassParser),
                Box::new(NestHostParser),
                Box::new(NestMembersParser),
            ]
        }
    }

    pub fn parse_attribute(&self, attribute_name: &str,
                           parser: &mut Reader, constant_pool: &ConstantPool) ->
                           Option<AttributeType>
    {
        for self_parser in &self.parsers {
            if self_parser.get_name() == attribute_name {
                return Some(self_parser.parse_attribute(parser, constant_pool));
            }
        }
        None
    }
}
