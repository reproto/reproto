package test;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonValue;

public enum EnumU32 {
  MIN(0),
  MAX(2147483647);

  private final int value;

  private EnumU32(
    final int value
  ) {
    this.value = value;
  }

  @JsonCreator
  public static EnumU32 fromValue(final int value) {
    for (final EnumU32 v_value : values()) {
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
