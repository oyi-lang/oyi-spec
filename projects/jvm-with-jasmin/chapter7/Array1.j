.class public Array1
.super java/lang/Object

.method public <init>()V
  aload_0
  invokespecial java/lang/Object/<init>()V
  return
.end method

.method public static main([Ljava/lang/String;)V
  .limit stack 5
  .limit locals 5

  ; int[] a = new int[5]
  iconst_5
  newarray int
  astore_1

  ; a[0] = 100
  aload_1
  bipush 0
  bipush 100
  iastore 

  ; a[1] = 200
  aload_1
  bipush 1
  ldc 200
  iastore 

  ; print `a`
  iconst_0
  istore_2
  iconst_5
  istore_3

loop:
  getstatic java/lang/System/out Ljava/io/PrintStream;
  aload_1
  iload_2
  iload_3
  if_icmpeq exit
  iload_2
  iaload
  invokestatic java/lang/String/valueOf(I)Ljava/lang/String;
  invokevirtual java/io/PrintStream/println(Ljava/lang/String;)V
  iinc 2 1
  goto loop

exit:
  return
.end method