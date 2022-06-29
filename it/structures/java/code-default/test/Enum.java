package test;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonValue;

public enum Enum {
    Variant("Variant");

    String value;

    Enum(final String value) {
        this.value = value;
    }

    @JsonCreator
    public static Enum fromValue(final String value) {
        for (final Enum v : values()) {
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
