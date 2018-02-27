package test;

import java.util.Objects;

public enum Enum {
  VARIANT("Variant");

  private final String value;

  private Enum(
    final String value
  ) {
    Objects.requireNonNull(value, "value");
    this.value = value;
  }

  public static Enum fromValue(final String value) {
    for (final Enum v_value : values()) {
      if (v_value.value.equals(value)) {
        return v_value;
      }
    }

    throw new IllegalArgumentException("value");
  }

  public String toValue() {
    return this.value;
  }

  public void enumMethod() {
  }
}
