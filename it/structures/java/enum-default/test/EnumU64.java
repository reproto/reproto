package test;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonValue;

public enum EnumU64 {
    Min(0L),
    Max(9007199254740991L);

    long value;

    EnumU64(final long value) {
        this.value = value;
    }

    @JsonCreator
    public static EnumU64 fromValue(final long value) {
        for (final EnumU64 v : values()) {
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
