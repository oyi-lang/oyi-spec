## The Object Model

The Phoron object model is a simple mapping from the [Classfile Representation](doc/classfile_format.md) into Rust types.

## ClassFile

```
#[repr(packed)]
struct ClassFile<const CP: usize> {
  magic: u32,
  minor_version: u16,
  major_version: u16,
  constant_pool_count: u16,
  constant_pool: [cp_info; CP],
  ...
}
```

