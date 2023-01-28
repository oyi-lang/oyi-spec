# phoron_core

This project provides the low-level functionality of interacting with the JVM, and thus it provides the following high-level features:

  - generating `class` files from the object rrpresentation, and
  - generating object representations from `class` files
  - feature checks against JVM versions

For details on the object representation, see [Object Model](doc/object_model.md) document.

## Build

  ```
    $ cargo build --release
    $ cargo test --release
  ```

## Demo

## Planned Features

  - Pluggable support for custom Attributes.

## LICENCE

See [LICENSE](LICENSE).
