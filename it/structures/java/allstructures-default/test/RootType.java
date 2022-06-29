package test;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonSubTypes;
import com.fasterxml.jackson.annotation.JsonTypeInfo;
import com.fasterxml.jackson.annotation.JsonValue;
import com.fasterxml.jackson.core.JsonGenerator;
import com.fasterxml.jackson.core.JsonParser;
import com.fasterxml.jackson.core.JsonToken;
import com.fasterxml.jackson.databind.DeserializationContext;
import com.fasterxml.jackson.databind.JsonDeserializer;
import com.fasterxml.jackson.databind.JsonSerializer;
import com.fasterxml.jackson.databind.SerializerProvider;
import com.fasterxml.jackson.databind.annotation.JsonDeserialize;
import com.fasterxml.jackson.databind.annotation.JsonSerialize;
import java.io.IOException;

public class RootType {

    @JsonCreator
    public RootType() {}

    @Override
    public String toString() {
        return "RootType()";
    }

    @Override
    public int hashCode() {
        int result = 1;
        final StringBuilder b = new StringBuilder();
        return result;
    }

    @Override
    public boolean equals(final Object other_) {
        if (other_ == null) {
            return false;
        }

        if (!(other_ instanceof RootType)) {
            return false;
        }

        @SuppressWarnings("unchecked")
        final RootType o_ = (RootType)other_;

        return true;
    }

    public static class Builder {

        private Builder() {}

        public RootType build() {

            return new RootType();
        }
    }

    /**
     * Construct a new builder.
     */
    public static Builder builder() {
        return new Builder();
    }

    public static class NestedType {

        @JsonCreator
        public NestedType() {}

        @Override
        public String toString() {
            return "NestedType()";
        }

        @Override
        public int hashCode() {
            int result = 1;
            final StringBuilder b = new StringBuilder();
            return result;
        }

        @Override
        public boolean equals(final Object other_) {
            if (other_ == null) {
                return false;
            }

            if (!(other_ instanceof NestedType)) {
                return false;
            }

            @SuppressWarnings("unchecked")
            final NestedType o_ = (NestedType)other_;

            return true;
        }

        public static class Builder {

            private Builder() {}

            public NestedType build() {

                return new NestedType();
            }
        }

        /**
         * Construct a new builder.
         */
        public static Builder builder() {
            return new Builder();
        }
    }

    @JsonTypeInfo(use=JsonTypeInfo.Id.NAME, include=JsonTypeInfo.As.PROPERTY, property="type")
    @JsonSubTypes({
        @JsonSubTypes.Type(name="Foo", value=NestedInterface.Foo.class),
    })
    public static interface NestedInterface {

        public static class Foo implements NestedInterface {

            @JsonCreator
            public Foo() {}

            @Override
            public String toString() {
                return "Foo()";
            }

            @Override
            public int hashCode() {
                int result = 1;
                final StringBuilder b = new StringBuilder();
                return result;
            }

            @Override
            public boolean equals(final Object other_) {
                if (other_ == null) {
                    return false;
                }

                if (!(other_ instanceof Foo)) {
                    return false;
                }

                @SuppressWarnings("unchecked")
                final Foo o_ = (Foo)other_;

                return true;
            }

            public static class Builder {

                private Builder() {}

                public Foo build() {

                    return new Foo();
                }
            }

            /**
             * Construct a new builder.
             */
            public static Builder builder() {
                return new Builder();
            }

            public static class Nested {

                @JsonCreator
                public Nested() {}

                @Override
                public String toString() {
                    return "Nested()";
                }

                @Override
                public int hashCode() {
                    int result = 1;
                    final StringBuilder b = new StringBuilder();
                    return result;
                }

                @Override
                public boolean equals(final Object other_) {
                    if (other_ == null) {
                        return false;
                    }

                    if (!(other_ instanceof Nested)) {
                        return false;
                    }

                    @SuppressWarnings("unchecked")
                    final Nested o_ = (Nested)other_;

                    return true;
                }

                public static class Builder {

                    private Builder() {}

                    public Nested build() {

                        return new Nested();
                    }
                }

                /**
                 * Construct a new builder.
                 */
                public static Builder builder() {
                    return new Builder();
                }
            }
        };
    }

    public static enum NestedEnum {
        Foo("Foo");

        String value;

        NestedEnum(final String value) {
            this.value = value;
        }

        @JsonCreator
        public static NestedEnum fromValue(final String value) {
            for (final NestedEnum v : values()) {
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

    @JsonSerialize(using = NestedTuple.Serializer.class)
    @JsonDeserialize(using = NestedTuple.Deserializer.class)
    public static class NestedTuple {

        @JsonCreator
        public NestedTuple() {}

        @Override
        public String toString() {
            return "NestedTuple()";
        }

        @Override
        public int hashCode() {
            int result = 1;
            final StringBuilder b = new StringBuilder();
            return result;
        }

        @Override
        public boolean equals(final Object other_) {
            if (other_ == null) {
                return false;
            }

            if (!(other_ instanceof NestedTuple)) {
                return false;
            }

            @SuppressWarnings("unchecked")
            final NestedTuple o_ = (NestedTuple)other_;

            return true;
        }

        public static class Builder {

            private Builder() {}

            public NestedTuple build() {

                return new NestedTuple();
            }
        }

        /**
         * Construct a new builder.
         */
        public static Builder builder() {
            return new Builder();
        }

        public static class Serializer extends JsonSerializer<NestedTuple> {
            @Override
            public void serialize(final NestedTuple value_, final JsonGenerator gen_, final SerializerProvider provider_) throws IOException {
                gen_.writeStartArray();

                gen_.writeEndArray();
            }
        }

        public static class Deserializer extends JsonDeserializer<NestedTuple> {
            @Override
            public NestedTuple deserialize(final JsonParser parser_, final DeserializationContext ctxt_) throws IOException {
                if (parser_.getCurrentToken() != JsonToken.START_ARRAY) {
                    throw ctxt_.wrongTokenException(parser_, JsonToken.START_ARRAY, null);
                }

                if (parser_.nextToken() != JsonToken.END_ARRAY) {
                    throw ctxt_.wrongTokenException(parser_, JsonToken.END_ARRAY, null);
                }

                return new NestedTuple();
            }
        }

        public static class Nested {

            @JsonCreator
            public Nested() {}

            @Override
            public String toString() {
                return "Nested()";
            }

            @Override
            public int hashCode() {
                int result = 1;
                final StringBuilder b = new StringBuilder();
                return result;
            }

            @Override
            public boolean equals(final Object other_) {
                if (other_ == null) {
                    return false;
                }

                if (!(other_ instanceof Nested)) {
                    return false;
                }

                @SuppressWarnings("unchecked")
                final Nested o_ = (Nested)other_;

                return true;
            }

            public static class Builder {

                private Builder() {}

                public Nested build() {

                    return new Nested();
                }
            }

            /**
             * Construct a new builder.
             */
            public static Builder builder() {
                return new Builder();
            }
        }
    }
}
