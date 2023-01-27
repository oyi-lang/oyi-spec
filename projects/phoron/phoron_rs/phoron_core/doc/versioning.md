## JVM Versioning Support

The basic idea for `phoron_rs` vis-a-vis vesioning is threfold:

  - Base support will be for JVM version 1.0.2 (major number 45, minor number 3). [Phase 1][ ]
  - For the `Deserializer`, deserialise the specified version (using the major and minor numbers), and perform checks and error-handling accordingly. [Phase 2] [ ]
  - For the `Serializer`, take in the version (default v1.0.2) and generate class files according to the feature set supported in that version. Perform checks and error-handling accordingly. [Phase 3] [ ]

### JVM Version 1.0.2

### JVM version 19

