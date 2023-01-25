Source: The Java Virtual Machine Specification (version 1.0.2) (ISBN: 0-201-63452-X).

## ClassFile

Note: All bytes are unsigned 8-bit bytes. `u2` refers to a 16-bit value, `u4` to a 32-bit value, and so on. All bytes are laid out in Network Order (Big-Endian).

```
ClassFile {
  u4 magic;
  u2 minor_version;
  u2 major_version;
  u2 constant_pool_count;
  cp_info constant_pool[constant_pool_count - 1];
  u2 access_flags;
  u2 this_class;
  u2 super_class;
  u2 interfaces_count;
  u2 interfaces[interfaces_count];
  u2 fields_count;
  field_info fields[fields_count];
  u2 methods_count;
  method_info methods[methods_count];
  u2 attributes_count;
  attribute_info attributes[attributes_count];
}

```

### Access Flags

```
ACC_PUBLIC     0x0001
ACC_FINAL      0x0010
ACC_SUPER      0x0020
ACC_INTERFACE  0x0200
ACC_ABSTRACT   0x0400
```

## Cnnstant Pool

```
cp_info {
  u1 tag;
  u1 info[];
}

```

### Constant Pool Tags

```
CONSTANT_Class               7
CONSTANT_Fieldref            9
CONSTANT_MethodRref          10
CONSTANT_InterfaceMethodref  11
CONSTANT_String              8
CONSTANT_Integer             3
CONSTANT_Float               4
CONSTANT_Long                5
CONSTANT_Double              6
CONSTANT_NameAndType         12
CONSTANT_Utf8                1
```

### Constant Pool Types

```
CONSTANT_Class_info {
  u1 tag;
  u2 name_index;
}

CONSTANT_Fieldref_info {
  u1 tag;
  u2 class_index;
  u2 name_and_type_index;
}

CONSTANT_Methodref_info {
  u1 tag;
  u2 class_index;
  u2 name_and_type_index;
}

CONSTANT_InterfaceMethodref_info {
  u1 tag;
  u2 class_index;
  u2 name_and_type_index;
}

CONSTANT_String {
  u1 tag;
  u2 string_index;
}

CONSTANT_Integer_info {
  u1 tag;
  u4 bytes;
}

CONSTANT_Float_info {
  u1 tag;
  u4 bytes; 
}

CONSTANT_Long_info {
  u1 tag;
  u4 high_bytes;
  u4 low_bytes;
}

CONSTANT_Double_info {
  u1 tag;
  u4 high_bytes;
  u4 low_bytes;
}

CONSTANT_NameAndType_info {
  u1 tag;
  u2 name_index;
  u2 descriptor_index;
}

CONSTANT_Utf8_info {
  u1 tag;
  u2 length;
  u1 bytes[length];
}

```

## Fields


```
field_info {
  u2 access_flags;
  u2 name_index;
  u2 descriptor_index;
  u2 attributes_count;
  attribute_info attributes[attributes_count];
}
```

### Access Flags

```
ACC_PUBLIC     0x0001
ACC_PRIVATE    0x0002
ACC_PROTECTED  0x0004
ACC_STATIC     0x0008
ACC_FINAL      0x0010
ACC_VOLATILE   0x0040
ACC_TRANSIENT  0x0080
```

## Methods


```
method_info {
  u2 access_flags;
  u2 name_index;
  u2 descriptor_index;
  u2 attributes_count;
  attribute_info attributes[attributes_count];
}
```

### Access Flags

```
ACC_PUBLIC        0x0001
ACC_PRIVATE       0x0002
ACC_PROTECTED     0x0004
ACC_STATIC        0x0008
ACC_FINAL         0x0010
ACC_SYNCHRONIZED  0x0020
ACC_NATIVE        0x0100
ACC_ABSTRACT      0x0400
```

## Attributes


```
attribute_info {
  u2 attribute_name_index;
  u4 attribute_length;
  u1 info[attribute_length];
}
```

### Predefined Attributes

```
SourceFile_attribute {
  u2 attribute_name_index;
  u2 attribute_length;
  u2 sourcefile_index;
}

ConstantValue_attribute {
  u2 attribute_name_index;
  u2 attribute_length;
  u2 constantvalue_index;
}

Code_attribute {
  u2 attribute_name_index;
  u4 attribute_length;
  u2 max_stack;
  u2 max_locals;
  u4 code_length;
  u1 code[code_length];
  u2 exception_table_length;
  {
    u2 start_pc;
    u2 end_pc;
    u2 handler_pc;
    u2 catch_type;
  } exception_table[exception_table_length];
  u2 attributes_count;
  attribute_info attributes[attributes_count];
}

Exceptions_attribute {
  u2 attribute_name_index;
  u4 attribute_length;
  u2 number_of_exceptions;
  u2 exception_index_table[number_of_exceptions];
}

LineNUmberTable_attribute {
  u2 attribute_name_index;
  u4 attribute_length;
  u2 line_number_table_length;
  {
    u2 start_pc;
    u2 line_number;
  } line_number_table[line_number_table_length];
}

LocalVariableTable_attribute {
  u2 attribute_name_index;
  u4 attribute_length;
  u2 local_variable_table_length;
  {
    u2 start_pc;
    u2 length;
    u2 name_index;
    u2 descriptor_index;
    u2 index;
  } local_variable_table[local_variable_table_length];
}
```
