Source: [The JVM Specification v7](https://docs.oracle.com/javase/specs/jvms/se7/html/index.html).

## ClassFile

```
ClassFile {
  u4 magic;
  u2 minor_version;
  u2 major_version;
  u2 const_pool_count;
  cp_info constant_pool[constant_pool_count = 1];
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
  attribute_info attributes[attribute_count];
}

```

### Access Flags

```
ACC_PUBLIC      0x0001
ACC_FINAL       0x0010
ACC_SUPER       0x0020
ACC_INTERFACE   0x0200
ACC_ABSTRACT    0x0400
ACC_SYNTHETIC   0x1000
ACC_ANNOTATION  0x2000
ACC_ENUM        0x4000

```

## cp_info

```
cp_info {
  u1 tag;
  u1 info[];
}
```

## Constant Pool tags

```
CONSTANT_Class               7
CONSTANT_Fieldref            9
CONSTANT_Methodref           10
CONSTANT_InterfaceMethodref  11
CONSTANT_String              8
CONSTANT_Integer             3
CONSTANT_FLoat               4
CONSTANT_Long                5
CONSTANT_Double              6
CONSTANT_NameAndType         12
CONSTANT_Utf8                1
CONSTANT_MethodHandle        15
CONSTANT_MethodType          16
CONSTANT_InvokeDynamic       18
```

## Constant Pool Entries

```
CONSTANT_Class_info {
  u1 tag; // CONSTANT_Class
  u2 name_index;
}

CONSTANT_Fieldref_info {
  u1 tag; // CONSTANT_Fieldref
  u2 class_index;
  u2 name_and_type_index;
}

CONSTANT_Methodref_info {
  u1 tag; // CONSTANT_Methodref
  u2 class_index; 
  u2 name_and_type_index;
}

CONSTANT_InterfaceMethodref_info {
  u1 tag; // CONSTANT_InterfaceMethodref
  u2 class_index;
  u2 name_and_type_index;
}

CONSTANT_String_info {
  u1 tag; // CONSTANT_String
  u2 string_index;
}

CONSTANT_Integer_info {
  u1 tag; // CONSTANT_Integer
  u4 bytes;
}

CONSTANT_FLoat_info {
  u1 tag; // CONSTANT_FLoat
  u4 bytes;
}

CONSTANT_Long_info {
  u1 tag; // CONSTANT_Long
  u4 high_bytes;
  u4 low_bytes;
}

CONSTANT_Double_info {
  u1 tag; // CONSTANT_Double
  u4 high_bytes;
  u4 low_bytes;
}

CONSTANT NameAndType_info {
  u1 tag;  // CONSTANT_NameAndType
  u2 name_index;
  u2 descriptor_index;
}

CONSTANT_Utf8_info {
  u1 tag; // CONSTANT_Utf8;
  u2 length;
  u1 bytes[length];
}

CONSTANT MethodHandle_info {
  u1 tag; // CONSTANT_MethodHandle
  u1 reference_kind;
  u2 reference_index;
}

CONSTANT_MethodType_info {
  u1 tag; // CONSTANT_MethodType
  u2 descriptor_index;
}

CONSTANT_InvokDynamic_info {
  u1 tag; // CONSTANT_InvokDynamic
  u2 bootstrap_method_attr_index;
  u2 name_and_type_index;
}

```

## Fields

```
field_info {
  u2 access_flags;
  u2 name_index;
  u2 descriptor_index;
  u2 attribute_count;
  attribute_info attributes[attribute_count];
}

```

### Access Flags

```
ACC_PUBLIC      0x0001
ACC_PRIVATE     0x0002
ACC_PROTECTED   0x0004
ACC_STATIC      0x0008
ACC_FINAL       0x0010
ACC_VOLATILE    0x0040
ACC_TRANSIENT   0x0080
ACC_VOLATILE    0x0040
ACC_SYNTHETIC   0x1000
ACC_ENUM        0x4000
```

## Methods

### Representation

```
method_info {
  u2 access_flags;
  u2 name_index;
  u2 descriptor_index;
  u2 attribute_count;
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
ACC_BRIDGE        0x0040
ACC_VARARGS       0x0080
ACC_NATIVE        0x0100
ACC_ABSTRACT      0x0400
ACC_STRUCT        0x0800
ACC_SYNTHETIC     0x1000
```

## Attributes

### Representation

```
attribute_info {
  u2 attribute_name_index;
  u4 attribute_length;
  u1 info[attribute_length];
}
```

### Predefined class file attributes

a
