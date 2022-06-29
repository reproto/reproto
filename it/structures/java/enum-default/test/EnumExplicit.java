package test;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonValue;

/**
 * Explicitly assigned strings
 */
public enum EnumExplicit {
    A("foo"),
    B("bar");

    String value;

    EnumExplicit(final String value) {
        this.value = value;
    }

    @JsonCreator
    public static EnumExplicit fromValue(final String value) {
        for (final EnumExplicit v : values()) {
            if (v.value.equals(value)) {
                return v;
            }
        }

        throw new IllegalArgumentException("value");
    }

    @JsonValue
    public String toValue() {
        return this.value;
    }
}
