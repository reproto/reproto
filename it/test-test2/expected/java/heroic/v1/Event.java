package heroic.v1;

import com.fasterxml.jackson.core.JsonGenerator;
import com.fasterxml.jackson.core.JsonParser;
import com.fasterxml.jackson.core.JsonToken;
import com.fasterxml.jackson.core.type.TypeReference;
import com.fasterxml.jackson.databind.DeserializationContext;
import com.fasterxml.jackson.databind.JsonDeserializer;
import com.fasterxml.jackson.databind.JsonSerializer;
import com.fasterxml.jackson.databind.SerializerProvider;
import com.fasterxml.jackson.databind.annotation.JsonDeserialize;
import com.fasterxml.jackson.databind.annotation.JsonSerialize;
import java.io.IOException;
import java.util.Objects;
import java.util.Optional;

@JsonSerialize(using = Event.Serializer.class)
@JsonDeserialize(using = Event.Deserializer.class)
public class Event {
  private final long timestamp;
  private final Optional<Object> payload;

  public Event(
    final long timestamp, final Optional<Object> payload
  ) {
    this.timestamp = timestamp;
    Objects.requireNonNull(payload, "payload");
    this.payload = payload;
  }

  public long getTimestamp() {
    return this.timestamp;
  }

  public Optional<Object> getPayload() {
    return this.payload;
  }

  @Override
  public int hashCode() {
    int result = 1;
    result = result * 31 + Long.hashCode(this.timestamp);
    result = result * 31 + this.payload.hashCode();
    return result;
  }

  @Override
  public boolean equals(final Object other) {
    if (other == null) {
      return false;
    }

    if (!(other instanceof Event)) {
      return false;
    }

    @SuppressWarnings("unchecked")
    final Event o = (Event) other;

    if (this.timestamp != o.timestamp) {
      return false;
    }

    if (!this.payload.equals(o.payload)) {
      return false;
    }

    return true;
  }

  @Override
  public String toString() {
    final StringBuilder b = new StringBuilder();

    b.append("Event");
    b.append("(");
    b.append("timestamp=");
    b.append(Long.toString(this.timestamp));
    b.append(", ");
    b.append("payload=");
    b.append(this.payload.toString());
    b.append(")");

    return b.toString();
  }

  public static class Serializer extends JsonSerializer<Event> {
    @Override
    public void serialize(final Event value, final JsonGenerator jgen, final SerializerProvider provider) throws IOException {
      jgen.writeStartArray();
      jgen.writeNumber(value.timestamp);
      jgen.writeObject(value.payload);
      jgen.writeEndArray();
    }
  }

  public static class Deserializer extends JsonDeserializer<Event> {
    @Override
    public Event deserialize(final JsonParser parser, final DeserializationContext ctxt) throws IOException {
      if (parser.getCurrentToken() != JsonToken.START_ARRAY) {
        throw ctxt.wrongTokenException(parser, JsonToken.START_ARRAY, null);
      }

      if (!parser.nextToken().isNumeric()) {
        throw ctxt.wrongTokenException(parser, JsonToken.VALUE_NUMBER_INT, null);
      }

      final long v_timestamp = parser.getLongValue();

      final Optional<Object> v_payload = parser.readValueAs(new TypeReference<Optional<Object>>(){});

      if (parser.nextToken() != JsonToken.END_ARRAY) {
        throw ctxt.wrongTokenException(parser, JsonToken.END_ARRAY, null);
      }

      return new Event(v_timestamp, v_payload);
    }
  }
}
