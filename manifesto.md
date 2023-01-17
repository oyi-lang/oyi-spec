Oyi is intended as a modern ML, stylistically and spiritually in the vein of OCaml, for the JVM. In a nutshell, F#-like on the JVM. 

The JVM is the *first tier* for Oyi. Other platforms (native, LLVM, C et al) are *second tier*. 

Note: Not everything in this document has to be completed before starting the Oyi project itself, but the intent is to document not only the essentials for Oyi, but also for
a continuing edificational journey into the world of Compilers (PLT covered elsewhere).

## Projects

  * [BASIC interpreter (OCaml)](projects/basic/basic_ocaml) [x]

  * [BASIC interpreter (Rust)](projects/basic/basic_rs) [x]

  * [JVM internals using Jasmin](projects/jvm-with-jasmin) [x]

  * [Lisp interpreter (Ocaml)](projects/lwhlisp/lwhlisp_ocaml) [ ]

  * [Lisp interpreter (Rust)](projects/lwhlisp/lwhlisp_ocaml) [ ]

  * [Triangle compiler (Ocaml) and Bytecode VM (Rust)](https://www.dcs.gla.ac.uk/~daw/books/PLPJ/) [ ]

  * [MinCaml (Ocaml)](projects/mincaml/mincaml_ocaml) [  ]

  * [MinCaml (Rust)](projects/mincaml/mincaml_ocaml) [  ]

  * [CLEmitter library (OCaml)] [ ]

  * [CLEmitter library (Rust)] [ ]

  * [J-- (OCaml)](projects/jminusminus/jminusminus_ocaml) [ ]

  * [J-- (Rust)](projects/jminusminus/jminusminus_rs) [ ]

  * [Example Compiler (Ocaml)](projects/example_compiler/example_compiler_ocaml) [ ]

  * [Example Compiler (Rust)](projects/example_compiler/example_compiler_rs) [ ]

  * [Kaleidoscope (OCaml)](projects/kaleidoscope/kaleidoscope_ocaml) [ ]

  * [Kaleidoscope (Rust)](projects/kaleidoscope/kaleidoscope_rs) [ ]

  
## Resources

### Tooling

  * [CF grammar checker](http://smlweb.cpsc.ucalgary.ca/start.html)

  * [peg](https://www.piumarta.com/software/peg/)

### Type Systems

  * [TAPL](https://www.cis.upenn.edu/~bcpierce/tapl/) [ ]
  
  * [ATAPL](https://www.cis.upenn.edu/~bcpierce/attapl/) [ ]

  * [SRTPs in F#](https://learn.microsoft.com/en-us/dotnet/fsharp/language-reference/generics/statically-resolved-type-parameters)

  * [Active Patterns in F#](https://dl.acm.org/doi/10.1145/1291151.1291159)

### OCaml and F#/ML specific

  * [The Definition of Standard ML - SML 90](https://github.com/SMLFamily/sml90) [Milner et al] [ ]

  * [The Definition of Standard ML - SML 97](https://github.com/SMLFamily/sml97) [Milner et al] [ ]

  * [The ML Family](https://smlfamily.github.io/) [ ]

  * [Caml papers](https://caml.inria.fr/about/papers.en.html) [ ]

  * [The Early History of F#](https://dl.acm.org/doi/pdf/10.1145/3386325) [x]

### JVM specific

  * [Java Virtual Machine](https://archive.org/details/javavirtualmachi0000meye) [Troy Downing and Jon Meyer] [x]

  * [The JVM specification](https://docs.oracle.com/javase/specs/jls/se19/html/index.html) [ ]

  * [Tail Call Elimination of the Java Virtual Machine](https://www.researchgate.net/publication/222659379_Tail_Call_Elimination_on_the_Java_Virtual_Machine/fulltext/0e5fab00f0c41c4932e2ff21/Tail-Call-Elimination-on-the-Java-Virtual-Machine.pdf) [Schinz and Odersky] [ ]

  * [Memory Efficient Tail calls in the JVM with Imperative Functional Objects](https://i.cs.hku.hk/~bruno/papers/APLAS2015.pdf) [Tauber] [ ]

  * [Tail Call Elimination and Data Representation for Functional Languages on the Java Virtual Machine](https://flix.dev/paper/cc2018.pdf) [Madsen] [ ]