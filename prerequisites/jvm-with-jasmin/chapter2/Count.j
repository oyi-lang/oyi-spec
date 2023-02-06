.class public Count
.super java/lang/Object

.method public <init>()V
  aload_0
  invokespecial java/lang/Object/<init>()V
  return
.end method

.method public static main([Ljava/lang/String;)V
  .limit stack 3
  .limit locals 2

  iconst_0
  istore_1

loop:
  iload_1
  bipush 10
  if_icmpge exit
  getstatic java/lang/System/out Ljava/io/PrintStream;
  iload_1
  invokestatic java/lang/String/valueOf(I)Ljava/lang/String;
  invokevirtual java/io/PrintStream/println(Ljava/lang/String;)V
  iinc 1 1
  goto loop
exit:
  return
.end method