package test;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonValue;
import java.util.Objects;

public enum EnumLongNames {
  FOO_BAR("FooBar"),
  BAZ("Baz");

  private final String value;

  private EnumLongNames(
    final String value
  ) {
    Objects.requireNonNull(value, "value");
    this.value = value;
  }

  @JsonCreator
  public static EnumLongNames fromValue(final String value) {
    for (final EnumLongNames v_value : values()) {
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
