package test;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonValue;

public enum EnumU64 {
  MIN(0L),
  MAX(9007199254740991L);

  private final long value;

  private EnumU64(
    final long value
  ) {
    this.value = value;
  }

  @JsonCreator
  public static EnumU64 fromValue(final long value) {
    for (final EnumU64 v_value : values()) {
      if (v_value.value == value) {
        return v_value;
      }
    }

    throw new IllegalArgumentException("value");
  }

  @JsonValue
  public long toValue() {
    return this.value;
  }
}
