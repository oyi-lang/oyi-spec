type unary_op = UPLUS | UMINUS | NOT

type binary_op =
  | PLUS
  | MINUS
  | MULT
  | DIV
  | MOD
  | EQ
  | NOTEQ
  | LESS
  | LESSEQ
  | GREATER
  | GREATEEQ
  | AND
  | OR

type expression =
  | ExpInt of int
  | ExpStr of string
  | ExpUnary of unary_op * expression
  | ExpBinary of expression * binary_op * expression

type command =
  | Rem of string
  | Goto of int
  | Input of string
  | Print of expression
  | If of expression * int
  | Let of string * expression

type line = { line_number : int; cmd : command }
type program = line list
type phrase = Line of line | Run | List | End
