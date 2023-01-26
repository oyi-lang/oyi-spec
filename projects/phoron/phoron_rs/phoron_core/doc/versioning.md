## JVM Versioning Support

The basic idea for `phoron_rs` vis-a-vis vesioning is threfold:

  - The base support will be for JVM version 1.0.2 (major number 45, minor number 3). [Phase 1][ ]
  - For the `Deserializer`, serialise the specified version (using the major and minor numbers), and perform checks and error-handling accordingly. [Phase 2] [ ]
  - For the `Serializer`, take in the version and generate class files according to the feature set supported in that version. Performa checks and  [Phase 3] [ ]
  error-handling accordingly.

### JVM Version 1.0.2

### JVM version 19

