.class public Finally
.super java/lang/Object

.method public <init>()V
  aload_0
  invokespecial java/lang/Object/<init>()V
  return
.end method

.method public test()V
  .limit stack 4
  .limit locals 5

  .catch java/io/FileNotFoundException from Start to End1 using NotFound
  .catch java/io/IOException from Start to End2 using IOE
  .catch java/lang/Exception from Start to Done using Other_Exception

Start:
  new java/io/FileInputStream
  dup
  ldc "myfile"
  invokespecial java/io/FileInputStream/<init>(Ljava/lang/String;)V
  astore_1
End1:
  goto Done

NotFound:
  pop

  getstatic java/lang/System/out Ljava/io/PrintStream;
  ldc "no such file"
  invokevirtual java/io/PrintStream/println(Ljava/lang/String;)V
  goto Done

End2:
IOE:
  pop

  getstatic java/lang/System/out Ljava/io/PrintStream;
  ldc "IO exception"
  invokevirtual java/io/PrintStream/println(Ljava/lang/String;)V
  goto Done

Done:
  jsr FinalSub
  return

Other_Exception:
  astore_2 ; store the exception to rethrow later
  jsr FinalSub
  aload_2
  athrow

FinalSub:
  astore_3

  getstatic java/lang/System/out Ljava/io/PrintStream;
  ldc "done"
  invokevirtual java/io/PrintStream/println(Ljava/lang/String;)V
  ret 3
.end method

.method public static main([Ljava/lang/String;)V
  .limit stack 2
  .limit locals 1

  new Finally
  dup
  invokespecial Finally/<init>()V
  invokevirtual Finally/test()V

  return
.end method