package test;

import com.fasterxml.jackson.annotation.JsonCreator;
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

@JsonSerialize(using = Tuple.Serializer.class)
@JsonDeserialize(using = Tuple.Deserializer.class)
public class Tuple {

    @JsonCreator
    public Tuple() {}

    @Override
    public String toString() {
        return "Tuple()";
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

        if (!(other_ instanceof Tuple)) {
            return false;
        }

        @SuppressWarnings("unchecked")
        final Tuple o_ = (Tuple)other_;

        return true;
    }

    public static class Builder {

        private Builder() {}

        public Tuple build() {

            return new Tuple();
        }
    }

    /**
     * Construct a new builder.
     */
    public static Builder builder() {
        return new Builder();
    }

    public static class Serializer extends JsonSerializer<Tuple> {
        @Override
        public void serialize(final Tuple value_, final JsonGenerator gen_, final SerializerProvider provider_) throws IOException {
            gen_.writeStartArray();

            gen_.writeEndArray();
        }
    }

    public static class Deserializer extends JsonDeserializer<Tuple> {
        @Override
        public Tuple deserialize(final JsonParser parser_, final DeserializationContext ctxt_) throws IOException {
            if (parser_.getCurrentToken() != JsonToken.START_ARRAY) {
                throw ctxt_.wrongTokenException(parser_, JsonToken.START_ARRAY, null);
            }

            if (parser_.nextToken() != JsonToken.END_ARRAY) {
                throw ctxt_.wrongTokenException(parser_, JsonToken.END_ARRAY, null);
            }

            return new Tuple();
        }
    }

    public void tupleMethod() {
    }
}
