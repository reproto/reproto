package test;

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
  public Tuple() {
  }

  @Override
  public int hashCode() {
    int result = 1;
    return result;
  }

  @Override
  public boolean equals(final Object other) {
    if (other == null) {
      return false;
    }

    if (!(other instanceof Tuple)) {
      return false;
    }

    @SuppressWarnings("unchecked")
    final Tuple o = (Tuple) other;

    return true;
  }

  @Override
  public String toString() {
    final StringBuilder b = new StringBuilder();

    b.append("Tuple");
    b.append("(");
    b.append(")");

    return b.toString();
  }

  public void tupleMethod() {
  }

  public static class Serializer extends JsonSerializer<Tuple> {
    @Override
    public void serialize(final Tuple value, final JsonGenerator jgen, final SerializerProvider provider) throws IOException {
      jgen.writeStartArray();
      jgen.writeEndArray();
    }
  }

  public static class Deserializer extends JsonDeserializer<Tuple> {
    @Override
    public Tuple deserialize(final JsonParser parser, final DeserializationContext ctxt) throws IOException {
      if (parser.getCurrentToken() != JsonToken.START_ARRAY) {
        throw ctxt.wrongTokenException(parser, JsonToken.START_ARRAY, null);
      }

      if (parser.nextToken() != JsonToken.END_ARRAY) {
        throw ctxt.wrongTokenException(parser, JsonToken.END_ARRAY, null);
      }

      return new Tuple();
    }
  }
}
