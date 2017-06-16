package heroic.v1;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonValue;
import java.util.Objects;

public enum TimeUnit {
  SECONDS("seconds", 1000D),
  MINUTES("minutes", 60000D);

  private final String name;
  private final double number;

  private TimeUnit(
    final String name, final double number
  ) {
    Objects.requireNonNull(name, "name");
    this.name = name;
    this.number = number;
  }

  public String getName() {
    return this.name;
  }

  public double getNumber() {
    return this.number;
  }

  public double toMilliseconds() {
    return this.number;
  }

  @JsonCreator
  public static TimeUnit fromValue(final double number) {
    for (final TimeUnit value : values()) {
      if (value.number == number) {
        return value;
      }
    }

    throw new IllegalArgumentException("number");
  }

  @JsonValue
  public double toValue() {
    return this.number;
  }
}
