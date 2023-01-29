//! Module to construct a Java (JVM) raw class file bytes from the class fileobject model.

use crate::{
    error::SerializeError,
    model::{
        attributes::AttributeInfo, constant_pool::types::CpInfo, ClassFile, FieldInfo, MethodInfo,
    },
    rw::writer::Writer,
};
use std::io::Write;

pub type SerializeResult<T> = Result<T, SerializeError>;

/// The Serializer takes in the JVM class file object model, and writes a stream of valid
/// JVM bytecode to the supplied writer.
pub struct Serializer<'a, W: Write> {
    writer: Writer<'a, W>,
}

impl<'a, W: Write> Serializer<'a, W> {
    pub fn new(writer: Writer<'a, W>) -> Self {
        Serializer { writer }
    }

    /// Serialize the attributes of the class file.
    fn serialize_attributes(&mut self, attributes: &[AttributeInfo]) -> SerializeResult<()> {
        for attribute in attributes {
            match attribute {
                AttributeInfo::SourceFile {
                    attribute_name_index,
                    attribute_length,
                    sourcefile_index,
                } => {
                    self.writer.write_unsigned_short(*attribute_name_index)?;
                    self.writer.write_unsigned_int(*attribute_length)?;
                    self.writer.write_unsigned_short(*sourcefile_index)?;
                }

                AttributeInfo::ConstantValue {
                    attribute_name_index,
                    attribute_length,
                    constantvalue_index,
                } => {
                    self.writer.write_unsigned_short(*attribute_name_index)?;
                    self.writer.write_unsigned_int(*attribute_length)?;
                    self.writer.write_unsigned_short(*constantvalue_index)?;
                }

                AttributeInfo::Code {
                    attribute_name_index,
                    attribute_length,
                    max_stack,
                    max_locals,
                    code_length,
                    code,
                    exception_table_length,
                    exception_table,
                    code_attributes_count,
                    code_attributes,
                } => {
                    self.writer.write_unsigned_short(*attribute_name_index)?;
                    self.writer.write_unsigned_int(*attribute_length)?;
                    self.writer.write_unsigned_short(*max_stack)?;
                    self.writer.write_unsigned_short(*max_locals)?;

                    self.writer.write_unsigned_int(*code_length)?;
                    for b in code {
                        self.writer.write_unsigned_byte(*b)?;
                    }

                    self.writer.write_unsigned_short(*exception_table_length)?;
                    for ehandler in exception_table {
                        self.writer.write_unsigned_short(ehandler.start_pc)?;
                        self.writer.write_unsigned_short(ehandler.end_pc)?;
                        self.writer.write_unsigned_short(ehandler.handler_pc)?;

                        let mut catch_type = ehandler.catch_type;
                        if catch_type != 0 {
                            catch_type += 1;
                        }
                        self.writer.write_unsigned_short(catch_type)?;
                    }

                    self.writer.write_unsigned_short(*code_attributes_count)?;
                    self.serialize_attributes(code_attributes)?;
                }

                AttributeInfo::Exceptions {
                    attribute_name_index,
                    attribute_length,
                    number_of_exceptions,
                    exception_index_table,
                } => {
                    self.writer.write_unsigned_short(*attribute_name_index)?;
                    self.writer.write_unsigned_int(*attribute_length)?;
                    self.writer.write_unsigned_short(*number_of_exceptions)?;

                    for idx in exception_index_table {
                        self.writer
                            .write_unsigned_short(if *idx == 0 { 0 } else { idx + 1 })?;
                    }
                }

                AttributeInfo::LineNumberTable {
                    attribute_name_index,
                    attribute_length,
                    line_number_table_length,
                    line_number_table,
                } => {
                    self.writer.write_unsigned_short(*attribute_name_index)?;
                    self.writer.write_unsigned_int(*attribute_length)?;
                    self.writer
                        .write_unsigned_short(*line_number_table_length)?;

                    for line_number in line_number_table {
                        self.writer.write_unsigned_short(line_number.start_pc)?;
                        self.writer.write_unsigned_short(line_number.line_number)?;
                    }
                }

                AttributeInfo::LocalVariableTable {
                    attribute_name_index,
                    attribute_length,
                    local_variable_table_length,
                    local_variable_table,
                } => {
                    self.writer.write_unsigned_short(*attribute_name_index)?;
                    self.writer.write_unsigned_int(*attribute_length)?;
                    self.writer
                        .write_unsigned_short(*local_variable_table_length)?;

                    for local_var in local_variable_table {
                        self.writer.write_unsigned_short(local_var.start_pc)?;
                        self.writer.write_unsigned_short(local_var.length)?;
                        self.writer.write_unsigned_short(local_var.name_index)?;
                        self.writer
                            .write_unsigned_short(local_var.descriptor_index)?;
                        self.writer.write_unsigned_short(local_var.index)?;
                    }
                }

                AttributeInfo::StackMapTable {
                    attribute_name_index,
                    attribute_length,
                    number_of_entries,
                    entries,
                } => {}

                AttributeInfo::InnerClasses {
                    attribute_name_index,
                    attribute_length,
                    number_of_classes,
                    classes,
                } => {}

                AttributeInfo::EnclosingMethod {
                    attribute_name_index,
                    attribute_length,
                    class_index,
                    method_index,
                } => {}

                AttributeInfo::Synthetic {
                    attribute_name_index,
                    attribute_length,
                } => {}

                AttributeInfo::Signature {
                    attribute_name_index,
                    attribute_length,
                    signature_index,
                } => {}

                AttributeInfo::SourceDebugExtension {
                    attribute_name_index,
                    attribute_length,
                    debug_extension,
                } => {}

                AttributeInfo::LocalVariableTypeTable {
                    attribute_name_index,
                    attribute_length,
                    local_variable_type_table_length,
                    local_variable_type_table,
                } => {}

                AttributeInfo::Deprecated {
                    attribute_name_index,
                    attribute_length,
                } => {}

                AttributeInfo::RuntimeVisibleAnnotations {
                    attribute_name_index,
                    attribute_length,
                    num_annotations,
                    annotations,
                } => {}

                AttributeInfo::RuntimeInvisibleAnnotations {
                    attribute_name_index,
                    attribute_length,
                    num_annotations,
                    annotations,
                } => {}

                AttributeInfo::RuntimeVisibleParameterAnnotations {
                    attribute_name_index,
                    attribute_length,
                    num_parameters,
                    parameter_annotations,
                } => {}

                AttributeInfo::RuntimeInvisibleParameterAnnotations {
                    attribute_name_index,
                    attribute_length,
                    num_parameters,
                    parameter_annotations,
                } => {}

                AttributeInfo::RuntimeVisibleTypeAnnotations {
                    attribute_name_index,
                    attribute_length,
                    num_annotations,
                    annotations,
                } => {}

                AttributeInfo::RuntimeInvisibleTypeAnnotations {
                    attribute_name_index,
                    attribute_length,
                    num_annotations,
                    annotations,
                } => {}

                AttributeInfo::AnnotationDefault {
                    attribute_name_index,
                    attribute_length,
                    default_value,
                } => {}

                AttributeInfo::BootstrapMethods {
                    attribute_name_index,
                    attribute_length,
                    num_bootstrap_methods,
                    bootstrap_methods,
                } => {}

                AttributeInfo::MethodParameters {
                    attribute_name_index,
                    attribute_length,
                    parameters_count,
                    parameters,
                } => {}

                AttributeInfo::Module {
                    attribute_name_index,
                    attribute_length,
                    module_name_index,
                    module_flags,
                    module_version_index,
                    requires_count,
                    requires,
                    exports_count,
                    exports,
                    opens_count,
                    opens,
                    uses_count,
                    uses_index,
                    provides_count,
                    provides,
                } => {}

                AttributeInfo::ModulePackages {
                    attribute_name_index,
                    attribute_length,
                    package_count,
                    package_index,
                } => {}

                AttributeInfo::ModuleMainClass {
                    attribute_name_index,
                    attribute_length,
                    main_class_index,
                } => {}

                AttributeInfo::NestHost {
                    attribute_name_index,
                    attribute_length,
                    host_class_index,
                } => {}

                AttributeInfo::NestMembers {
                    attribute_name_index,
                    attribute_length,
                    number_of_classes,
                    classes,
                } => {}

                AttributeInfo::Record {
                    attribute_name_index,
                    attribute_length,
                    components_count,
                    components,
                } => {}

                AttributeInfo::PermittedSubclasses {
                    attribute_name_index,
                    attribute_length,
                    number_of_classes,
                    classes,
                } => {}
            }
        }

        Ok(())
    }

