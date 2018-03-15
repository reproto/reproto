package test;

import com.fasterxml.jackson.annotation.JsonProperty;
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

@JsonSerialize(using = Tuple1.Serializer.class)
@JsonDeserialize(using = Tuple1.Deserializer.class)
public class Tuple1 {
  private final String a;
  private final long b;

  public Tuple1(
    final String a,
    final long b
  ) {
    Objects.requireNonNull(a, "a");
    this.a = a;
    this.b = b;
  }

  @Override
  public int hashCode() {
    int result = 1;
    result = result * 31 + this.a.hashCode();
    result = result * 31 + Long.hashCode(this.b);
    return result;
  }

  @Override
  public boolean equals(final Object other) {
    if (other == null) {
      return false;
    }

    if (!(other instanceof Tuple1)) {
      return false;
    }

    @SuppressWarnings("unchecked")
    final Tuple1 o = (Tuple1) other;

    if (!this.a.equals(o.a)) {
      return false;
    }

    if (this.b != o.b) {
      return false;
    }

    return true;
  }

  @Override
  public String toString() {
    final StringBuilder b = new StringBuilder();

    b.append("Tuple1");
    b.append("(");
    b.append("a=");
    b.append(this.a.toString());
    b.append(", ");
    b.append("b=");
    b.append(Long.toString(this.b));
    b.append(")");

    return b.toString();
  }

  @JsonProperty("a")
  public String getA() {
    return this.a;
  }

  @JsonProperty("b")
  public long getB() {
    return this.b;
  }

  public static class Serializer extends JsonSerializer<Tuple1> {
    @Override
    public void serialize(final Tuple1 value, final JsonGenerator jgen, final SerializerProvider provider) throws IOException {
      jgen.writeStartArray();
      jgen.writeString(value.a);
      jgen.writeNumber(value.b);
      jgen.writeEndArray();
    }
  }

  public static class Deserializer extends JsonDeserializer<Tuple1> {
    @Override
    public Tuple1 deserialize(final JsonParser parser, final DeserializationContext ctxt) throws IOException {
      if (parser.getCurrentToken() != JsonToken.START_ARRAY) {
        throw ctxt.wrongTokenException(parser, JsonToken.START_ARRAY, null);
      }

      if (parser.nextToken() != JsonToken.VALUE_STRING) {
        throw ctxt.wrongTokenException(parser, JsonToken.VALUE_STRING, null);
      }

      final String v_a = parser.getText();

      if (!parser.nextToken().isNumeric()) {
        throw ctxt.wrongTokenException(parser, JsonToken.VALUE_NUMBER_INT, null);
      }

      final long v_b = parser.getLongValue();

      if (parser.nextToken() != JsonToken.END_ARRAY) {
        throw ctxt.wrongTokenException(parser, JsonToken.END_ARRAY, null);
      }

      return new Tuple1(v_a, v_b);
    }
  }
}
