## BASIC interpreter in OCaml

Support for a very restricted subset of the BASIC language.


## Features

  * `PRINT expression` - prints the value of `expression` post evaluation.

  * `INPUT variable` - prints a prompt, reads in an integer from standard input, and assigns it to `variable`.

  * `LET variable = expression` - assigns the result of the evaluation of `expression` to `variable`.

  * `IF expression THEN line` - if the evaluation of `expression` is `true` then continue execution as line number `line`.

  * `GOTO line` - continue execution at the given line number.

  * `REM comment` - a single-line comment - ignore the whole line.


## EBNF grammar

```
  UNARY_OP ::= + | - | !

  BINARY_OP ::= + | - | * | / | % | = | <> | < | <= | > | >= | & | '|'

  EXPRESSION ::= integer
              | variable
              | "string"
              | UNARY_OP EXPRESSION
              | EXPRESSION BINARY_OP EXPRESSION
              | '(' EXPRESSION ')''

  COMMAND ::= REM string
            | GOTO integer
            | INPUT variable
            | PRINT EXPRESSION
            | LET variable = EXPRESSION
            | IF EXPRESSION THEN integer

  LINE ::= integer COMMAND

  PROGRAM ::= LINE | LINE PROGRAM

  PHRASE ::= LINE | RUN | LIST | END

```

Source: Derivative work based on ["Developing Applications with Object Caml", Chapter 6](https://caml.inria.fr/pub/docs/oreilly-book/html/book-ora058.html).


## Build and Run

```
  $ dune build && dune exec basic

```