    /// Serialize the fields of the class file.
    fn serialize_fields(&mut self, fields: &[FieldInfo]) -> SerializeResult<()> {
        for field in fields {
            self.writer.write_unsigned_short(field.access_flags)?;
            self.writer.write_unsigned_short(field.name_index)?;
            self.writer.write_unsigned_short(field.descriptor_index)?;
            self.writer.write_unsigned_short(field.attributes_count)?;
            self.serialize_attributes(&field.attributes)?;
        }

        Ok(())
    }

    /// Deserialize the methods of the class file.
    fn serialize_methods(&mut self, methods: &[MethodInfo]) -> SerializeResult<()> {
        for method in methods {
            self.writer.write_unsigned_short(method.access_flags)?;
            self.writer.write_unsigned_short(method.name_index)?;
            self.writer.write_unsigned_short(method.descriptor_index)?;
            self.writer.write_unsigned_short(method.attributes_count)?;
            self.serialize_attributes(&method.attributes)?;
        }
        Ok(())
    }

    /// Serialize the contents of the Constant Pool.
    fn serialize_constant_pool(
        &mut self,
        constant_pool: &Vec<Option<CpInfo>>,
    ) -> SerializeResult<()> {
        for cp_info in constant_pool {
            if let Some(cp_info) = cp_info {
                match cp_info {
                    CpInfo::ConstantMethodrefInfo {
                        tag,
                        class_index,
                        name_and_type_index,
                    } => {
                        self.writer.write_unsigned_byte(*tag)?;
                        self.writer.write_unsigned_short(*class_index)?;
                        self.writer.write_unsigned_short(*name_and_type_index)?;
                    }

                    CpInfo::ConstantClassInfo { tag, name_index } => {
                        self.writer.write_unsigned_byte(*tag)?;
                        self.writer.write_unsigned_short(*name_index)?;
                    }

                    CpInfo::ConstantFieldrefInfo {
                        tag,
                        class_index,
                        name_and_type_index,
                    } => {
                        self.writer.write_unsigned_byte(*tag)?;
                        self.writer.write_unsigned_short(*class_index)?;
                        self.writer.write_unsigned_short(*name_and_type_index)?;
                    }

                    CpInfo::ConstantInterfaceMethodrefInfo {
                        tag,
                        class_index,
                        name_and_type_index,
                    } => {
                        self.writer.write_unsigned_byte(*tag)?;
                        self.writer.write_unsigned_short(*class_index)?;
                        self.writer.write_unsigned_short(*name_and_type_index)?;
                    }

                    CpInfo::ConstantStringInfo { tag, string_index } => {
                        self.writer.write_unsigned_byte(*tag)?;
                        self.writer.write_unsigned_short(*string_index)?;
                    }

                    CpInfo::ConstantIntegerInfo { tag, bytes } => {
                        self.writer.write_unsigned_byte(*tag)?;
                        self.writer.write_unsigned_int(*bytes)?;
                    }

                    CpInfo::ConstantFloatInfo { tag, bytes } => {
                        self.writer.write_unsigned_byte(*tag)?;
                        self.writer.write_unsigned_int(*bytes)?;
                    }

                    CpInfo::ConstantLongInfo {
                        tag,
                        high_bytes,
                        low_bytes,
                    } => {
                        self.writer.write_unsigned_byte(*tag)?;
                        self.writer.write_unsigned_int(*high_bytes)?;
                        self.writer.write_unsigned_int(*low_bytes)?;
                    }

                    CpInfo::ConstantDoubleInfo {
                        tag,
                        high_bytes,
                        low_bytes,
                    } => {
                        self.writer.write_unsigned_byte(*tag)?;
                        self.writer.write_unsigned_int(*high_bytes)?;
                        self.writer.write_unsigned_int(*low_bytes)?;
                    }

                    CpInfo::ConstantNameAndTypeInfo {
                        tag,
                        name_index,
                        descriptor_index,
                    } => {
                        self.writer.write_unsigned_byte(*tag)?;
                        self.writer.write_unsigned_short(*name_index)?;
                        self.writer.write_unsigned_short(*descriptor_index)?;
                    }

                    CpInfo::ConstantUtf8Info { tag, length, bytes } => {
                        self.writer.write_unsigned_byte(*tag)?;
                        self.writer.write_unsigned_short(*length)?;

                        for b in bytes {
                            self.writer.write_unsigned_byte(*b)?;
                        }
                    }

                    CpInfo::ConstantMethodHandleInfo {
                        tag,
                        reference_kind,
                        reference_index,
                    } => {
                        self.writer.write_unsigned_byte(*tag)?;
                        self.writer.write_unsigned_byte(*reference_kind)?;
                        self.writer.write_unsigned_short(*reference_index)?;
                    }

                    CpInfo::ConstantMethodTypeInfo {
                        tag,
                        descriptor_index,
                    } => {
                        self.writer.write_unsigned_byte(*tag)?;
                        self.writer.write_unsigned_short(*descriptor_index)?;
                    }

                    CpInfo::ConstantDynamicInfo {
                        tag,
                        bootstrap_method_attr_index,
                        name_and_type_index,
                    } => {
                        self.writer.write_unsigned_byte(*tag)?;
                        self.writer
                            .write_unsigned_short(*bootstrap_method_attr_index)?;
                        self.writer.write_unsigned_short(*name_and_type_index)?;
                    }

                    CpInfo::ConstantInvokeDynamicInfo {
                        tag,
                        bootstrap_method_attr_index,
                        name_and_type_index,
                    } => {
                        self.writer.write_unsigned_byte(*tag)?;
                        self.writer
                            .write_unsigned_short(*bootstrap_method_attr_index)?;
                        self.writer.write_unsigned_short(*name_and_type_index)?;
                    }

                    CpInfo::ConstantModuleInfo { tag, name_index } => {
                        self.writer.write_unsigned_byte(*tag)?;
                        self.writer.write_unsigned_short(*name_index)?;
                    }

                    CpInfo::ConstantPackageInfo { tag, name_index } => {
                        self.writer.write_unsigned_byte(*tag)?;
                        self.writer.write_unsigned_short(*name_index)?;
                    }
                }
            }
        }

        Ok(())
    }

