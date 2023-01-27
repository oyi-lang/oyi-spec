//! Module to read a Java (JVM) class file and construct the object model from the raw bytes.

use crate::{
    error::DeserializeError,
    model::{
        constant_pool::{tags::*, types::CpInfo},
        predefined_attributes, AttributeInfo, ClassFile, ExceptionHandler, FieldInfo, LineNumber,
        LocalVariable, MethodInfo,
    },
    rw::reader::Reader,
};
use std::io::Read;

pub type DeserializeResult<T> = Result<T, DeserializeError>;

/// The Deserializer reads a class file byte stream and converts it into the
/// object model repreensting the class file.
struct Deserializer<R: Read> {
    reader: Reader<R>,
}

impl<R: Read> Deserializer<R> {
    pub fn new(reader: Reader<R>) -> Self {
        Deserializer { reader }
    }

    /// Deserialize the attributes of the class file.
    fn deserialize_attributes(
        &mut self,
        attributes_count: u16,
        constant_pool: &[CpInfo],
    ) -> DeserializeResult<Vec<AttributeInfo>> {
        let mut attributes = Vec::new();

        for _ in 0..attributes_count {
            let attribute_name_index = self.reader.read_unsigned_short()? - 1;
            let attribute_length = self.reader.read_unsigned_int()?;

            match &constant_pool[attribute_name_index as usize] {
                CpInfo::ConstantUtf8Info { tag, length, bytes } => {
                    match String::from_utf8_lossy(bytes).into_owned().as_str() {
                        predefined_attributes::SOURCE_FILE => {
                            let sourcefile_index = self.reader.read_unsigned_short()?;
                            attributes.push(AttributeInfo::SourceFile {
                                attribute_name_index,
                                attribute_length,
                                sourcefile_index,
                            });
                        }

                        predefined_attributes::CONSTANT_VALUE => {
                            let constantvalue_index = self.reader.read_unsigned_short()? - 1;
                            attributes.push(AttributeInfo::ConstantValue {
                                attribute_name_index,
                                attribute_length,
                                constantvalue_index,
                            });
                        }

                        predefined_attributes::CODE => {
                            let max_stack = self.reader.read_unsigned_short()?;
                            let max_locals = self.reader.read_unsigned_short()?;

                            let code_length = self.reader.read_unsigned_int()?;
                            assert!(code_length > 0);
                            let mut code = Vec::new();
                            for _ in 0..code_length {
                                code.push(self.reader.read_unsigned_byte()?);
                            }

                            let exception_table_length = self.reader.read_unsigned_short()?;
                            let mut exception_table = Vec::new();
                            for _ in 0..exception_table_length {
                                let start_pc = self.reader.read_unsigned_short()?;
                                let end_pc = self.reader.read_unsigned_short()?;
                                let handler_pc = self.reader.read_unsigned_short()?;
                                let mut catch_type = self.reader.read_unsigned_short()?;

                                if catch_type != 0 {
                                    catch_type -= 1;
                                }

                                exception_table.push(ExceptionHandler {
                                    start_pc,
                                    end_pc,
                                    handler_pc,
                                    catch_type,
                                });
                            }

                            let code_attributes_count = self.reader.read_unsigned_short()?;
                            let code_attributes =
                                self.deserialize_attributes(code_attributes_count, constant_pool)?;
                            attributes.push(AttributeInfo::Code {
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
                            });
                        }

                        predefined_attributes::EXCEPTIONS => {
                            let number_of_exceptions = self.reader.read_unsigned_short()?;
                            let mut exception_index_table =
                                Vec::with_capacity(number_of_exceptions as usize);

                            for _ in 0..number_of_exceptions {
                                let mut idx = self.reader.read_unsigned_short()?;
                                if idx != 0 {
                                    idx -= 1;
                                }
                                exception_index_table.push(idx);
                            }
                            attributes.push(AttributeInfo::Exceptions {
                                attribute_name_index,
                                attribute_length,
                                number_of_exceptions,
                                exception_index_table,
                            });
                        }

                        predefined_attributes::LINE_NUMBER_TABLE => {
                            let line_number_table_length = self.reader.read_unsigned_short()?;
                            let mut line_number_table =
                                Vec::with_capacity(line_number_table_length as usize);

                            for _ in 0..line_number_table_length {
                                let start_pc = self.reader.read_unsigned_short()?;
                                let line_number = self.reader.read_unsigned_short()?;
                                line_number_table.push(LineNumber {
                                    start_pc,
                                    line_number,
                                });
                            }

                            attributes.push(AttributeInfo::LineNumberTable {
                                attribute_name_index,
                                attribute_length,
                                line_number_table_length,
                                line_number_table,
                            });
                        }

                        predefined_attributes::LOCAL_VARIABLE_TABLE => {
                            let local_variable_table_length = self.reader.read_unsigned_short()?;
                            let mut local_variable_table =
                                Vec::with_capacity(local_variable_table_length as usize);

                            for _ in 0..local_variable_table_length {
                                let start_pc = self.reader.read_unsigned_short()?;
                                let length = self.reader.read_unsigned_short()?;
                                let name_index = self.reader.read_unsigned_short()? - 1;
                                let descriptor_index = self.reader.read_unsigned_short()? - 1;
                                let index = self.reader.read_unsigned_short()?;

                                local_variable_table.push(LocalVariable {
                                    start_pc,
                                    length,
                                    name_index,
                                    descriptor_index,
                                    index,
                                });
                            }

                            attributes.push(AttributeInfo::LocalVariableTable {
                                attribute_name_index,
                                attribute_length,
                                local_variable_table_length,
                                local_variable_table,
                            });
                        }
                        _ => {
                            // simply read the bytes and discard for any unknown attributes
                            for _ in 0..attribute_length {
                                let _ = self.reader.read_unsigned_byte()?;
                            }
                        }
                    }
                }
                _ => unreachable!(),
            }
        }

        Ok(attributes)
    }

