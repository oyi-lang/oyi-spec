.class public Factorial
.super java/lang/Object

.method public <init>()V
  aload_0
  invokespecial java/lang/Object/<init>()V
  return
.end method

; for static methods, `0` is the first argument, not `this`
.method static factorial(I)I
  .limit stack 2
  .limit locals 3

  iconst_1
  istore_1

loop:
  iload_0
  bipush 0
  if_icmpeq exit
  iload_0
  iload_1
  imul
  istore_1
  iinc 0 -1
  goto loop

exit:
  iload_1
  ireturn
.end method

.method public static main([Ljava/lang/String;)V
  .limit stack 3
  .limit locals 2

  getstatic java/lang/System/out Ljava/io/PrintStream;
  bipush 10
  invokestatic Factorial/factorial(I)I
  invokestatic java/lang/String/valueOf(I)Ljava/lang/String;
  invokevirtual java/io/PrintStream/println(Ljava/lang/String;)V

  return
.end method