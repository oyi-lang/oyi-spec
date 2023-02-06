.class public Catcher
.super java/lang/Object

.method public <init>()V
  aload_0
  invokespecial java/lang/Object/<init>()V
  return
.end method

.method public static main([Ljava/lang/String;)V
  .limit stack 3
  .limit locals 1

  .catch java/lang/Exception from label1 to label2 using Handler

label1:
  new java/lang/Exception
  dup
  invokespecial java/lang/Exception/<init>()V
  athrow
label2:
Handler:
  ; remove the exception off the stack
  pop

  getstatic java/lang/System/out Ljava/io/PrintStream;
  ldc "Exception caught"
  invokevirtual java/io/PrintStream/println(Ljava/lang/String;)V
  return
.end method