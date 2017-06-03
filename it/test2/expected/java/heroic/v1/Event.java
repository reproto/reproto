package heroic.v1;

import java.util.Objects;
import java.util.Optional;

public class Event {
  private final long timestamp;
  private final Optional<Object> payload;

  public Event(final long timestamp, final Optional<Object> payload) {
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
}
