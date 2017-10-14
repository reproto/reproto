package test;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonValue;
import java.util.Objects;

public enum EnumImplicit {
  A("A"),
  B("B");

  private final String value;

  private EnumImplicit(
    final String value
  ) {
    Objects.requireNonNull(value, "value");
    this.value = value;
  }

  @JsonCreator
  public static EnumImplicit fromValue(final String value) {
    for (final EnumImplicit v_value : values()) {
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
