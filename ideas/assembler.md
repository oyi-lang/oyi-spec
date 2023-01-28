1. Idea: Create [Jasmin](https://jasmin.sourceforge.net/) (or Jasmin-like) ports for OCaml and Rust, and target this assembler format? In lieu of generating the bytecode directly, that is.
  
Advantages:
  - more robust and consistent
  - easier to debug and maintain
  - modular, can be decoupled and replaced

Disadvantages:
  - extra steps
  - more complexity
  - possibly less performance

2. Look into using JVM Attributes for specific features support in the Oyi Runtime.
