.class public ThrowRuntimeException
.super java/lang/Object

.method public <init>()V
  aload_0
  invokespecial java/lang/Object/<init>()V
  return
.end method

.method public static main([Ljava/lang/String;)V
  .limit stack 3
  .limit locals 1

  new java/lang/RuntimeException
  dup
  ldc "Oops!"
  invokespecial java/lang/RuntimeException/<init>(Ljava/lang/String;)V
  athrow

  return
.end method