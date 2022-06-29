package test;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonValue;

public enum EnumI64 {
    Min(-9007199254740991L),
    NegativeOne(-1L),
    Zero(0L),
    Max(9007199254740991L);

    long value;

    EnumI64(final long value) {
        this.value = value;
    }

    @JsonCreator
    public static EnumI64 fromValue(final long value) {
        for (final EnumI64 v : values()) {
            if (v.value == value) {
                return v;
            }
        }

        throw new IllegalArgumentException("value");
    }

    @JsonValue
    public long toValue() {
        return this.value;
    }
}
