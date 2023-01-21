import java.util.function.Function;

public class InvokedDynamicDemo {
  public static void main(String[] args) {
    Function<String, Integer> len = s -> s.length();
    System.out.println(len.apply("Bob"));
  }
}