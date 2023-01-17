## BASIC interpreter in OCaml

Support for a very restricted subset of the BASIC language.


## Features

  * `PRINT expression` - prints the value of `expression` post evaluation.

  * `INPUT variable` - prints a prompt, reads in an integer from standard input, and assigns it to `variable`.

  * `LET variable = expression` - assigns the result of the evaluation of `expression` to `variable`.

  * `IF expression THEN line` - if the evaluation of `expression` is `true` then continue execution as line number `line`.

  * `GOTO line`

  * `REM single line comment`


## EBNF grammar

```
  UNARY_OP ::= + | - | !

  BINARY_OP ::= + | - | * | / | % | = | <> | < | <= | > | >= | & | '|'

  EXPRESSION ::= 





```

Source: Derivative work based on ["Developing Applications with Object Caml", Chapter 6](https://caml.inria.fr/pub/docs/oreilly-book/html/book-ora058.html).


## Build and Run

```
  $ dune build && dune exec basic

```