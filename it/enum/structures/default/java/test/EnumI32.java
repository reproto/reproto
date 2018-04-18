package test;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonValue;

public enum EnumI32 {
  MIN(-2147483648),
  NEGATIVE_ONE(-1),
  ZERO(0),
  MAX(2147483647);

  private final int value;

  private EnumI32(
    final int value
  ) {
    this.value = value;
  }

  @JsonCreator
  public static EnumI32 fromValue(final int value) {
    for (final EnumI32 v_value : values()) {
      if (v_value.value == value) {
        return v_value;
      }
    }

    throw new IllegalArgumentException("value");
  }

  @JsonValue
  public int toValue() {
    return this.value;
  }
}
