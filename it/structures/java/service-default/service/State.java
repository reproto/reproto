package service;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonValue;
import java.util.Objects;

public enum State {
  OPEN("open"),
  CLOSED("closed");

  private final String value;

  private State(
    final String value
  ) {
    Objects.requireNonNull(value, "value");
    this.value = value;
  }

  @JsonCreator
  public static State fromValue(final String value) {
    for (final State v_value : values()) {
      if (v_value.value.equals(value)) {
        return v_value;
      }
    }

    throw new IllegalArgumentException("value");
  }

  @JsonValue
  public String toValue() {
    return this.value;
  }
}
