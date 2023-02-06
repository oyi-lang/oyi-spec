.class public JsrDemo
.super java/lang/Object

.method public <init>()V
  aload_0
  invokespecial java/lang/Object/<init>()V
  return
.end method

.method public static main([Ljava/lang/String;)V
  .limit stack 2
  .limit locals 2

  ldc "Hello"
  jsr printMessage

  ldc "World"
  jsr printMessage

  return

printMessage:
  ; store the return address to `ret` to
  astore_1

  getstatic java/lang/System/out Ljava/io/PrintStream;
  swap
  invokevirtual java/io/PrintStream/println(Ljava/lang/String;)V
  ret 1 
.end method