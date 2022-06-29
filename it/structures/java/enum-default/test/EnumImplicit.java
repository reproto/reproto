package test;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonValue;

/**
 * Implicit naming depending on the variant
 */
public enum EnumImplicit {
    A("A"),
    B("B");

    String value;

    EnumImplicit(final String value) {
        this.value = value;
    }

    @JsonCreator
    public static EnumImplicit fromValue(final String value) {
        for (final EnumImplicit v : values()) {
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
