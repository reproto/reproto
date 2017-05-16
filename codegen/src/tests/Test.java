package se.tedro;

import com.fasterxml.jackson.annotation.JsonCreator;
import java.util.List;

public class Test {
  private final List<String> values;

  @JsonCreator
  public Test(final List<String> values) {
    this.values = values;
  }

  public List<String> getValues() {
    return this.values;
  }
}