    /// Deserialize the fields of the class file.
    fn deserialize_fields(
        &mut self,
        fields_count: u16,
        constant_pool: &[CpInfo],
    ) -> DeserializeResult<Vec<FieldInfo>> {
        let mut fields = Vec::new();
        for _ in 0..fields_count {
            let access_flags = self.reader.read_unsigned_short()?;
            let name_index = self.reader.read_unsigned_short()? - 1;
            let descriptor_index = self.reader.read_unsigned_short()? - 1;
            let attributes_count = self.reader.read_unsigned_short()?;

            let attributes = self.deserialize_attributes(attributes_count, constant_pool)?;
            fields.push(FieldInfo {
                access_flags,
                name_index,
                descriptor_index,
                attributes_count,
                attributes,
            });
        }

        Ok(fields)
    }

    /// Deserialize the methods of the class file.
    fn deserialize_methods(
        &mut self,
        methods_count: u16,
        constant_pool: &[CpInfo],
    ) -> DeserializeResult<Vec<MethodInfo>> {
        let mut methods = Vec::new();

        for _ in 0..methods_count {
            let access_flags = self.reader.read_unsigned_short()?;
            let name_index = self.reader.read_unsigned_short()? - 1;
            let descriptor_index = self.reader.read_unsigned_short()? - 1;
            let attributes_count = self.reader.read_unsigned_short()?;

            let attributes = self.deserialize_attributes(attributes_count, constant_pool)?;
            methods.push(MethodInfo {
                access_flags,
                name_index,
                descriptor_index,
                attributes_count,
                attributes,
            });
        }

        Ok(methods)
    }

