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
import java.util.Objects;

@JsonSerialize(using = Tuple2.Serializer.class)
@JsonDeserialize(using = Tuple2.Deserializer.class)
public class Tuple2 {
  private final String a;
  private final Other b;

  public Tuple2(
    final String a,
    final Other b
  ) {
    Objects.requireNonNull(a, "a");
    this.a = a;
    Objects.requireNonNull(b, "b");
    this.b = b;
  }

  @Override
  public int hashCode() {
    int result = 1;
    result = result * 31 + this.a.hashCode();
    result = result * 31 + this.b.hashCode();
    return result;
  }

  @Override
  public boolean equals(final Object other) {
    if (other == null) {
      return false;
    }

    if (!(other instanceof Tuple2)) {
      return false;
    }

    @SuppressWarnings("unchecked")
    final Tuple2 o = (Tuple2) other;

    if (!this.a.equals(o.a)) {
      return false;
    }

    if (!this.b.equals(o.b)) {
      return false;
    }

    return true;
  }

  @Override
  public String toString() {
    final StringBuilder b = new StringBuilder();

    b.append("Tuple2");
    b.append("(");
    b.append("a=");
    b.append(this.a.toString());
    b.append(", ");
    b.append("b=");
    b.append(this.b.toString());
    b.append(")");

    return b.toString();
  }

  public String getA() {
    return this.a;
  }

  public Other getB() {
    return this.b;
  }

  public static class Serializer extends JsonSerializer<Tuple2> {
    @Override
    public void serialize(final Tuple2 value, final JsonGenerator jgen, final SerializerProvider provider) throws IOException {
      jgen.writeStartArray();
      jgen.writeString(value.a);
      jgen.writeObject(value.b);
      jgen.writeEndArray();
    }
  }

  public static class Deserializer extends JsonDeserializer<Tuple2> {
    @Override
    public Tuple2 deserialize(final JsonParser parser, final DeserializationContext ctxt) throws IOException {
      if (parser.getCurrentToken() != JsonToken.START_ARRAY) {
        throw ctxt.wrongTokenException(parser, JsonToken.START_ARRAY, null);
      }

      if (parser.nextToken() != JsonToken.VALUE_STRING) {
        throw ctxt.wrongTokenException(parser, JsonToken.VALUE_STRING, null);
      }

      final String v_a = parser.getText();

      parser.nextToken();

      final Other v_b = parser.readValueAs(Other.class);

      if (parser.nextToken() != JsonToken.END_ARRAY) {
        throw ctxt.wrongTokenException(parser, JsonToken.END_ARRAY, null);
      }

      return new Tuple2(v_a, v_b);
    }
  }
}
