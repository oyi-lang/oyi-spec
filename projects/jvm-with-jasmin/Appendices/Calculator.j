.class public Calculator
.super java/lang/Object

.method public <init>()V
  aload_0
  invokespecial java/lang/Object/<init>()V
  return
.end method

.method public add(II)I
  .limit stack 2
  .limit locals 3

  iload_1
  iload_2
  iadd
  ireturn
.end method

.method public sub(II)I
  .limit stack 2
  .limit locals 3

  iload_1
  iload_2
  isub
  ireturn
.end method

.method public mul(II)I
  .limit stack 2
  .limit locals 3

  iload_1
  iload_2
  imul
  ireturn
.end method

.method public div(II)I
  .limit stack 2
  .limit locals 3

  .catch java/lang/ArithmeticException from Start to End using ZeroDivHandler

Start:
  iload_1
  iload_2
  idiv
  ireturn
End:

ZeroDivHandler:
  pop
  iconst_0
  ireturn
.end method

.method public static main([Ljava/lang/String;)V
  .limit stack 4
  .limit locals 7

  jsr ReadNum
  istore_1

  jsr ReadNum
  istore_2

  new Calculator
  dup
  invokespecial Calculator/<init>()V
  astore_3

  aload_3
  iload_1
  iload_2
  invokevirtual Calculator/add(II)I
  jsr PrintNum

  aload_3
  iload_1
  iload_2
  invokevirtual Calculator/sub(II)I
  jsr PrintNum

  aload_3
  iload_1
  iload_2
  invokevirtual Calculator/mul(II)I
  jsr PrintNum

  aload_3
  iload_1
  iload_2
  invokevirtual Calculator/div(II)I
  jsr PrintNum

exit:

  return

ReadNum:
  astore 4
  new java/util/Scanner
  dup
  getstatic java/lang/System/in Ljava/io/InputStream;
  invokespecial java/util/Scanner/<init>(Ljava/io/InputStream;)V
  invokevirtual java/util/Scanner/nextInt()I

  ret 4

PrintNum:
  astore 5
  getstatic java/lang/System/out Ljava/io/PrintStream;
  swap
  invokestatic java/lang/String/valueOf(I)Ljava/lang/String;
  invokevirtual java/io/PrintStream/println(Ljava/lang/String;)V
  ret 5
.end method
