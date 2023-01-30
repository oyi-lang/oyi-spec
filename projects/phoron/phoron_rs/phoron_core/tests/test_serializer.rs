use phoron_core::{
    model::{
        attributes::{AttributeInfo::*, *},
        constant_pool::types::CpInfo::*,
        *,
    },
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
            None,
            Some(ConstantMethodrefInfo {
                tag: 10,
                class_index: 2,
                name_and_type_index: 3,
            }),
            Some(ConstantClassInfo {
                tag: 7,
                name_index: 4,
            }),
            Some(ConstantNameAndTypeInfo {
                tag: 12,
                name_index: 5,
                descriptor_index: 6,
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
                length: 6,
                bytes: vec![60, 105, 110, 105, 116, 62],
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 3,
                bytes: vec![40, 41, 86],
            }),
            Some(ConstantFieldrefInfo {
                tag: 9,
                class_index: 8,
                name_and_type_index: 9,
            }),
            Some(ConstantClassInfo {
                tag: 7,
                name_index: 10,
            }),
            Some(ConstantNameAndTypeInfo {
                tag: 12,
                name_index: 11,
                descriptor_index: 12,
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 16,
                bytes: vec![
                    106, 97, 118, 97, 47, 108, 97, 110, 103, 47, 83, 121, 115, 116, 101, 109,
                ],
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 3,
                bytes: vec![111, 117, 116],
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 21,
                bytes: vec![
                    76, 106, 97, 118, 97, 47, 105, 111, 47, 80, 114, 105, 110, 116, 83, 116, 114,
                    101, 97, 109, 59,
                ],
            }),
            Some(ConstantStringInfo {
                tag: 8,
                string_index: 14,
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 12,
                bytes: vec![72, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108, 100],
            }),
            Some(ConstantMethodrefInfo {
                tag: 10,
                class_index: 16,
                name_and_type_index: 17,
            }),
            Some(ConstantClassInfo {
                tag: 7,
                name_index: 18,
            }),
            Some(ConstantNameAndTypeInfo {
                tag: 12,
                name_index: 19,
                descriptor_index: 20,
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 19,
                bytes: vec![
                    106, 97, 118, 97, 47, 105, 111, 47, 80, 114, 105, 110, 116, 83, 116, 114, 101,
                    97, 109,
                ],
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 7,
                bytes: vec![112, 114, 105, 110, 116, 108, 110],
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 21,
                bytes: vec![
                    40, 76, 106, 97, 118, 97, 47, 108, 97, 110, 103, 47, 83, 116, 114, 105, 110,
                    103, 59, 41, 86,
                ],
            }),
            Some(ConstantClassInfo {
                tag: 7,
                name_index: 22,
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 10,
                bytes: vec![72, 101, 108, 108, 111, 87, 111, 114, 108, 100],
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 4,
                bytes: vec![67, 111, 100, 101],
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 15,
                bytes: vec![
                    76, 105, 110, 101, 78, 117, 109, 98, 101, 114, 84, 97, 98, 108, 101,
                ],
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 4,
                bytes: vec![109, 97, 105, 110],
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
                length: 10,
                bytes: vec![83, 111, 117, 114, 99, 101, 70, 105, 108, 101],
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 15,
                bytes: vec![
                    72, 101, 108, 108, 111, 87, 111, 114, 108, 100, 46, 106, 97, 118, 97,
                ],
            }),
        ],
        access_flags: 33,
        this_class: 21,
        super_class: 1,
        interfaces_count: 0,
        interfaces: vec![],
        fields_count: 0,
        fields: vec![],
        methods_count: 2,
        methods: vec![
            MethodInfo {
                access_flags: 1,
                name_index: 5,
                descriptor_index: 6,
                attributes_count: 1,
                attributes: vec![Code {
                    attribute_name_index: 23,
                    attribute_length: 29,
                    max_stack: 1,
                    max_locals: 1,
                    code_length: 5,
                    code: vec![42, 183, 0, 1, 177],
                    exception_table_length: 0,
                    exception_table: vec![],
                    code_attributes_count: 1,
                    code_attributes: vec![LineNumberTable {
                        attribute_name_index: 24,
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
                name_index: 25,
                descriptor_index: 26,
                attributes_count: 1,
                attributes: vec![Code {
                    attribute_name_index: 23,
                    attribute_length: 33,
                    max_stack: 2,
                    max_locals: 1,
                    code_length: 9,
                    code: vec![178, 0, 7, 18, 13, 182, 0, 15, 177],
                    exception_table_length: 0,
                    exception_table: vec![],
                    code_attributes_count: 1,
                    code_attributes: vec![LineNumberTable {
                        attribute_name_index: 24,
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
            attribute_name_index: 27,
            attribute_length: 2,
            sourcefile_index: 28,
        }],
    };
    let mut bytes = Vec::new();
    let mut serializer = Serializer::new(Writer::new(&mut bytes));
    let _ = serializer.serialize(&classfile);
    assert_eq!(expected_bytes, &bytes[..]);
}

#[test]
//Classfile /Users/z0ltan/dev/playground/Fields.class
//  Last modified 27-Jan-2023; size 1023 bytes
//  SHA-256 checksum 65434f38c6bb13a5bf08b4226f80394cfaba5cc5dcbb7cacd3145cb3336f49f2
//  Compiled from "Fields.java"
//public class Fields
//  minor version: 0
//  major version: 65
//  flags: (0x0021) ACC_PUBLIC, ACC_SUPER
//  this_class: #8                          // Fields
//  super_class: #2                         // java/lang/Object
//  interfaces: 0, fields: 5, methods: 2, attributes: 1
//Constant pool:
//   #1 = Methodref          #2.#3          // java/lang/Object."<init>":()V
//   #2 = Class              #4             // java/lang/Object
//   #3 = NameAndType        #5:#6          // "<init>":()V
//   #4 = Utf8               java/lang/Object
//   #5 = Utf8               <init>
//   #6 = Utf8               ()V
//   #7 = Fieldref           #8.#9          // Fields.one:I
//   #8 = Class              #10            // Fields
//   #9 = NameAndType        #11:#12        // one:I
//  #10 = Utf8               Fields
//  #11 = Utf8               one
//  #12 = Utf8               I
//  #13 = Fieldref           #8.#14         // Fields.two:Ljava/lang/String;
//  #14 = NameAndType        #15:#16        // two:Ljava/lang/String;
//  #15 = Utf8               two
//  #16 = Utf8               Ljava/lang/String;
//  #17 = Fieldref           #8.#18         // Fields.three:D
//  #18 = NameAndType        #19:#20        // three:D
//  #19 = Utf8               three
//  #20 = Utf8               D
//  #21 = Fieldref           #8.#22         // Fields.four:Z
//  #22 = NameAndType        #23:#24        // four:Z
//  #23 = Utf8               four
//  #24 = Utf8               Z
//  #25 = Fieldref           #8.#26         // Fields.five:Ljava/lang/Integer;
//  #26 = NameAndType        #27:#28        // five:Ljava/lang/Integer;
//  #27 = Utf8               five
//  #28 = Utf8               Ljava/lang/Integer;
//  #29 = String             #15            // two
//  #30 = Double             3.0d
//  #32 = Methodref          #33.#34        // java/lang/Integer.valueOf:(I)Ljava/lang/Integer;
//  #33 = Class              #35            // java/lang/Integer
//  #34 = NameAndType        #36:#37        // valueOf:(I)Ljava/lang/Integer;
//  #35 = Utf8               java/lang/Integer
//  #36 = Utf8               valueOf
//  #37 = Utf8               (I)Ljava/lang/Integer;
//  #38 = Methodref          #8.#39         // Fields."<init>":(ILjava/lang/String;DZLjava/lang/Integer;)V
//  #39 = NameAndType        #5:#40         // "<init>":(ILjava/lang/String;DZLjava/lang/Integer;)V
//  #40 = Utf8               (ILjava/lang/String;DZLjava/lang/Integer;)V
//  #41 = Fieldref           #42.#43        // java/lang/System.out:Ljava/io/PrintStream;
//  #42 = Class              #44            // java/lang/System
//  #43 = NameAndType        #45:#46        // out:Ljava/io/PrintStream;
//  #44 = Utf8               java/lang/System
//  #45 = Utf8               out
//  #46 = Utf8               Ljava/io/PrintStream;
//  #47 = String             #48            // %d, %s, %f, %b, %d\n
//  #48 = Utf8               %d, %s, %f, %b, %d\n
//  #49 = Methodref          #50.#51        // java/lang/Double.valueOf:(D)Ljava/lang/Double;
//  #50 = Class              #52            // java/lang/Double
//  #51 = NameAndType        #36:#53        // valueOf:(D)Ljava/lang/Double;
//  #52 = Utf8               java/lang/Double
//  #53 = Utf8               (D)Ljava/lang/Double;
//  #54 = Methodref          #55.#56        // java/lang/Boolean.valueOf:(Z)Ljava/lang/Boolean;
//  #55 = Class              #57            // java/lang/Boolean
//  #56 = NameAndType        #36:#58        // valueOf:(Z)Ljava/lang/Boolean;
//  #57 = Utf8               java/lang/Boolean
//  #58 = Utf8               (Z)Ljava/lang/Boolean;
//  #59 = Methodref          #60.#61        // java/io/PrintStream.printf:(Ljava/lang/String;[Ljava/lang/Object;)Ljava/io/PrintStream;
//  #60 = Class              #62            // java/io/PrintStream
//  #61 = NameAndType        #63:#64        // printf:(Ljava/lang/String;[Ljava/lang/Object;)Ljava/io/PrintStream;
//  #62 = Utf8               java/io/PrintStream
//  #63 = Utf8               printf
//  #64 = Utf8               (Ljava/lang/String;[Ljava/lang/Object;)Ljava/io/PrintStream;
//  #65 = Utf8               Code
//  #66 = Utf8               LineNumberTable
//  #67 = Utf8               main
//  #68 = Utf8               ([Ljava/lang/String;)V
//  #69 = Utf8               SourceFile
//  #70 = Utf8               Fields.java
//{
//  public double three;
//    descriptor: D
//    flags: (0x0001) ACC_PUBLIC
//
//  protected boolean four;
//    descriptor: Z
//    flags: (0x0004) ACC_PROTECTED
//
//  java.lang.Integer five;
//    descriptor: Ljava/lang/Integer;
//    flags: (0x0000)
//
//  Fields(int, java.lang.String, double, boolean, java.lang.Integer);
//    descriptor: (ILjava/lang/String;DZLjava/lang/Integer;)V
//    flags: (0x0000)
//    Code:
//      stack=3, locals=7, args_size=6
//         0: aload_0
//         1: invokespecial #1                  // Method java/lang/Object."<init>":()V
//         4: aload_0
//         5: iload_1
//         6: putfield      #7                  // Field one:I
//         9: aload_0
//        10: aload_2
//        11: putfield      #13                 // Field two:Ljava/lang/String;
//        14: aload_0
//        15: dload_3
//        16: putfield      #17                 // Field three:D
//        19: aload_0
//        20: iload         5
//        22: putfield      #21                 // Field four:Z
//        25: aload_0
//        26: aload         6
//        28: putfield      #25                 // Field five:Ljava/lang/Integer;
//        31: return
//      LineNumberTable:
//        line 8: 0
//        line 9: 4
//        line 10: 9
//        line 11: 14
//        line 12: 19
//        line 13: 25
//        line 14: 31
//
//  public static void main(java.lang.String[]);
//    descriptor: ([Ljava/lang/String;)V
//    flags: (0x0009) ACC_PUBLIC, ACC_STATIC
//    Code:
//      stack=8, locals=2, args_size=1
//         0: new           #8                  // class Fields
//         3: dup
//         4: iconst_1
//         5: ldc           #29                 // String two
//         7: ldc2_w        #30                 // double 3.0d
//        10: iconst_1
//        11: iconst_5
//        12: invokestatic  #32                 // Method java/lang/Integer.valueOf:(I)Ljava/lang/Integer;
//        15: invokespecial #38                 // Method "<init>":(ILjava/lang/String;DZLjava/lang/Integer;)V
//        18: astore_1
//        19: getstatic     #41                 // Field java/lang/System.out:Ljava/io/PrintStream;
//        22: ldc           #47                 // String %d, %s, %f, %b, %d\n
//        24: iconst_5
//        25: anewarray     #2                  // class java/lang/Object
//        28: dup
//        29: iconst_0
//        30: aload_1
//        31: getfield      #7                  // Field one:I
//        34: invokestatic  #32                 // Method java/lang/Integer.valueOf:(I)Ljava/lang/Integer;
//        37: aastore
//        38: dup
//        39: iconst_1
//        40: aload_1
//        41: getfield      #13                 // Field two:Ljava/lang/String;
//        44: aastore
//        45: dup
//        46: iconst_2
//        47: aload_1
//        48: getfield      #17                 // Field three:D
//        51: invokestatic  #49                 // Method java/lang/Double.valueOf:(D)Ljava/lang/Double;
//        54: aastore
//        55: dup
//        56: iconst_3
//        57: aload_1
//        58: getfield      #21                 // Field four:Z
//        61: invokestatic  #54                 // Method java/lang/Boolean.valueOf:(Z)Ljava/lang/Boolean;
//        64: aastore
//        65: dup
//        66: iconst_4
//        67: aload_1
//        68: getfield      #25                 // Field five:Ljava/lang/Integer;
//        71: aastore
//        72: invokevirtual #59                 // Method java/io/PrintStream.printf:(Ljava/lang/String;[Ljava/lang/Object;)Ljava/io/PrintStream;
//        75: pop
//        76: return
//      LineNumberTable:
//        line 17: 0
//        line 18: 19
//        line 19: 51
//        line 18: 72
//        line 20: 76
//}
//SourceFile: "Fields.java"
fn test_serialize_fields() {
    let expected_bytes = [
        0xca, 0xfe, 0xba, 0xbe, 0x00, 0x00, 0x00, 0x41, 0x00, 0x47, 0x0a, 0x00, 0x02, 0x00, 0x03,
        0x07, 0x00, 0x04, 0x0c, 0x00, 0x05, 0x00, 0x06, 0x01, 0x00, 0x10, 0x6a, 0x61, 0x76, 0x61,
        0x2f, 0x6c, 0x61, 0x6e, 0x67, 0x2f, 0x4f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x01, 0x00, 0x06,
        0x3c, 0x69, 0x6e, 0x69, 0x74, 0x3e, 0x01, 0x00, 0x03, 0x28, 0x29, 0x56, 0x09, 0x00, 0x08,
        0x00, 0x09, 0x07, 0x00, 0x0a, 0x0c, 0x00, 0x0b, 0x00, 0x0c, 0x01, 0x00, 0x06, 0x46, 0x69,
        0x65, 0x6c, 0x64, 0x73, 0x01, 0x00, 0x03, 0x6f, 0x6e, 0x65, 0x01, 0x00, 0x01, 0x49, 0x09,
        0x00, 0x08, 0x00, 0x0e, 0x0c, 0x00, 0x0f, 0x00, 0x10, 0x01, 0x00, 0x03, 0x74, 0x77, 0x6f,
        0x01, 0x00, 0x12, 0x4c, 0x6a, 0x61, 0x76, 0x61, 0x2f, 0x6c, 0x61, 0x6e, 0x67, 0x2f, 0x53,
        0x74, 0x72, 0x69, 0x6e, 0x67, 0x3b, 0x09, 0x00, 0x08, 0x00, 0x12, 0x0c, 0x00, 0x13, 0x00,
        0x14, 0x01, 0x00, 0x05, 0x74, 0x68, 0x72, 0x65, 0x65, 0x01, 0x00, 0x01, 0x44, 0x09, 0x00,
        0x08, 0x00, 0x16, 0x0c, 0x00, 0x17, 0x00, 0x18, 0x01, 0x00, 0x04, 0x66, 0x6f, 0x75, 0x72,
        0x01, 0x00, 0x01, 0x5a, 0x09, 0x00, 0x08, 0x00, 0x1a, 0x0c, 0x00, 0x1b, 0x00, 0x1c, 0x01,
        0x00, 0x04, 0x66, 0x69, 0x76, 0x65, 0x01, 0x00, 0x13, 0x4c, 0x6a, 0x61, 0x76, 0x61, 0x2f,
        0x6c, 0x61, 0x6e, 0x67, 0x2f, 0x49, 0x6e, 0x74, 0x65, 0x67, 0x65, 0x72, 0x3b, 0x08, 0x00,
        0x0f, 0x06, 0x40, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0a, 0x00, 0x21, 0x00, 0x22,
        0x07, 0x00, 0x23, 0x0c, 0x00, 0x24, 0x00, 0x25, 0x01, 0x00, 0x11, 0x6a, 0x61, 0x76, 0x61,
        0x2f, 0x6c, 0x61, 0x6e, 0x67, 0x2f, 0x49, 0x6e, 0x74, 0x65, 0x67, 0x65, 0x72, 0x01, 0x00,
        0x07, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x4f, 0x66, 0x01, 0x00, 0x16, 0x28, 0x49, 0x29, 0x4c,
        0x6a, 0x61, 0x76, 0x61, 0x2f, 0x6c, 0x61, 0x6e, 0x67, 0x2f, 0x49, 0x6e, 0x74, 0x65, 0x67,
        0x65, 0x72, 0x3b, 0x0a, 0x00, 0x08, 0x00, 0x27, 0x0c, 0x00, 0x05, 0x00, 0x28, 0x01, 0x00,
        0x2b, 0x28, 0x49, 0x4c, 0x6a, 0x61, 0x76, 0x61, 0x2f, 0x6c, 0x61, 0x6e, 0x67, 0x2f, 0x53,
        0x74, 0x72, 0x69, 0x6e, 0x67, 0x3b, 0x44, 0x5a, 0x4c, 0x6a, 0x61, 0x76, 0x61, 0x2f, 0x6c,
        0x61, 0x6e, 0x67, 0x2f, 0x49, 0x6e, 0x74, 0x65, 0x67, 0x65, 0x72, 0x3b, 0x29, 0x56, 0x09,
        0x00, 0x2a, 0x00, 0x2b, 0x07, 0x00, 0x2c, 0x0c, 0x00, 0x2d, 0x00, 0x2e, 0x01, 0x00, 0x10,
        0x6a, 0x61, 0x76, 0x61, 0x2f, 0x6c, 0x61, 0x6e, 0x67, 0x2f, 0x53, 0x79, 0x73, 0x74, 0x65,
        0x6d, 0x01, 0x00, 0x03, 0x6f, 0x75, 0x74, 0x01, 0x00, 0x15, 0x4c, 0x6a, 0x61, 0x76, 0x61,
        0x2f, 0x69, 0x6f, 0x2f, 0x50, 0x72, 0x69, 0x6e, 0x74, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d,
        0x3b, 0x08, 0x00, 0x30, 0x01, 0x00, 0x13, 0x25, 0x64, 0x2c, 0x20, 0x25, 0x73, 0x2c, 0x20,
        0x25, 0x66, 0x2c, 0x20, 0x25, 0x62, 0x2c, 0x20, 0x25, 0x64, 0x0a, 0x0a, 0x00, 0x32, 0x00,
        0x33, 0x07, 0x00, 0x34, 0x0c, 0x00, 0x24, 0x00, 0x35, 0x01, 0x00, 0x10, 0x6a, 0x61, 0x76,
        0x61, 0x2f, 0x6c, 0x61, 0x6e, 0x67, 0x2f, 0x44, 0x6f, 0x75, 0x62, 0x6c, 0x65, 0x01, 0x00,
        0x15, 0x28, 0x44, 0x29, 0x4c, 0x6a, 0x61, 0x76, 0x61, 0x2f, 0x6c, 0x61, 0x6e, 0x67, 0x2f,
        0x44, 0x6f, 0x75, 0x62, 0x6c, 0x65, 0x3b, 0x0a, 0x00, 0x37, 0x00, 0x38, 0x07, 0x00, 0x39,
        0x0c, 0x00, 0x24, 0x00, 0x3a, 0x01, 0x00, 0x11, 0x6a, 0x61, 0x76, 0x61, 0x2f, 0x6c, 0x61,
        0x6e, 0x67, 0x2f, 0x42, 0x6f, 0x6f, 0x6c, 0x65, 0x61, 0x6e, 0x01, 0x00, 0x16, 0x28, 0x5a,
        0x29, 0x4c, 0x6a, 0x61, 0x76, 0x61, 0x2f, 0x6c, 0x61, 0x6e, 0x67, 0x2f, 0x42, 0x6f, 0x6f,
        0x6c, 0x65, 0x61, 0x6e, 0x3b, 0x0a, 0x00, 0x3c, 0x00, 0x3d, 0x07, 0x00, 0x3e, 0x0c, 0x00,
        0x3f, 0x00, 0x40, 0x01, 0x00, 0x13, 0x6a, 0x61, 0x76, 0x61, 0x2f, 0x69, 0x6f, 0x2f, 0x50,
        0x72, 0x69, 0x6e, 0x74, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x01, 0x00, 0x06, 0x70, 0x72,
        0x69, 0x6e, 0x74, 0x66, 0x01, 0x00, 0x3c, 0x28, 0x4c, 0x6a, 0x61, 0x76, 0x61, 0x2f, 0x6c,
        0x61, 0x6e, 0x67, 0x2f, 0x53, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x3b, 0x5b, 0x4c, 0x6a, 0x61,
        0x76, 0x61, 0x2f, 0x6c, 0x61, 0x6e, 0x67, 0x2f, 0x4f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x3b,
        0x29, 0x4c, 0x6a, 0x61, 0x76, 0x61, 0x2f, 0x69, 0x6f, 0x2f, 0x50, 0x72, 0x69, 0x6e, 0x74,
        0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x3b, 0x01, 0x00, 0x04, 0x43, 0x6f, 0x64, 0x65, 0x01,
        0x00, 0x0f, 0x4c, 0x69, 0x6e, 0x65, 0x4e, 0x75, 0x6d, 0x62, 0x65, 0x72, 0x54, 0x61, 0x62,
        0x6c, 0x65, 0x01, 0x00, 0x04, 0x6d, 0x61, 0x69, 0x6e, 0x01, 0x00, 0x16, 0x28, 0x5b, 0x4c,
        0x6a, 0x61, 0x76, 0x61, 0x2f, 0x6c, 0x61, 0x6e, 0x67, 0x2f, 0x53, 0x74, 0x72, 0x69, 0x6e,
        0x67, 0x3b, 0x29, 0x56, 0x01, 0x00, 0x0a, 0x53, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x46, 0x69,
        0x6c, 0x65, 0x01, 0x00, 0x0b, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x73, 0x2e, 0x6a, 0x61, 0x76,
        0x61, 0x00, 0x21, 0x00, 0x08, 0x00, 0x02, 0x00, 0x00, 0x00, 0x05, 0x00, 0x02, 0x00, 0x0b,
        0x00, 0x0c, 0x00, 0x00, 0x00, 0x02, 0x00, 0x0f, 0x00, 0x10, 0x00, 0x00, 0x00, 0x01, 0x00,
        0x13, 0x00, 0x14, 0x00, 0x00, 0x00, 0x04, 0x00, 0x17, 0x00, 0x18, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x1b, 0x00, 0x1c, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x05, 0x00, 0x28, 0x00,
        0x01, 0x00, 0x41, 0x00, 0x00, 0x00, 0x50, 0x00, 0x03, 0x00, 0x07, 0x00, 0x00, 0x00, 0x20,
        0x2a, 0xb7, 0x00, 0x01, 0x2a, 0x1b, 0xb5, 0x00, 0x07, 0x2a, 0x2c, 0xb5, 0x00, 0x0d, 0x2a,
        0x29, 0xb5, 0x00, 0x11, 0x2a, 0x15, 0x05, 0xb5, 0x00, 0x15, 0x2a, 0x19, 0x06, 0xb5, 0x00,
        0x19, 0xb1, 0x00, 0x00, 0x00, 0x01, 0x00, 0x42, 0x00, 0x00, 0x00, 0x1e, 0x00, 0x07, 0x00,
        0x00, 0x00, 0x08, 0x00, 0x04, 0x00, 0x09, 0x00, 0x09, 0x00, 0x0a, 0x00, 0x0e, 0x00, 0x0b,
        0x00, 0x13, 0x00, 0x0c, 0x00, 0x19, 0x00, 0x0d, 0x00, 0x1f, 0x00, 0x0e, 0x00, 0x09, 0x00,
        0x43, 0x00, 0x44, 0x00, 0x01, 0x00, 0x41, 0x00, 0x00, 0x00, 0x75, 0x00, 0x08, 0x00, 0x02,
        0x00, 0x00, 0x00, 0x4d, 0xbb, 0x00, 0x08, 0x59, 0x04, 0x12, 0x1d, 0x14, 0x00, 0x1e, 0x04,
        0x08, 0xb8, 0x00, 0x20, 0xb7, 0x00, 0x26, 0x4c, 0xb2, 0x00, 0x29, 0x12, 0x2f, 0x08, 0xbd,
        0x00, 0x02, 0x59, 0x03, 0x2b, 0xb4, 0x00, 0x07, 0xb8, 0x00, 0x20, 0x53, 0x59, 0x04, 0x2b,
        0xb4, 0x00, 0x0d, 0x53, 0x59, 0x05, 0x2b, 0xb4, 0x00, 0x11, 0xb8, 0x00, 0x31, 0x53, 0x59,
        0x06, 0x2b, 0xb4, 0x00, 0x15, 0xb8, 0x00, 0x36, 0x53, 0x59, 0x07, 0x2b, 0xb4, 0x00, 0x19,
        0x53, 0xb6, 0x00, 0x3b, 0x57, 0xb1, 0x00, 0x00, 0x00, 0x01, 0x00, 0x42, 0x00, 0x00, 0x00,
        0x16, 0x00, 0x05, 0x00, 0x00, 0x00, 0x11, 0x00, 0x13, 0x00, 0x12, 0x00, 0x33, 0x00, 0x13,
        0x00, 0x48, 0x00, 0x12, 0x00, 0x4c, 0x00, 0x14, 0x00, 0x01, 0x00, 0x45, 0x00, 0x00, 0x00,
        0x02, 0x00, 0x46,
    ];

    let classfile = ClassFile {
        magic: 3405691582,
        minor_version: 0,
        major_version: 65,
        constant_pool_count: 71,
        constant_pool: vec![
            None,
            Some(ConstantMethodrefInfo {
                tag: 10,
                class_index: 2,
                name_and_type_index: 3,
            }),
            Some(ConstantClassInfo {
                tag: 7,
                name_index: 4,
            }),
            Some(ConstantNameAndTypeInfo {
                tag: 12,
                name_index: 5,
                descriptor_index: 6,
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
                length: 6,
                bytes: vec![60, 105, 110, 105, 116, 62],
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 3,
                bytes: vec![40, 41, 86],
            }),
            Some(ConstantFieldrefInfo {
                tag: 9,
                class_index: 8,
                name_and_type_index: 9,
            }),
            Some(ConstantClassInfo {
                tag: 7,
                name_index: 10,
            }),
            Some(ConstantNameAndTypeInfo {
                tag: 12,
                name_index: 11,
                descriptor_index: 12,
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 6,
                bytes: vec![70, 105, 101, 108, 100, 115],
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 3,
                bytes: vec![111, 110, 101],
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 1,
                bytes: vec![73],
            }),
            Some(ConstantFieldrefInfo {
                tag: 9,
                class_index: 8,
                name_and_type_index: 14,
            }),
            Some(ConstantNameAndTypeInfo {
                tag: 12,
                name_index: 15,
                descriptor_index: 16,
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 3,
                bytes: vec![116, 119, 111],
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 18,
                bytes: vec![
                    76, 106, 97, 118, 97, 47, 108, 97, 110, 103, 47, 83, 116, 114, 105, 110, 103,
                    59,
                ],
            }),
            Some(ConstantFieldrefInfo {
                tag: 9,
                class_index: 8,
                name_and_type_index: 18,
            }),
            Some(ConstantNameAndTypeInfo {
                tag: 12,
                name_index: 19,
                descriptor_index: 20,
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 5,
                bytes: vec![116, 104, 114, 101, 101],
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 1,
                bytes: vec![68],
            }),
            Some(ConstantFieldrefInfo {
                tag: 9,
                class_index: 8,
                name_and_type_index: 22,
            }),
            Some(ConstantNameAndTypeInfo {
                tag: 12,
                name_index: 23,
                descriptor_index: 24,
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 4,
                bytes: vec![102, 111, 117, 114],
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 1,
                bytes: vec![90],
            }),
            Some(ConstantFieldrefInfo {
                tag: 9,
                class_index: 8,
                name_and_type_index: 26,
            }),
            Some(ConstantNameAndTypeInfo {
                tag: 12,
                name_index: 27,
                descriptor_index: 28,
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 4,
                bytes: vec![102, 105, 118, 101],
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 19,
                bytes: vec![
                    76, 106, 97, 118, 97, 47, 108, 97, 110, 103, 47, 73, 110, 116, 101, 103, 101,
                    114, 59,
                ],
            }),
            Some(ConstantStringInfo {
                tag: 8,
                string_index: 15,
            }),
            Some(ConstantDoubleInfo {
                tag: 6,
                high_bytes: 1074266112,
                low_bytes: 0,
            }),
            None,
            Some(ConstantMethodrefInfo {
                tag: 10,
                class_index: 33,
                name_and_type_index: 34,
            }),
            Some(ConstantClassInfo {
                tag: 7,
                name_index: 35,
            }),
            Some(ConstantNameAndTypeInfo {
                tag: 12,
                name_index: 36,
                descriptor_index: 37,
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 17,
                bytes: vec![
                    106, 97, 118, 97, 47, 108, 97, 110, 103, 47, 73, 110, 116, 101, 103, 101, 114,
                ],
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 7,
                bytes: vec![118, 97, 108, 117, 101, 79, 102],
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 22,
                bytes: vec![
                    40, 73, 41, 76, 106, 97, 118, 97, 47, 108, 97, 110, 103, 47, 73, 110, 116, 101,
                    103, 101, 114, 59,
                ],
            }),
            Some(ConstantMethodrefInfo {
                tag: 10,
                class_index: 8,
                name_and_type_index: 39,
            }),
            Some(ConstantNameAndTypeInfo {
                tag: 12,
                name_index: 5,
                descriptor_index: 40,
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 43,
                bytes: vec![
                    40, 73, 76, 106, 97, 118, 97, 47, 108, 97, 110, 103, 47, 83, 116, 114, 105,
                    110, 103, 59, 68, 90, 76, 106, 97, 118, 97, 47, 108, 97, 110, 103, 47, 73, 110,
                    116, 101, 103, 101, 114, 59, 41, 86,
                ],
            }),
            Some(ConstantFieldrefInfo {
                tag: 9,
                class_index: 42,
                name_and_type_index: 43,
            }),
            Some(ConstantClassInfo {
                tag: 7,
                name_index: 44,
            }),
            Some(ConstantNameAndTypeInfo {
                tag: 12,
                name_index: 45,
                descriptor_index: 46,
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 16,
                bytes: vec![
                    106, 97, 118, 97, 47, 108, 97, 110, 103, 47, 83, 121, 115, 116, 101, 109,
                ],
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 3,
                bytes: vec![111, 117, 116],
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 21,
                bytes: vec![
                    76, 106, 97, 118, 97, 47, 105, 111, 47, 80, 114, 105, 110, 116, 83, 116, 114,
                    101, 97, 109, 59,
                ],
            }),
            Some(ConstantStringInfo {
                tag: 8,
                string_index: 48,
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 19,
                bytes: vec![
                    37, 100, 44, 32, 37, 115, 44, 32, 37, 102, 44, 32, 37, 98, 44, 32, 37, 100, 10,
                ],
            }),
            Some(ConstantMethodrefInfo {
                tag: 10,
                class_index: 50,
                name_and_type_index: 51,
            }),
            Some(ConstantClassInfo {
                tag: 7,
                name_index: 52,
            }),
            Some(ConstantNameAndTypeInfo {
                tag: 12,
                name_index: 36,
                descriptor_index: 53,
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 16,
                bytes: vec![
                    106, 97, 118, 97, 47, 108, 97, 110, 103, 47, 68, 111, 117, 98, 108, 101,
                ],
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 21,
                bytes: vec![
                    40, 68, 41, 76, 106, 97, 118, 97, 47, 108, 97, 110, 103, 47, 68, 111, 117, 98,
                    108, 101, 59,
                ],
            }),
            Some(ConstantMethodrefInfo {
                tag: 10,
                class_index: 55,
                name_and_type_index: 56,
            }),
            Some(ConstantClassInfo {
                tag: 7,
                name_index: 57,
            }),
            Some(ConstantNameAndTypeInfo {
                tag: 12,
                name_index: 36,
                descriptor_index: 58,
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 17,
                bytes: vec![
                    106, 97, 118, 97, 47, 108, 97, 110, 103, 47, 66, 111, 111, 108, 101, 97, 110,
                ],
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 22,
                bytes: vec![
                    40, 90, 41, 76, 106, 97, 118, 97, 47, 108, 97, 110, 103, 47, 66, 111, 111, 108,
                    101, 97, 110, 59,
                ],
            }),
            Some(ConstantMethodrefInfo {
                tag: 10,
                class_index: 60,
                name_and_type_index: 61,
            }),
            Some(ConstantClassInfo {
                tag: 7,
                name_index: 62,
            }),
            Some(ConstantNameAndTypeInfo {
                tag: 12,
                name_index: 63,
                descriptor_index: 64,
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 19,
                bytes: vec![
                    106, 97, 118, 97, 47, 105, 111, 47, 80, 114, 105, 110, 116, 83, 116, 114, 101,
                    97, 109,
                ],
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 6,
                bytes: vec![112, 114, 105, 110, 116, 102],
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 60,
                bytes: vec![
                    40, 76, 106, 97, 118, 97, 47, 108, 97, 110, 103, 47, 83, 116, 114, 105, 110,
                    103, 59, 91, 76, 106, 97, 118, 97, 47, 108, 97, 110, 103, 47, 79, 98, 106, 101,
                    99, 116, 59, 41, 76, 106, 97, 118, 97, 47, 105, 111, 47, 80, 114, 105, 110,
                    116, 83, 116, 114, 101, 97, 109, 59,
                ],
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 4,
                bytes: vec![67, 111, 100, 101],
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 15,
                bytes: vec![
                    76, 105, 110, 101, 78, 117, 109, 98, 101, 114, 84, 97, 98, 108, 101,
                ],
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 4,
                bytes: vec![109, 97, 105, 110],
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
                length: 10,
                bytes: vec![83, 111, 117, 114, 99, 101, 70, 105, 108, 101],
            }),
            Some(ConstantUtf8Info {
                tag: 1,
                length: 11,
                bytes: vec![70, 105, 101, 108, 100, 115, 46, 106, 97, 118, 97],
            }),
        ],
        access_flags: 33,
        this_class: 8,
        super_class: 1,
        interfaces_count: 0,
        interfaces: vec![],
        fields_count: 5,
        fields: vec![
            FieldInfo {
                access_flags: 2,
                name_index: 11,
                descriptor_index: 12,
                attributes_count: 0,
                attributes: vec![],
            },
            FieldInfo {
                access_flags: 2,
                name_index: 15,
                descriptor_index: 16,
                attributes_count: 0,
                attributes: vec![],
            },
            FieldInfo {
                access_flags: 1,
                name_index: 19,
                descriptor_index: 20,
                attributes_count: 0,
                attributes: vec![],
            },
            FieldInfo {
                access_flags: 4,
                name_index: 23,
                descriptor_index: 24,
                attributes_count: 0,
                attributes: vec![],
            },
            FieldInfo {
                access_flags: 0,
                name_index: 27,
                descriptor_index: 28,
                attributes_count: 0,
                attributes: vec![],
            },
        ],
        methods_count: 2,
        methods: vec![
            MethodInfo {
                access_flags: 0,
                name_index: 5,
                descriptor_index: 40,
                attributes_count: 1,
                attributes: vec![Code {
                    attribute_name_index: 65,
                    attribute_length: 80,
                    max_stack: 3,
                    max_locals: 7,
                    code_length: 32,
                    code: vec![
                        42, 183, 0, 1, 42, 27, 181, 0, 7, 42, 44, 181, 0, 13, 42, 41, 181, 0, 17,
                        42, 21, 5, 181, 0, 21, 42, 25, 6, 181, 0, 25, 177,
                    ],
                    exception_table_length: 0,
                    exception_table: vec![],
                    code_attributes_count: 1,
                    code_attributes: vec![LineNumberTable {
                        attribute_name_index: 66,
                        attribute_length: 30,
                        line_number_table_length: 7,
                        line_number_table: vec![
                            LineNumber {
                                start_pc: 0,
                                line_number: 8,
                            },
                            LineNumber {
                                start_pc: 4,
                                line_number: 9,
                            },
                            LineNumber {
                                start_pc: 9,
                                line_number: 10,
                            },
                            LineNumber {
                                start_pc: 14,
                                line_number: 11,
                            },
                            LineNumber {
                                start_pc: 19,
                                line_number: 12,
                            },
                            LineNumber {
                                start_pc: 25,
                                line_number: 13,
                            },
                            LineNumber {
                                start_pc: 31,
                                line_number: 14,
                            },
                        ],
                    }],
                }],
            },
            MethodInfo {
                access_flags: 9,
                name_index: 67,
                descriptor_index: 68,
                attributes_count: 1,
                attributes: vec![Code {
                    attribute_name_index: 65,
                    attribute_length: 117,
                    max_stack: 8,
                    max_locals: 2,
                    code_length: 77,
                    code: vec![
                        187, 0, 8, 89, 4, 18, 29, 20, 0, 30, 4, 8, 184, 0, 32, 183, 0, 38, 76, 178,
                        0, 41, 18, 47, 8, 189, 0, 2, 89, 3, 43, 180, 0, 7, 184, 0, 32, 83, 89, 4,
                        43, 180, 0, 13, 83, 89, 5, 43, 180, 0, 17, 184, 0, 49, 83, 89, 6, 43, 180,
                        0, 21, 184, 0, 54, 83, 89, 7, 43, 180, 0, 25, 83, 182, 0, 59, 87, 177,
                    ],
                    exception_table_length: 0,
                    exception_table: vec![],
                    code_attributes_count: 1,
                    code_attributes: vec![LineNumberTable {
                        attribute_name_index: 66,
                        attribute_length: 22,
                        line_number_table_length: 5,
                        line_number_table: vec![
                            LineNumber {
                                start_pc: 0,
                                line_number: 17,
                            },
                            LineNumber {
                                start_pc: 19,
                                line_number: 18,
                            },
                            LineNumber {
                                start_pc: 51,
                                line_number: 19,
                            },
                            LineNumber {
                                start_pc: 72,
                                line_number: 18,
                            },
                            LineNumber {
                                start_pc: 76,
                                line_number: 20,
                            },
                        ],
                    }],
                }],
            },
        ],
        attributes_count: 1,
        attributes: vec![SourceFile {
            attribute_name_index: 69,
            attribute_length: 2,
            sourcefile_index: 70,
        }],
    };

    let mut bytes = Vec::new();
    let mut serializer = Serializer::new(Writer::new(&mut bytes));
    let _ = serializer.serialize(&classfile);
    assert_eq!(expected_bytes, &bytes[..]);
}
