package lower_camel;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonProperty;
import java.util.Objects;
import java.util.Optional;

public class Value {
  @JsonProperty("fooBar")
  private final String fooBar;

  @JsonCreator
  public Value(
    @JsonProperty("fooBar") final String fooBar
  ) {
    Objects.requireNonNull(fooBar, "fooBar");
    this.fooBar = fooBar;
  }

  @JsonProperty("fooBar")
  public String getFooBar() {
    return this.fooBar;
  }

  @Override
  public int hashCode() {
    int result = 1;
    result = result * 31 + this.fooBar.hashCode();
    return result;
  }

  @Override
  public boolean equals(final Object other) {
    if (other == null) {
      return false;
    }

    if (!(other instanceof Value)) {
      return false;
    }

    @SuppressWarnings("unchecked")
    final Value o = (Value) other;

    if (!this.fooBar.equals(o.fooBar)) {
      return false;
    }

    return true;
  }

  @Override
  public String toString() {
    final StringBuilder b = new StringBuilder();

    b.append("Value");
    b.append("(");
    b.append("fooBar=");
    b.append(this.fooBar.toString());
    b.append(")");

    return b.toString();
  }

  public static class Builder {
    private Optional<String> fooBar = Optional.empty();

    public Builder fooBar(final String fooBar) {
      this.fooBar = Optional.of(fooBar);
      return this;
    }

    public Value build() {
      final String fooBar = this.fooBar.orElseThrow(() -> new RuntimeException("fooBar: is required"));

      return new Value(fooBar);
    }
  }
}
