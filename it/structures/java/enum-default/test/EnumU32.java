package test;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonValue;

public enum EnumU32 {
    Min(0),
    Max(2147483647);

    int value;

    EnumU32(final int value) {
        this.value = value;
    }

    @JsonCreator
    public static EnumU32 fromValue(final int value) {
        for (final EnumU32 v : values()) {
            if (v.value == value) {
                return v;
            }
        }

        throw new IllegalArgumentException("value");
    }

    @JsonValue
    public int toValue() {
        return this.value;
    }
}
