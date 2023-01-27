#[derive(Debug)]
pub struct ClassFile {
    pub magic: u32,
    pub minor_version: u16,
    pub major_version: u16,
    pub constant_pool_count: u16,
    pub constant_pool: Vec<constant_pool::types::CpInfo>,
    pub access_flags: u16,
    pub this_class: u16,
    pub super_class: u16,
    pub interfaces_count: u16,
    pub interfaces: Vec<u16>,
    pub fields_count: u16,
    pub fields: Vec<FieldInfo>,
    pub methods_count: u16,
    pub methods: Vec<MethodInfo>,
    pub attributes_count: u16,
    pub attributes: Vec<AttributeInfo>,
}

#[derive(Debug)]
pub struct FieldInfo {
    pub access_flags: u16,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub attributes_count: u16,
    pub attributes: Vec<AttributeInfo>,
}

#[derive(Debug)]
pub struct MethodInfo {
    pub access_flags: u16,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub attributes_count: u16,
    pub attributes: Vec<AttributeInfo>,
}

#[derive(Debug)]
pub enum AttributeInfo {
    SourceFile {
        attribute_name_index: u16,
        attribute_length: u32,
        sourcefile_index: u16,
    },

    ConstantValue {
        attribute_name_index: u16,
        attribute_length: u32,
        constantvalue_index: u16,
    },

    Code {
        attribute_name_index: u16,
        attribute_length: u32,
        max_stack: u16,
        max_locals: u16,
        code_length: u32,
        code: Vec<u8>,
        exception_table_length: u16,
        exception_table: Vec<ExceptionHandler>,
        code_attributes_count: u16,
        code_attributes: Vec<AttributeInfo>,
    },

    Exceptions {
        attribute_name_index: u16,
        attribute_length: u32,
        number_of_exceptions: u16,
        exception_index_table: Vec<u16>,
    },

    LineNumberTable {
        attribute_name_index: u16,
        attribute_length: u32,
        line_number_table_length: u16,
        line_number_table: Vec<LineNumber>,
    },

    LocalVariableTable {
        attribute_name_index: u16,
        attribute_length: u32,
        local_variable_table_length: u16,
        local_variable_table: Vec<LocalVariable>,
    },
}

pub mod predefined_attributes {
    pub const SOURCE_fILE: &'static str = "SourceFile";
    pub const CONSTANT_VALUE: &'static str = "ConstantValue";
    pub const CODE: &'static str = "Code";
    pub const EXCEPTIONS: &'static str = "Exceptions";
    pub const LINE_NUMBER_TABLE: &'static str = "LineNumberTable";
    pub const LOCAL_VARIABLE_TABLE: &'static str = "LocalVariableTable";
}

#[derive(Debug)]
pub struct ExceptionHandler {
    pub start_pc: u16,
    pub end_pc: u16,
    pub handler_pc: u16,
    pub catch_type: u16,
}

#[derive(Debug)]
pub struct LineNumber {
    pub start_pc: u16,
    pub line_number: u16,
}

#[derive(Debug)]
pub struct LocalVariable {
    pub start_pc: u16,
    pub length: u16,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub index: u16,
}

pub mod access_flags {
    pub const ACC_PUBLIC: u16 = 0x0001;
    pub const ACC_PRIVATE: u16 = 0x0002;
    pub const ACC_PROTECTED: u16 = 0x0004;
    pub const ACC_STATIC: u16 = 0x0008;
    pub const ACC_FINAL: u16 = 0x0010;
    pub const ACC_SUPER: u16 = 0x0020;
    pub const ACC_SYNCHRONIZED: u16 = 0x0020;
    pub const ACC_VOLATILE: u16 = 0x0040;
    pub const ACC_TRANSIENT: u16 = 0x0080;
    pub const ACC_NATIVE: u16 = 0x0100;
    pub const ACC_INTERFACE: u16 = 0x0200;
    pub const ACC_ABSTRACT: u16 = 0x0400;
}

pub mod constant_pool {
    pub mod tags {
        pub const CONSTANT_CLASS: u8 = 7;
        pub const CONSTANT_FIELDREF: u8 = 9;
        pub const CONSTANT_METHODREF: u8 = 10;
        pub const CONSTANT_INTERFACEMETHODREF: u8 = 11;
        pub const CONSTANT_STRING: u8 = 8;
        pub const CONSTANT_INTEGER: u8 = 3;
        pub const CONSTANT_FLOAT: u8 = 4;
        pub const CONSTANT_LONG: u8 = 5;
        pub const CONSTANT_DOUBLE: u8 = 6;
        pub const CONSTANT_NAMEANDTYPE: u8 = 12;
        pub const CONSTANT_UTF8: u8 = 1;
    }

    pub mod types {
        #[derive(Debug)]
        pub enum CpInfo {
            ConstantClassInfo {
                tag: u8,
                name_index: u16,
            },
            ConstantFieldrefInfo {
                tag: u8,
                class_index: u16,
                name_and_type_index: u16,
            },

            ConstantMethodrefInfo {
                tag: u8,
                class_index: u16,
                name_and_type_index: u16,
            },

            ConstantInterfaceMethodrefInfo {
                tag: u8,
                class_index: u16,
                name_and_type_index: u16,
            },

            ConstantStringInfo {
                tag: u8,
                string_index: u16,
            },

            ConstantIntegerInfo {
                tag: u8,
                bytes: u32,
            },

            ConstantFloatInfo {
                tag: u8,
                bytes: u32,
            },

            ConstantLongInfo {
                tag: u8,
                high_bytes: u32,
                low_bytes: u32,
            },

            ConstantDoubleInfo {
                tag: u8,
                high_bytes: u32,
                low_bytes: u32,
            },

            ConstantNameAndTypeInfo {
                tag: u8,
                name_index: u16,
                descriptor_index: u16,
            },

            ConstantUtf8Info {
                tag: u8,
                length: u16,
                bytes: Vec<u8>,
            },
        }
    }
}
