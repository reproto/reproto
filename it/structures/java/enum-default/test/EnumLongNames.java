package test;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonValue;

/**
 * Variants with long names.
 */
public enum EnumLongNames {
    FooBar("FooBar"),
    Baz("Baz");

    String value;

    EnumLongNames(final String value) {
        this.value = value;
    }

    @JsonCreator
    public static EnumLongNames fromValue(final String value) {
        for (final EnumLongNames v : values()) {
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
