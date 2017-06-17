import com.fasterxml.jackson.databind.ObjectMapper;
import java.io.BufferedReader;
import java.io.InputStreamReader;
import test.Entry;

public class Test {
  public void main(String[] argv) throws Exception {
    final ObjectMapper m = new ObjectMapper();

    while (true) {
      final BufferedReader reader = new BufferedReader(new InputStreamReader(System.in));
      final String line = reader.readLine();
      final Entry entry = m.readValue(line, Entry.class);
      System.out.println(entry);
    }
  }
}
