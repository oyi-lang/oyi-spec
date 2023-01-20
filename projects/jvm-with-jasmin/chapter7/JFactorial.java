public class JFactorial {
  public static void main(String[] args) {
    System.out.println(java.lang.String.valueOf(JFactorial.factorial(10)));
  }

  static int factorial(int n) {
    int res = 1;

    while (true) {
      if (n == 0) {
        return res;
      }

      res *= n;
      n--;
    }
  }
}