//! Module to construct a Java (JVM) raw class file bytes from the object model.

use crate::{
    error::SerializeError,
    model::{
        constant_pool::{tags::*, types::CpInfo},
        ClassFile,
    },
    rw::writer::Writer,
};
use std::io::Write;

pub type SerializeResult<T> = Result<T, SerializeError>;

/// The Serializer reads a class file object model, converts it to a stream of raw bytes compliant
/// with the `class` file model, and
/// writes it to the supplier writer.
struct Serializer<'a, W: Write> {
    writer: Writer<'a, W>,
}

impl<'a, W: Write> Serializer<'a, W> {
    pub fn new(writer: Writer<'a, W>) -> Self {
        Serializer { writer }
    }

    //pub fn serialize(&mut self, classfile: ClassFile) -> SerializeResult<()> {
    //    // Headers
    //    self.writer.write_unsigned_int(classfile.magic)?;
    //    self.writer.write_unsigned_short(classfile.minor_version)?;
    //    self.writer.write_unsigned_short(classfile.major_version)?;

    //    // Constant Pool
    //    self.writer
    //        .write_unsigned_short(classfile.constant_pool_count)?;

    //    for constant_pool_entry in classfile.constant_pool.iter() {

    //    }

    //    let mut constant_pool: Vec<Box<dyn CpInfo>> = Vec::new();

    //    for idx in 0..constant_pool_count - 1 {
    //        let tag = self.reader.read_unsigned_byte()?;
    //        println!("tag = {tag}");

    //        match tag {
    //            CONSTANT_METHODREF => {
    //                let class_index = self.reader.read_unsigned_short()?;
    //                let name_and_type_index = self.reader.read_unsigned_short()?;
    //                constant_pool.push(Box::new(ConstantMethodrefInfo {
    //                    tag,
    //                    class_index,
    //                    name_and_type_index,
    //                }));
    //            }

    //            CONSTANT_CLASS => {
    //                let name_index = self.reader.read_unsigned_short()?;
    //                constant_pool.push(Box::new(ConstantClassInfo { tag, name_index }));
    //            }

    //            CONSTANT_FIELDREF => {
    //                let class_index = self.reader.read_unsigned_short()?;
    //                let name_and_type_index = self.reader.read_unsigned_short()?;
    //                constant_pool.push(Box::new(ConstantFieldrefInfo {
    //                    tag,
    //                    class_index,
    //                    name_and_type_index,
    //                }));
    //            }

    //            CONSTANT_INTERFACEMETHODREF => {
    //                let class_index = self.reader.read_unsigned_short()?;
    //                let name_and_type_index = self.reader.read_unsigned_short()?;
    //                constant_pool.push(Box::new(ConstantInterfaceMethodrefInfo {
    //                    tag,
    //                    class_index,
    //                    name_and_type_index,
    //                }));
    //            }

    //            CONSTANT_STRING => {
    //                let string_index = self.reader.read_unsigned_short()?;
    //                constant_pool.push(Box::new(ConstantStringInfo { tag, string_index }));
    //            }

    //            CONSTANT_INTEGER => {
    //                let bytes = self.reader.read_unsigned_int()?;
    //                constant_pool.push(Box::new(ConstantIntegerInfo { tag, bytes }));
    //            }

    //            CONSTANT_FLOAT => {
    //                let bytes = self.reader.read_unsigned_int()?;
    //                constant_pool.push(Box::new(ConstantFloatInfo { tag, bytes }));
    //            }

    //            CONSTANT_LONG => {
    //                let high_bytes = self.reader.read_unsigned_int()?;
    //                let low_bytes = self.reader.read_unsigned_int()?;
    //                constant_pool.push(Box::new(ConstantLongInfo {
    //                    tag,
    //                    high_bytes,
    //                    low_bytes,
    //                }));
    //            }

    //            CONSTANT_DOUBLE => {
    //                let high_bytes = self.reader.read_unsigned_int()?;
    //                let low_bytes = self.reader.read_unsigned_int()?;
    //                constant_pool.push(Box::new(ConstantDoubleInfo {
    //                    tag,
    //                    high_bytes,
    //                    low_bytes,
    //                }));
    //            }

