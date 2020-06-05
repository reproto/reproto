package service;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonValue;

public enum ErrorCode {
  USER(400),
  SERVER(500);

  private final int value;

  private ErrorCode(
    final int value
  ) {
    this.value = value;
  }

  @JsonCreator
  public static ErrorCode fromValue(final int value) {
    for (final ErrorCode v_value : values()) {
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
