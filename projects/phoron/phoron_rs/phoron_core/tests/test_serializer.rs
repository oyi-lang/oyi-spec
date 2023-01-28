use phoron_core::{
    model::{constant_pool::types::CpInfo::*, AttributeInfo::*, *},
    rw::writer::Writer,
    serializer::Serializer,
};

#[test]
// Bytecode for the following class file:
//Classfile /Users/z0ltan/dev/playground/HelloWorld.class
//  Last modified 27-Jan-2023; size 422 bytes
//  SHA-256 checksum 8b07d9dd65152998eda6951af14be9052f0dd66d8c60bbf1be42530fefe2e056
//  Compiled from "HelloWorld.java"
//public class HelloWorld
//  minor version: 0
//  major version: 65
//  flags: (0x0021) ACC_PUBLIC, ACC_SUPER
//  this_class: #21                         // HelloWorld
//  super_class: #2                         // java/lang/Object
//  interfaces: 0, fields: 0, methods: 2, attributes: 1
//Constant pool:
//   #1 = Methodref          #2.#3          // java/lang/Object."<init>":()V
//   #2 = Class              #4             // java/lang/Object
//   #3 = NameAndType        #5:#6          // "<init>":()V
//   #4 = Utf8               java/lang/Object
//   #5 = Utf8               <init>
//   #6 = Utf8               ()V
//   #7 = Fieldref           #8.#9          // java/lang/System.out:Ljava/io/PrintStream;
//   #8 = Class              #10            // java/lang/System
//   #9 = NameAndType        #11:#12        // out:Ljava/io/PrintStream;
//  #10 = Utf8               java/lang/System
//  #11 = Utf8               out
//  #12 = Utf8               Ljava/io/PrintStream;
//  #13 = String             #14            // Hello, world
//  #14 = Utf8               Hello, world
//  #15 = Methodref          #16.#17        // java/io/PrintStream.println:(Ljava/lang/String;)V
//  #16 = Class              #18            // java/io/PrintStream
//  #17 = NameAndType        #19:#20        // println:(Ljava/lang/String;)V
//  #18 = Utf8               java/io/PrintStream
//  #19 = Utf8               println
//  #20 = Utf8               (Ljava/lang/String;)V
//  #21 = Class              #22            // HelloWorld
//  #22 = Utf8               HelloWorld
//  #23 = Utf8               Code
//  #24 = Utf8               LineNumberTable
//  #25 = Utf8               main
//  #26 = Utf8               ([Ljava/lang/String;)V
//  #27 = Utf8               SourceFile
//  #28 = Utf8               HelloWorld.java
//{
//  public HelloWorld();
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
//      stack=2, locals=1, args_size=1
//         0: getstatic     #7                  // Field java/lang/System.out:Ljava/io/PrintStream;
//         3: ldc           #13                 // String Hello, world
//         5: invokevirtual #15                 // Method java/io/PrintStream.println:(Ljava/lang/String;)V
//         8: return
//      LineNumberTable:
//        line 2: 0
//}
//SourceFile: "HelloWorld.java"
fn test_serialize_hello_world() {
    let expected_bytes = [
        0xca, 0xfe, 0xba, 0xbe, 0x00, 0x00, 0x00, 0x41, 0x00, 0x1d, 0x0a, 0x00, 0x02, 0x00, 0x03,
        0x07, 0x00, 0x04, 0x0c, 0x00, 0x05, 0x00, 0x06, 0x01, 0x00, 0x10, 0x6a, 0x61, 0x76, 0x61,
        0x2f, 0x6c, 0x61, 0x6e, 0x67, 0x2f, 0x4f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x01, 0x00, 0x06,
        0x3c, 0x69, 0x6e, 0x69, 0x74, 0x3e, 0x01, 0x00, 0x03, 0x28, 0x29, 0x56, 0x09, 0x00, 0x08,
        0x00, 0x09, 0x07, 0x00, 0x0a, 0x0c, 0x00, 0x0b, 0x00, 0x0c, 0x01, 0x00, 0x10, 0x6a, 0x61,
        0x76, 0x61, 0x2f, 0x6c, 0x61, 0x6e, 0x67, 0x2f, 0x53, 0x79, 0x73, 0x74, 0x65, 0x6d, 0x01,
        0x00, 0x03, 0x6f, 0x75, 0x74, 0x01, 0x00, 0x15, 0x4c, 0x6a, 0x61, 0x76, 0x61, 0x2f, 0x69,
        0x6f, 0x2f, 0x50, 0x72, 0x69, 0x6e, 0x74, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x3b, 0x08,
        0x00, 0x0e, 0x01, 0x00, 0x0c, 0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x2c, 0x20, 0x77, 0x6f, 0x72,
        0x6c, 0x64, 0x0a, 0x00, 0x10, 0x00, 0x11, 0x07, 0x00, 0x12, 0x0c, 0x00, 0x13, 0x00, 0x14,
        0x01, 0x00, 0x13, 0x6a, 0x61, 0x76, 0x61, 0x2f, 0x69, 0x6f, 0x2f, 0x50, 0x72, 0x69, 0x6e,
        0x74, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x01, 0x00, 0x07, 0x70, 0x72, 0x69, 0x6e, 0x74,
        0x6c, 0x6e, 0x01, 0x00, 0x15, 0x28, 0x4c, 0x6a, 0x61, 0x76, 0x61, 0x2f, 0x6c, 0x61, 0x6e,
        0x67, 0x2f, 0x53, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x3b, 0x29, 0x56, 0x07, 0x00, 0x16, 0x01,
        0x00, 0x0a, 0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x57, 0x6f, 0x72, 0x6c, 0x64, 0x01, 0x00, 0x04,
        0x43, 0x6f, 0x64, 0x65, 0x01, 0x00, 0x0f, 0x4c, 0x69, 0x6e, 0x65, 0x4e, 0x75, 0x6d, 0x62,
        0x65, 0x72, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x01, 0x00, 0x04, 0x6d, 0x61, 0x69, 0x6e, 0x01,
        0x00, 0x16, 0x28, 0x5b, 0x4c, 0x6a, 0x61, 0x76, 0x61, 0x2f, 0x6c, 0x61, 0x6e, 0x67, 0x2f,
        0x53, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x3b, 0x29, 0x56, 0x01, 0x00, 0x0a, 0x53, 0x6f, 0x75,
        0x72, 0x63, 0x65, 0x46, 0x69, 0x6c, 0x65, 0x01, 0x00, 0x0f, 0x48, 0x65, 0x6c, 0x6c, 0x6f,
        0x57, 0x6f, 0x72, 0x6c, 0x64, 0x2e, 0x6a, 0x61, 0x76, 0x61, 0x00, 0x21, 0x00, 0x15, 0x00,
        0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x01, 0x00, 0x05, 0x00, 0x06, 0x00, 0x01,
        0x00, 0x17, 0x00, 0x00, 0x00, 0x1d, 0x00, 0x01, 0x00, 0x01, 0x00, 0x00, 0x00, 0x05, 0x2a,
        0xb7, 0x00, 0x01, 0xb1, 0x00, 0x00, 0x00, 0x01, 0x00, 0x18, 0x00, 0x00, 0x00, 0x06, 0x00,
        0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x09, 0x00, 0x19, 0x00, 0x1a, 0x00, 0x01, 0x00, 0x17,
        0x00, 0x00, 0x00, 0x21, 0x00, 0x02, 0x00, 0x01, 0x00, 0x00, 0x00, 0x09, 0xb2, 0x00, 0x07,
        0x12, 0x0d, 0xb6, 0x00, 0x0f, 0xb1, 0x00, 0x00, 0x00, 0x01, 0x00, 0x18, 0x00, 0x00, 0x00,
        0x06, 0x00, 0x01, 0x00, 0x00, 0x00, 0x02, 0x00, 0x01, 0x00, 0x1b, 0x00, 0x00, 0x00, 0x02,
        0x00, 0x1c,
    ];

    let classfile = ClassFile {
        magic: 3405691582,
        minor_version: 0,
        major_version: 65,
        constant_pool_count: 29,
        constant_pool: vec![
            ConstantMethodrefInfo {
                tag: 10,
                class_index: 1,
                name_and_type_index: 2,
            },
            ConstantClassInfo {
                tag: 7,
                name_index: 3,
            },
            ConstantNameAndTypeInfo {
                tag: 12,
                name_index: 4,
                descriptor_index: 5,
            },
            ConstantUtf8Info {
                tag: 1,
                length: 16,
                bytes: vec![
                    106, 97, 118, 97, 47, 108, 97, 110, 103, 47, 79, 98, 106, 101, 99, 116,
                ],
            },
            ConstantUtf8Info {
                tag: 1,
                length: 6,
                bytes: vec![60, 105, 110, 105, 116, 62],
            },
            ConstantUtf8Info {
                tag: 1,
                length: 3,
                bytes: vec![40, 41, 86],
            },
            ConstantFieldrefInfo {
                tag: 9,
                class_index: 7,
                name_and_type_index: 8,
            },
            ConstantClassInfo {
                tag: 7,
                name_index: 9,
            },
            ConstantNameAndTypeInfo {
                tag: 12,
                name_index: 10,
                descriptor_index: 11,
            },
            ConstantUtf8Info {
                tag: 1,
                length: 16,
                bytes: vec![
                    106, 97, 118, 97, 47, 108, 97, 110, 103, 47, 83, 121, 115, 116, 101, 109,
                ],
            },
            ConstantUtf8Info {
                tag: 1,
                length: 3,
                bytes: vec![111, 117, 116],
            },
            ConstantUtf8Info {
                tag: 1,
                length: 21,
                bytes: vec![
                    76, 106, 97, 118, 97, 47, 105, 111, 47, 80, 114, 105, 110, 116, 83, 116, 114,
                    101, 97, 109, 59,
                ],
            },
            ConstantStringInfo {
                tag: 8,
                string_index: 13,
            },
            ConstantUtf8Info {
                tag: 1,
                length: 12,
                bytes: vec![72, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108, 100],
            },
            ConstantMethodrefInfo {
                tag: 10,
                class_index: 15,
                name_and_type_index: 16,
            },
            ConstantClassInfo {
                tag: 7,
                name_index: 17,
            },
            ConstantNameAndTypeInfo {
                tag: 12,
                name_index: 18,
                descriptor_index: 19,
            },
            ConstantUtf8Info {
                tag: 1,
                length: 19,
                bytes: vec![
                    106, 97, 118, 97, 47, 105, 111, 47, 80, 114, 105, 110, 116, 83, 116, 114, 101,
                    97, 109,
                ],
            },
            ConstantUtf8Info {
                tag: 1,
                length: 7,
                bytes: vec![112, 114, 105, 110, 116, 108, 110],
            },
            ConstantUtf8Info {
                tag: 1,
                length: 21,
                bytes: vec![
                    40, 76, 106, 97, 118, 97, 47, 108, 97, 110, 103, 47, 83, 116, 114, 105, 110,
                    103, 59, 41, 86,
                ],
            },
            ConstantClassInfo {
                tag: 7,
                name_index: 21,
            },
            ConstantUtf8Info {
                tag: 1,
                length: 10,
                bytes: vec![72, 101, 108, 108, 111, 87, 111, 114, 108, 100],
            },
            ConstantUtf8Info {
                tag: 1,
                length: 4,
                bytes: vec![67, 111, 100, 101],
            },
            ConstantUtf8Info {
                tag: 1,
                length: 15,
                bytes: vec![
                    76, 105, 110, 101, 78, 117, 109, 98, 101, 114, 84, 97, 98, 108, 101,
                ],
            },
            ConstantUtf8Info {
                tag: 1,
                length: 4,
                bytes: vec![109, 97, 105, 110],
            },
            ConstantUtf8Info {
                tag: 1,
                length: 22,
                bytes: vec![
                    40, 91, 76, 106, 97, 118, 97, 47, 108, 97, 110, 103, 47, 83, 116, 114, 105,
                    110, 103, 59, 41, 86,
                ],
            },
            ConstantUtf8Info {
                tag: 1,
                length: 10,
                bytes: vec![83, 111, 117, 114, 99, 101, 70, 105, 108, 101],
            },
            ConstantUtf8Info {
                tag: 1,
                length: 15,
                bytes: vec![
                    72, 101, 108, 108, 111, 87, 111, 114, 108, 100, 46, 106, 97, 118, 97,
                ],
            },
        ],
        access_flags: 33,
        this_class: 20,
        super_class: 1,
        interfaces_count: 0,
        interfaces: vec![],
        fields_count: 0,
        fields: vec![],
        methods_count: 2,
        methods: vec![
            MethodInfo {
                access_flags: 1,
                name_index: 4,
                descriptor_index: 5,
                attributes_count: 1,
                attributes: vec![Code {
                    attribute_name_index: 22,
                    attribute_length: 29,
                    max_stack: 1,
                    max_locals: 1,
                    code_length: 5,
                    code: vec![42, 183, 0, 1, 177],
                    exception_table_length: 0,
                    exception_table: vec![],
                    code_attributes_count: 1,
                    code_attributes: vec![LineNumberTable {
                        attribute_name_index: 23,
                        attribute_length: 6,
                        line_number_table_length: 1,
                        line_number_table: vec![LineNumber {
                            start_pc: 0,
                            line_number: 1,
                        }],
                    }],
                }],
            },
            MethodInfo {
                access_flags: 9,
                name_index: 24,
                descriptor_index: 25,
                attributes_count: 1,
                attributes: vec![Code {
                    attribute_name_index: 22,
                    attribute_length: 33,
                    max_stack: 2,
                    max_locals: 1,
                    code_length: 9,
                    code: vec![178, 0, 7, 18, 13, 182, 0, 15, 177],
                    exception_table_length: 0,
                    exception_table: vec![],
                    code_attributes_count: 1,
                    code_attributes: vec![LineNumberTable {
                        attribute_name_index: 23,
                        attribute_length: 6,
                        line_number_table_length: 1,
                        line_number_table: vec![LineNumber {
                            start_pc: 0,
                            line_number: 2,
                        }],
                    }],
                }],
            },
        ],
        attributes_count: 1,
        attributes: vec![SourceFile {
            attribute_name_index: 26,
            attribute_length: 2,
            sourcefile_index: 27,
        }],
    };

    let mut bytes = Vec::new();
    let mut serializer = Serializer::new(Writer::new(&mut bytes));
    serializer.serialize(&classfile).unwrap();
    assert_eq!(expected_bytes, &bytes[..]);
}
