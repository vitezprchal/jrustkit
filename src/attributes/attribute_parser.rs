use crate::attributes::implementation::annotation_default_parser::*;
use crate::attributes::implementation::bootstrap_methods_parser::*;
use crate::attributes::implementation::code_parser::*;
use crate::attributes::implementation::constant_value_parser::*;
use crate::attributes::implementation::deprecated_parser::*;
use crate::attributes::implementation::enclosing_method_parser::*;
use crate::attributes::implementation::exceptions_parser::*;
use crate::attributes::implementation::inner_classes_parser::*;
use crate::attributes::implementation::line_number_table_parser::*;
use crate::attributes::implementation::local_variable_table_parser::*;
use crate::attributes::implementation::method_parameters::*;
use crate::attributes::implementation::module_main_class_parser::*;
use crate::attributes::implementation::module_packages_parser::*;
use crate::attributes::implementation::module_parser::*;
use crate::attributes::implementation::nest_host_parser::*;
use crate::attributes::implementation::nest_members_parser::*;
use crate::attributes::implementation::runtime_invisible_annotations_parser::*;
use crate::attributes::implementation::runtime_invisible_parameter_annotations_parser::*;
use crate::attributes::implementation::runtime_visible_annotations::*;
use crate::attributes::implementation::runtime_visible_parameter_annotations::*;
use crate::attributes::implementation::signature_parser::*;
use crate::attributes::implementation::source_debug_extension_parser::*;
use crate::attributes::implementation::source_file_parser::*;
use crate::attributes::implementation::stack_map_table_parser::*;
use crate::attributes::implementation::synthetic_parser::*;
use crate::attributes::implementation::*;
use crate::reader::Reader;
use crate::structures::constant_pool::ConstantPool;

macro_rules! attribute_parser {
    ($($parser:ident),*) => {
        vec![
            $(
                Box::new($parser),
            )*
        ]
    };
}

pub enum AttributeType {
    ConstantValue(ConstantValueAttribute),
    Code(CodeAttribute),
    StackMapTable(StackMapTableAttribute),
    Signature(SignatureAttribute),
    Exceptions(ExceptionsAttribute),
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
            parsers: attribute_parser![
                ConstantValueParser,
                CodeAttributeParser,
                StackMapTableAttributeParser,
                SignatureParser,
                ExceptionsParser,
                SourceFileAttributeParser,
                InnerClassesParser,
                EnclosingMethodParser,
                SyntheticParser,
                SourceDebugExtensionParser,
                LineNumberTableParser,
                LocalVariableTableParser,
                DeprecatedParser,
                RuntimeInvisibleAnnotationsParser,
                RuntimeInvisibleParameterAnnotationsParser,
                RuntimeVisibleAnnotationsParser,
                AnnotationDefaultParser,
                BootstrapMethodsParser,
                MethodParametersParser,
                ModuleParser,
                ModulePackagesParser,
                ModuleMainClassParser,
                NestHostParser,
                NestMembersParser
            ],
        }
    }

    pub fn parse_attribute(
        &self,
        attribute_name: &str,
        parser: &mut Reader,
        constant_pool: &ConstantPool,
    ) -> Option<AttributeType> {
        for self_parser in &self.parsers {
            if self_parser.get_name() == attribute_name {
                return Some(self_parser.parse_attribute(parser, constant_pool));
            }
        }
        None
    }
}