    //            CONSTANT_NAMEANDTYPE => {
    //                let name_index = self.reader.read_unsigned_short()?;
    //                let descriptor_index = self.reader.read_unsigned_short()?;
    //                constant_pool.push(Box::new(ConstantNameAndTypeInfo {
    //                    tag,
    //                    name_index,
    //                    descriptor_index,
    //                }));
    //            }

    //            CONSTANT_UTF8 => {
    //                let length = self.reader.read_unsigned_short()?;
    //                let mut bytes = Vec::new();
    //                for _ in 0..length {
    //                    bytes.push(self.reader.read_unsigned_byte()?);
    //                }

    //                constant_pool.push(Box::new(ConstantUtf8Info { tag, length, bytes }));
    //            }

    //            _ => unimplemented!(),
    //        }
    //    }

    //    Ok(ClassFile {
    //        magic,
    //        minor_version,
    //        major_version,
    //        constant_pool_count,
    //        constant_pool,
    //    })
    //}
}

#[cfg(test)]
mod test {
    use super::*;

    //#[test]
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
    //fn test_serialize_minimal() {
    //    let expected_bytes = [
    //        0xca, 0xfe, 0xba, 0xbe, 0x00, 0x00, 0x00, 0x41, 0x00, 0x0f, 0x0a, 0x00, 0x02, 0x00,
    //        0x03, 0x07, 0x00, 0x04, 0x0c, 0x00, 0x05, 0x00, 0x06, 0x01, 0x00, 0x10, 0x6a, 0x61,
    //        0x76, 0x61, 0x2f, 0x6c, 0x61, 0x6e, 0x67, 0x2f, 0x4f, 0x62, 0x6a, 0x65, 0x63, 0x74,
    //        0x01, 0x00, 0x06, 0x3c, 0x69, 0x6e, 0x69, 0x74, 0x3e, 0x01, 0x00, 0x03, 0x28, 0x29,
    //        0x56, 0x07, 0x00, 0x08, 0x01, 0x00, 0x07, 0x4d, 0x69, 0x6e, 0x69, 0x6d, 0x61, 0x6c,
    //        0x01, 0x00, 0x04, 0x43, 0x6f, 0x64, 0x65, 0x01, 0x00, 0x0f, 0x4c, 0x69, 0x6e, 0x65,
    //        0x4e, 0x75, 0x6d, 0x62, 0x65, 0x72, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x01, 0x00, 0x04,
    //        0x6d, 0x61, 0x69, 0x6e, 0x01, 0x00, 0x16, 0x28, 0x5b, 0x4c, 0x6a, 0x61, 0x76, 0x61,
    //        0x2f, 0x6c, 0x61, 0x6e, 0x67, 0x2f, 0x53, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x3b, 0x29,
    //        0x56, 0x01, 0x00, 0x0a, 0x53, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x46, 0x69, 0x6c, 0x65,
    //        0x01, 0x00, 0x0c, 0x4d, 0x69, 0x6e, 0x69, 0x6d, 0x61, 0x6c, 0x2e, 0x6a, 0x61, 0x76,
    //        0x61, 0x00, 0x21, 0x00, 0x07, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00,
    //        0x01, 0x00, 0x05, 0x00, 0x06, 0x00, 0x01, 0x00, 0x09, 0x00, 0x00, 0x00, 0x1d, 0x00,
    //        0x01, 0x00, 0x01, 0x00, 0x00, 0x00, 0x05, 0x2a, 0xb7, 0x00, 0x01, 0xb1, 0x00, 0x00,
    //        0x00, 0x01, 0x00, 0x0a, 0x00, 0x00, 0x00, 0x06, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01,
    //        0x00, 0x09, 0x00, 0x0b, 0x00, 0x0c, 0x00, 0x01, 0x00, 0x09, 0x00, 0x00, 0x00, 0x19,
    //        0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0xb1, 0x00, 0x00, 0x00, 0x01, 0x00,
    //        0x0a, 0x00, 0x00, 0x00, 0x06, 0x00, 0x01, 0x00, 0x00, 0x00, 0x02, 0x00, 0x01, 0x00,
    //        0x0d, 0x00, 0x00, 0x00, 0x02, 0x00, 0x0e,
    //    ];

    //    let mut bytes = Vec::new();
    //    let mut serializer = Serializer::new(Writer::new(&mut bytes));
    //    deserializer.deserialize().unwrap();

    //    assert_eq!(bytes, expected_bytes);
    //}
}
