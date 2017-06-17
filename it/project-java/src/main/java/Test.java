import com.fasterxml.jackson.databind.ObjectMapper;
import com.fasterxml.jackson.datatype.jdk8.Jdk8Module;
import java.io.BufferedReader;
import java.io.InputStreamReader;
import test.Entry;

public class Test {
  public static void main(String[] argv) throws Exception {
    final ObjectMapper m = new ObjectMapper();
    m.registerModule(new Jdk8Module());

    final BufferedReader reader = new BufferedReader(new InputStreamReader(System.in));

    while (true) {
      final String line = reader.readLine();

      if (line == null) {
        break;
      }

      final Entry entry = m.readValue(line, Entry.class);
      System.out.println(entry);
    }
  }
}
