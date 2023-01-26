//! Module to read a Java (JVM) class file and construct the object model from the raw bytes.

use crate::{
    error::DeserializeError,
    model::{
        constant_pool::{tags::*, types::CpInfo},
        AttributeInfo, ClassFile, FieldInfo, MethodInfo,
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
    ) -> DeserializeResult<Vec<AttributeInfo>> {
        let mut attributes = Vec::new();
        for _ in 0..attributes_count {}
        Ok(attributes)
    }

    /// Deserialize the fields of the class file.
    fn deserialize_fields(&mut self, fields_count: u16) -> DeserializeResult<Vec<FieldInfo>> {
        let mut fields = Vec::new();
        for _ in 0..fields_count {
            let access_flags = self.reader.read_unsigned_short()?;
            let name_index = self.reader.read_unsigned_short()?;
            let descriptor_index = self.reader.read_unsigned_short()?;
            let attributes_count = self.reader.read_unsigned_short()?;

            let attributes = self.deserialize_attributes(attributes_count)?;

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
    fn deserialize_methods(&mut self, methods_count: u16) -> DeserializeResult<Vec<MethodInfo>> {
        let mut methods = Vec::new();

        for _ in 0..methods_count {
            let access_flags = self.reader.read_unsigned_short()?;
            let name_index = self.reader.read_unsigned_short()?;
            let descriptor_index = self.reader.read_unsigned_short()?;
            let attributes_count = self.reader.read_unsigned_short()?;

            let attributes = self.deserialize_attributes(attributes_count)?;

            methods.push(MethodInfo {
                access_flags,
                name_index,
                descriptor_index,
                attributes_count,
                attributes,
            })
        }

        Ok(methods)
    }

    /// Deserialize the contents of the Constant Pool.
    fn deserialize_constant_pool(
        &mut self,
        constant_pool_count: u16,
    ) -> DeserializeResult<Vec<CpInfo>> {
        let mut constant_pool = Vec::new();

        for idx in 0..constant_pool_count - 1 {
            let tag = self.reader.read_unsigned_byte()?;

            match tag {
                CONSTANT_METHODREF => {
                    let class_index = self.reader.read_unsigned_short()?;
                    let name_and_type_index = self.reader.read_unsigned_short()?;
                    constant_pool.push(CpInfo::ConstantMethodrefInfo {
                        tag,
                        class_index,
                        name_and_type_index,
                    });
                }

                CONSTANT_CLASS => {
                    let name_index = self.reader.read_unsigned_short()?;
                    constant_pool.push(CpInfo::ConstantClassInfo { tag, name_index });
                }

                CONSTANT_FIELDREF => {
                    let class_index = self.reader.read_unsigned_short()?;
                    let name_and_type_index = self.reader.read_unsigned_short()?;
                    constant_pool.push(CpInfo::ConstantFieldrefInfo {
                        tag,
                        class_index,
                        name_and_type_index,
                    });
                }

                CONSTANT_INTERFACEMETHODREF => {
                    let class_index = self.reader.read_unsigned_short()?;
                    let name_and_type_index = self.reader.read_unsigned_short()?;
                    constant_pool.push(CpInfo::ConstantInterfaceMethodrefInfo {
                        tag,
                        class_index,
                        name_and_type_index,
                    });
                }

                CONSTANT_STRING => {
                    let string_index = self.reader.read_unsigned_short()?;
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
                    let name_index = self.reader.read_unsigned_short()?;
                    let descriptor_index = self.reader.read_unsigned_short()?;
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

                _ => unimplemented!(),
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
        let mut constant_pool = self.deserialize_constant_pool(constant_pool_count)?;

        let access_flags = self.reader.read_unsigned_short()?;
        let this_class = self.reader.read_unsigned_short()?;
        let super_class = self.reader.read_unsigned_short()?;

        let interfaces_count = self.reader.read_unsigned_short()?;
        let mut interfaces = Vec::with_capacity(interfaces_count as usize);
        for _ in 0..interfaces_count {
            interfaces.push(self.reader.read_unsigned_short()?);
        }

        // Fields
        let fields_count = self.reader.read_unsigned_short()?;
        let fields = self.deserialize_fields(fields_count)?;

        // methods
        let methods_count = self.reader.read_unsigned_short()?;
        let methods = self.deserialize_methods(methods_count)?;

        // class attributes
        let attributes_count = self.reader.read_unsigned_short()?;
        let attributes = self.deserialize_attributes(attributes_count)?;

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
    //
    // ```bash`
    //   Classfile /Users/z0ltan/dev/oyi-lang/oyi-manifesto/projects/phoron/phoron_rs/phoron_core/Minimal.class
    //  Last modified 24-Jan-2023; size 259 bytes
    //  SHA-256 checksum a50a8c17f31dbb5ea4e7d6b919cfa21d7e58a33e235cf516d86533b003f32f82
    //  Compiled from "Minimal.java"
    //public class Minimal
    //  minor version: 0
    //  major version: 65
    //  flags: (0x0021) ACC_PUBLIC, ACC_SUPER
    //  this_class: #7                          // Minimal
    //  super_class: #2                         // java/lang/Object
    //  interfaces: 0, fields: 0, methods: 2, attributes: 1
    //Constant pool:
    //   #1 = Methodref          #2.#3          // java/lang/Object."<init>":()V
    //   #2 = Class              #4             // java/lang/Object
    //   #3 = NameAndType        #5:#6          // "<init>":()V
    //   #4 = Utf8               java/lang/Object
    //   #5 = Utf8               <init>
    //   #6 = Utf8               ()V
    //   #7 = Class              #8             // Minimal
    //   #8 = Utf8               Minimal
    //   #9 = Utf8               Code
    //  #10 = Utf8               LineNumberTable
    //  #11 = Utf8               main
    //  #12 = Utf8               ([Ljava/lang/String;)V
    //  #13 = Utf8               SourceFile
    //  #14 = Utf8               Minimal.java
    //{
    //  public Minimal();
    //    descriptor: ()V
    //    flags: (0x0001) ACC_PUBLIC
    //    Code:
    //      stack=1, locals=1, args_size=1
    //         0: aload_0
    //         1: invokespecial #1                  // Method java/lang/Object."<init>":()V
    //         4: return
    //      LineNumberTable:
    //        line 1: 0
    //
    //  public static void main(java.lang.String[]);
    //    descriptor: ([Ljava/lang/String;)V
    //    flags: (0x0009) ACC_PUBLIC, ACC_STATIC
    //    Code:
    //      stack=0, locals=1, args_size=1
    //         0: return
    //      LineNumberTable:
    //        line 2: 0
    //}
    //SourceFile: "Minimal.java"
    //````
    fn test_deserialize_minimal() {
        use std::io::Cursor;

        let bytes = [
            0xca, 0xfe, 0xba, 0xbe, 0x00, 0x00, 0x00, 0x41, 0x00, 0x0f, 0x0a, 0x00, 0x02, 0x00,
            0x03, 0x07, 0x00, 0x04, 0x0c, 0x00, 0x05, 0x00, 0x06, 0x01, 0x00, 0x10, 0x6a, 0x61,
            0x76, 0x61, 0x2f, 0x6c, 0x61, 0x6e, 0x67, 0x2f, 0x4f, 0x62, 0x6a, 0x65, 0x63, 0x74,
            0x01, 0x00, 0x06, 0x3c, 0x69, 0x6e, 0x69, 0x74, 0x3e, 0x01, 0x00, 0x03, 0x28, 0x29,
            0x56, 0x07, 0x00, 0x08, 0x01, 0x00, 0x07, 0x4d, 0x69, 0x6e, 0x69, 0x6d, 0x61, 0x6c,
            0x01, 0x00, 0x04, 0x43, 0x6f, 0x64, 0x65, 0x01, 0x00, 0x0f, 0x4c, 0x69, 0x6e, 0x65,
            0x4e, 0x75, 0x6d, 0x62, 0x65, 0x72, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x01, 0x00, 0x04,
            0x6d, 0x61, 0x69, 0x6e, 0x01, 0x00, 0x16, 0x28, 0x5b, 0x4c, 0x6a, 0x61, 0x76, 0x61,
            0x2f, 0x6c, 0x61, 0x6e, 0x67, 0x2f, 0x53, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x3b, 0x29,
            0x56, 0x01, 0x00, 0x0a, 0x53, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x46, 0x69, 0x6c, 0x65,
            0x01, 0x00, 0x0c, 0x4d, 0x69, 0x6e, 0x69, 0x6d, 0x61, 0x6c, 0x2e, 0x6a, 0x61, 0x76,
            0x61, 0x00, 0x21, 0x00, 0x07, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00,
            0x01, 0x00, 0x05, 0x00, 0x06, 0x00, 0x01, 0x00, 0x09, 0x00, 0x00, 0x00, 0x1d, 0x00,
            0x01, 0x00, 0x01, 0x00, 0x00, 0x00, 0x05, 0x2a, 0xb7, 0x00, 0x01, 0xb1, 0x00, 0x00,
            0x00, 0x01, 0x00, 0x0a, 0x00, 0x00, 0x00, 0x06, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01,
            0x00, 0x09, 0x00, 0x0b, 0x00, 0x0c, 0x00, 0x01, 0x00, 0x09, 0x00, 0x00, 0x00, 0x19,
            0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0xb1, 0x00, 0x00, 0x00, 0x01, 0x00,
            0x0a, 0x00, 0x00, 0x00, 0x06, 0x00, 0x01, 0x00, 0x00, 0x00, 0x02, 0x00, 0x01, 0x00,
            0x0d, 0x00, 0x00, 0x00, 0x02, 0x00, 0x0e,
        ];

        let mut deserializer = Deserializer::new(Reader::new(Cursor::new(bytes)));
        let classfile = deserializer.deserialize();
        println!("{:#?}", classfile);
    }
}
