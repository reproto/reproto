package test;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonValue;

public enum RootEnum {
    Foo("Foo");

    String value;

    RootEnum(final String value) {
        this.value = value;
    }

    @JsonCreator
    public static RootEnum fromValue(final String value) {
        for (final RootEnum v : values()) {
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
