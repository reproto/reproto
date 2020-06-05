package service;

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

@JsonSerialize(using = Point.Serializer.class)
@JsonDeserialize(using = Point.Deserializer.class)
public class Point {
  /**
   * <pre>
   * When the thing was measured.
   * </pre>
   */
  private final long timestamp;
  /**
   * <pre>
   * The value that was measured.
   * </pre>
   */
  private final double value;

  public Point(
    final long timestamp,
    final double value
  ) {
    this.timestamp = timestamp;
    this.value = value;
  }

  @Override
  public int hashCode() {
    int result = 1;
    result = result * 31 + Long.hashCode(this.timestamp);
    result = result * 31 + Double.hashCode(this.value);
    return result;
  }

  @Override
  public boolean equals(final Object other) {
    if (other == null) {
      return false;
    }

    if (!(other instanceof Point)) {
      return false;
    }

    @SuppressWarnings("unchecked")
    final Point o = (Point) other;

    if (this.timestamp != o.timestamp) {
      return false;
    }

    if (this.value != o.value) {
      return false;
    }

    return true;
  }

  @Override
  public String toString() {
    final StringBuilder b = new StringBuilder();

    b.append("Point");
    b.append("(");
    b.append("timestamp=");
    b.append(Long.toString(this.timestamp));
    b.append(", ");
    b.append("value=");
    b.append(Double.toString(this.value));
    b.append(")");

    return b.toString();
  }

  /**
   * <pre>
   * When the thing was measured.
   * </pre>
   */
  @JsonProperty("timestamp")
  public long getTimestamp() {
    return this.timestamp;
  }

  /**
   * <pre>
   * The value that was measured.
   * </pre>
   */
  @JsonProperty("value")
  public double getValue() {
    return this.value;
  }

  public static class Serializer extends JsonSerializer<Point> {
    @Override
    public void serialize(final Point value, final JsonGenerator jgen, final SerializerProvider provider) throws IOException {
      jgen.writeStartArray();
      jgen.writeNumber(value.timestamp);
      jgen.writeNumber(value.value);
      jgen.writeEndArray();
    }
  }

  public static class Deserializer extends JsonDeserializer<Point> {
    @Override
    public Point deserialize(final JsonParser parser, final DeserializationContext ctxt) throws IOException {
      if (parser.getCurrentToken() != JsonToken.START_ARRAY) {
        throw ctxt.wrongTokenException(parser, JsonToken.START_ARRAY, null);
      }

      if (!parser.nextToken().isNumeric()) {
        throw ctxt.wrongTokenException(parser, JsonToken.VALUE_NUMBER_INT, null);
      }

      final long v_timestamp = parser.getLongValue();

      if (!parser.nextToken().isNumeric()) {
        throw ctxt.wrongTokenException(parser, JsonToken.VALUE_NUMBER_FLOAT, null);
      }

      final double v_value = parser.getDoubleValue();

      if (parser.nextToken() != JsonToken.END_ARRAY) {
        throw ctxt.wrongTokenException(parser, JsonToken.END_ARRAY, null);
      }

      return new Point(v_timestamp, v_value);
    }
  }
}