    /// Serialize the ClassFile object into a stream of raw JVM bytecode bytes.
    pub fn serialize(&mut self, classfile: &ClassFile) -> SerializeResult<()> {
        // Headers
        self.writer.write_unsigned_int(classfile.magic)?;
        self.writer.write_unsigned_short(classfile.minor_version)?;
        self.writer.write_unsigned_short(classfile.major_version)?;

        // Constant Pool
        assert!(classfile.constant_pool_count > 0);
        self.writer
            .write_unsigned_short(classfile.constant_pool_count)?;
        self.serialize_constant_pool(&classfile.constant_pool)?;

        self.writer.write_unsigned_short(classfile.access_flags)?;
        self.writer.write_unsigned_short(classfile.this_class)?;

        let mut super_class = classfile.super_class;
        // if super_class == 0 then this is ``java.lang.Object``
        if super_class > 0 {
            super_class += 1;
        }
        self.writer.write_unsigned_short(super_class)?;

        self.writer
            .write_unsigned_short(classfile.interfaces_count)?;
        for idx in 0..classfile.interfaces_count as usize {
            self.writer
                .write_unsigned_short(classfile.interfaces[idx])?;
        }

        // Fields
        self.writer.write_unsigned_short(classfile.fields_count)?;
        self.serialize_fields(&classfile.fields)?;

        // methods
        self.writer.write_unsigned_short(classfile.methods_count)?;
        self.serialize_methods(&classfile.methods)?;

        // class attributes
        self.writer
            .write_unsigned_short(classfile.attributes_count)?;
        self.serialize_attributes(&classfile.attributes)?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    // The byte buffer `buf` corresponds to this disassembled class file:
    //Classfile /Users/z0ltan/dev/playground/Minimal.class
    //  Last modified 27-Jan-2023; size 217 bytes
    //  SHA-256 checksum b590391a0a1f08067e66f237803225d0246d178484e0608543ea4fd12180dc2a
    //  Compiled from "Minimal.java"
    //public class Minimal
    //  minor version: 3
    //  major version: 45
    //  flags: (0x0021) ACC_PUBLIC, ACC_SUPER
    //  this_class: #12                         // Minimal
    //  super_class: #13                        // java/lang/Object
    //  interfaces: 0, fields: 0, methods: 2, attributes: 1
    //Constant pool:
    //   #1 = Methodref          #13.#7         // java/lang/Object."<init>":()V
    //   #2 = Utf8               java/lang/Object
    //   #3 = Utf8               SourceFile
    //   #4 = Utf8               <init>
    //   #5 = Utf8               main
    //   #6 = Utf8               Minimal
    //   #7 = NameAndType        #4:#11         // "<init>":()V
    //   #8 = Utf8               Code
    //   #9 = Utf8               Minimal.java
    //  #10 = Utf8               ([Ljava/lang/String;)V
    //  #11 = Utf8               ()V
    //  #12 = Class              #6             // Minimal
    //  #13 = Class              #2             // java/lang/Object
    //{
    //  public Minimal();
    //    descriptor: ()V
    //    flags: (0x0001) ACC_PUBLIC
    //    Code:
    //      stack=1, locals=1, args_size=1
    //         0: aload_0
    //         1: invokespecial #1                  // Method java/lang/Object."<init>":()V
    //         4: return
    //
    //  public static void main(java.lang.String[]);
    //    descriptor: ([Ljava/lang/String;)V
    //    flags: (0x0009) ACC_PUBLIC, ACC_STATIC
    //    Code:
    //      stack=1, locals=1, args_size=1
    //         0: return
    //}
    //SourceFile: "Minimal.java"
    fn test_serialize_minimal() {
        use crate::model::{attributes::AttributeInfo::*, constant_pool::types::CpInfo::*};

        let expected_bytes = [
            0xca, 0xfe, 0xba, 0xbe, 0x00, 0x03, 0x00, 0x2d, 0x00, 0x0e, 0x0a, 0x00, 0x0d, 0x00,
            0x07, 0x01, 0x00, 0x10, 0x6a, 0x61, 0x76, 0x61, 0x2f, 0x6c, 0x61, 0x6e, 0x67, 0x2f,
            0x4f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x01, 0x00, 0x0a, 0x53, 0x6f, 0x75, 0x72, 0x63,
            0x65, 0x46, 0x69, 0x6c, 0x65, 0x01, 0x00, 0x06, 0x3c, 0x69, 0x6e, 0x69, 0x74, 0x3e,
            0x01, 0x00, 0x04, 0x6d, 0x61, 0x69, 0x6e, 0x01, 0x00, 0x07, 0x4d, 0x69, 0x6e, 0x69,
            0x6d, 0x61, 0x6c, 0x0c, 0x00, 0x04, 0x00, 0x0b, 0x01, 0x00, 0x04, 0x43, 0x6f, 0x64,
            0x65, 0x01, 0x00, 0x0c, 0x4d, 0x69, 0x6e, 0x69, 0x6d, 0x61, 0x6c, 0x2e, 0x6a, 0x61,
            0x76, 0x61, 0x01, 0x00, 0x16, 0x28, 0x5b, 0x4c, 0x6a, 0x61, 0x76, 0x61, 0x2f, 0x6c,
            0x61, 0x6e, 0x67, 0x2f, 0x53, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x3b, 0x29, 0x56, 0x01,
            0x00, 0x03, 0x28, 0x29, 0x56, 0x07, 0x00, 0x06, 0x07, 0x00, 0x02, 0x00, 0x21, 0x00,
            0x0c, 0x00, 0x0d, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x01, 0x00, 0x04, 0x00,
            0x0b, 0x00, 0x01, 0x00, 0x08, 0x00, 0x00, 0x00, 0x11, 0x00, 0x01, 0x00, 0x01, 0x00,
            0x00, 0x00, 0x05, 0x2a, 0xb7, 0x00, 0x01, 0xb1, 0x00, 0x00, 0x00, 0x00, 0x00, 0x09,
            0x00, 0x05, 0x00, 0x0a, 0x00, 0x01, 0x00, 0x08, 0x00, 0x00, 0x00, 0x0d, 0x00, 0x01,
            0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0xb1, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00,
            0x03, 0x00, 0x00, 0x00, 0x02, 0x00, 0x09,
        ];

        let classfile = ClassFile {
            magic: 3405691582,
            minor_version: 3,
            major_version: 45,
            constant_pool_count: 14,
            constant_pool: vec![
                None,
                Some(ConstantMethodrefInfo {
                    tag: 10,
                    class_index: 13,
                    name_and_type_index: 7,
                }),
                Some(ConstantUtf8Info {
                    tag: 1,
                    length: 16,
                    bytes: vec![
                        106, 97, 118, 97, 47, 108, 97, 110, 103, 47, 79, 98, 106, 101, 99, 116,
                    ],
                }),
                Some(ConstantUtf8Info {
                    tag: 1,
                    length: 10,
                    bytes: vec![83, 111, 117, 114, 99, 101, 70, 105, 108, 101],
                }),
                Some(ConstantUtf8Info {
                    tag: 1,
                    length: 6,
                    bytes: vec![60, 105, 110, 105, 116, 62],
                }),
                Some(ConstantUtf8Info {
                    tag: 1,
                    length: 4,
                    bytes: vec![109, 97, 105, 110],
                }),
                Some(ConstantUtf8Info {
                    tag: 1,
                    length: 7,
                    bytes: vec![77, 105, 110, 105, 109, 97, 108],
                }),
                Some(ConstantNameAndTypeInfo {
                    tag: 12,
                    name_index: 4,
                    descriptor_index: 11,
                }),
                Some(ConstantUtf8Info {
                    tag: 1,
                    length: 4,
                    bytes: vec![67, 111, 100, 101],
                }),
                Some(ConstantUtf8Info {
                    tag: 1,
                    length: 12,
                    bytes: vec![77, 105, 110, 105, 109, 97, 108, 46, 106, 97, 118, 97],
                }),
                Some(ConstantUtf8Info {
                    tag: 1,
                    length: 22,
                    bytes: vec![
                        40, 91, 76, 106, 97, 118, 97, 47, 108, 97, 110, 103, 47, 83, 116, 114, 105,
                        110, 103, 59, 41, 86,
                    ],
                }),
                Some(ConstantUtf8Info {
                    tag: 1,
                    length: 3,
                    bytes: vec![40, 41, 86],
                }),
                Some(ConstantClassInfo {
                    tag: 7,
                    name_index: 6,
                }),
                Some(ConstantClassInfo {
                    tag: 7,
                    name_index: 2,
                }),
            ],
            access_flags: 33,
            this_class: 12,
            super_class: 12,
            interfaces_count: 0,
            interfaces: vec![],
            fields_count: 0,
            fields: vec![],
            methods_count: 2,
            methods: vec![
                MethodInfo {
                    access_flags: 1,
                    name_index: 4,
                    descriptor_index: 11,
                    attributes_count: 1,
                    attributes: vec![Code {
                        attribute_name_index: 8,
                        attribute_length: 17,
                        max_stack: 1,
                        max_locals: 1,
                        code_length: 5,
                        code: vec![42, 183, 0, 1, 177],
                        exception_table_length: 0,
                        exception_table: vec![],
                        code_attributes_count: 0,
                        code_attributes: vec![],
                    }],
                },
                MethodInfo {
                    access_flags: 9,
                    name_index: 5,
                    descriptor_index: 10,
                    attributes_count: 1,
                    attributes: vec![Code {
                        attribute_name_index: 8,
                        attribute_length: 13,
                        max_stack: 1,
                        max_locals: 1,
                        code_length: 1,
                        code: vec![177],
                        exception_table_length: 0,
                        exception_table: vec![],
                        code_attributes_count: 0,
                        code_attributes: vec![],
                    }],
                },
            ],
            attributes_count: 1,
            attributes: vec![SourceFile {
                attribute_name_index: 3,
                attribute_length: 2,
                sourcefile_index: 9,
            }],
        };

        let mut bytes: Vec<u8> = Vec::new();
        let mut serializer = Serializer::new(Writer::new(&mut bytes));
        serializer.serialize(&classfile).unwrap();
        assert_eq!(expected_bytes, &bytes[..]);
    }
}