    /// Deserialize the contents of the Constant Pool.
    fn deserialize_constant_pool(
        &mut self,
        constant_pool_count: u16,
    ) -> DeserializeResult<Vec<CpInfo>> {
        let mut constant_pool = Vec::new();

        for _ in 0..constant_pool_count - 1 {
            let tag = self.reader.read_unsigned_byte()?;

            match tag {
                CONSTANT_METHODREF => {
                    let class_index = self.reader.read_unsigned_short()? - 1;
                    let name_and_type_index = self.reader.read_unsigned_short()? - 1;
                    constant_pool.push(CpInfo::ConstantMethodrefInfo {
                        tag,
                        class_index,
                        name_and_type_index,
                    });
                }

                CONSTANT_CLASS => {
                    let name_index = self.reader.read_unsigned_short()? - 1;
                    constant_pool.push(CpInfo::ConstantClassInfo { tag, name_index });
                }

                CONSTANT_FIELDREF => {
                    let class_index = self.reader.read_unsigned_short()? - 1;
                    let name_and_type_index = self.reader.read_unsigned_short()? - 1;
                    constant_pool.push(CpInfo::ConstantFieldrefInfo {
                        tag,
                        class_index,
                        name_and_type_index,
                    });
                }

                CONSTANT_INTERFACEMETHODREF => {
                    let class_index = self.reader.read_unsigned_short()? - 1;
                    let name_and_type_index = self.reader.read_unsigned_short()? - 1;
                    constant_pool.push(CpInfo::ConstantInterfaceMethodrefInfo {
                        tag,
                        class_index,
                        name_and_type_index,
                    });
                }

                CONSTANT_STRING => {
                    let string_index = self.reader.read_unsigned_short()? - 1;
                    constant_pool.push(CpInfo::ConstantStringInfo { tag, string_index });
                }

                CONSTANT_INTEGER => {
                    let bytes = self.reader.read_unsigned_int()?;
                    constant_pool.push(CpInfo::ConstantIntegerInfo { tag, bytes });
                }

                CONSTANT_FLOAT => {
                    let bytes = self.reader.read_unsigned_int()?;
                    constant_pool.push(CpInfo::ConstantFloatInfo { tag, bytes });
                }

                CONSTANT_LONG => {
                    let high_bytes = self.reader.read_unsigned_int()?;
                    let low_bytes = self.reader.read_unsigned_int()?;
                    constant_pool.push(CpInfo::ConstantLongInfo {
                        tag,
                        high_bytes,
                        low_bytes,
                    });
                }

                CONSTANT_DOUBLE => {
                    let high_bytes = self.reader.read_unsigned_int()?;
                    let low_bytes = self.reader.read_unsigned_int()?;
                    constant_pool.push(CpInfo::ConstantDoubleInfo {
                        tag,
                        high_bytes,
                        low_bytes,
                    });
                }

                CONSTANT_NAMEANDTYPE => {
                    let name_index = self.reader.read_unsigned_short()? - 1;
                    let descriptor_index = self.reader.read_unsigned_short()? - 1;
                    constant_pool.push(CpInfo::ConstantNameAndTypeInfo {
                        tag,
                        name_index,
                        descriptor_index,
                    });
                }

                CONSTANT_UTF8 => {
                    let length = self.reader.read_unsigned_short()?;
                    let mut bytes = Vec::new();
                    for _ in 0..length {
                        bytes.push(self.reader.read_unsigned_byte()?);
                    }

                    constant_pool.push(CpInfo::ConstantUtf8Info { tag, length, bytes });
                }

                _ => unreachable!(),
            }
        }

        Ok(constant_pool)
    }

    pub fn deserialize(&mut self) -> DeserializeResult<ClassFile> {
        // Headers
        let magic = self.reader.read_unsigned_int()?;
        let minor_version = self.reader.read_unsigned_short()?;
        let major_version = self.reader.read_unsigned_short()?;

        // Constant Pool
        let constant_pool_count = self.reader.read_unsigned_short()?;
        assert!(constant_pool_count > 0);
        let mut constant_pool = self.deserialize_constant_pool(constant_pool_count)?;

        let access_flags = self.reader.read_unsigned_short()?;
        let this_class = self.reader.read_unsigned_short()? - 1;

        let mut super_class = self.reader.read_unsigned_short()?;
        // if super_class == 0 then this is ``java.lang.Object``
        if super_class > 0 {
            super_class -= 1;
        }

        let interfaces_count = self.reader.read_unsigned_short()?;
        let mut interfaces = Vec::with_capacity(interfaces_count as usize);
        for _ in 0..interfaces_count {
            interfaces.push(self.reader.read_unsigned_short()?);
        }

        // Fields
        let fields_count = self.reader.read_unsigned_short()?;
        let fields = self.deserialize_fields(fields_count, &constant_pool)?;

        // methods
        let methods_count = self.reader.read_unsigned_short()?;
        let methods = self.deserialize_methods(methods_count, &constant_pool)?;

        // class attributes
        let attributes_count = self.reader.read_unsigned_short()?;
        let attributes = self.deserialize_attributes(attributes_count, &constant_pool)?;

        Ok(ClassFile {
            magic,
            minor_version,
            major_version,
            constant_pool_count,
            constant_pool,
            access_flags,
            this_class,
            super_class,
            interfaces_count,
            interfaces,
            fields_count,
            fields,
            methods_count,
            methods,
            attributes_count,
            attributes,
        })
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
    fn test_deserialize_minimal() {
        use std::io::Cursor;

        let bytes = [
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

        let mut deserializer = Deserializer::new(Reader::new(Cursor::new(bytes)));
        let classfile = deserializer.deserialize().unwrap();
        println!("{:#?}", classfile);
    }
}
