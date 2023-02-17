.class public AddNums
.super java/lang/Object

.method public <init>()V
  aload_0
  invokespecial java/lang/Object/<init>()V
  return
.end method

.method public static main([Ljava/lang/String;)V
  .limit stack 3
  .limit locals 8

  getstatic java/lang/System/out Ljava/io/PrintStream;
  astore_1

  new java/util/Scanner
  dup
  getstatic java/lang/System/in Ljava/io/InputStream;
  invokespecial java/util/Scanner/<init>(Ljava/io/InputStream;)V
  astore_2

  jsr readNum
  istore_3

  jsr readNum
  istore 4

  jsr addNums
  jsr printSum

  aload_2
  invokevirtual java/util/Scanner/close()V

  return

readNum:
  astore 5

  aload_2
  invokevirtual java/util/Scanner/nextInt()I
  ret 5

addNums:
  astore 6

  iload_3
  iload 4
  iadd 
  ret 6

printSum:
  astore 7

  aload_1
  swap
  invokestatic java/lang/String/valueOf(I)Ljava/lang/String;
  invokevirtual java/io/PrintStream/println(Ljava/lang/String;)V
  ret 7
.end method