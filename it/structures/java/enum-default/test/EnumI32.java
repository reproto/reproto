package test;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonValue;

public enum EnumI32 {
    Min(-2147483648),
    NegativeOne(-1),
    Zero(0),
    Max(2147483647);

    int value;

    EnumI32(final int value) {
        this.value = value;
    }

    @JsonCreator
    public static EnumI32 fromValue(final int value) {
        for (final EnumI32 v : values()) {
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
