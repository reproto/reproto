package test;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.core.JsonParser;
import com.fasterxml.jackson.core.JsonToken;
import com.fasterxml.jackson.databind.DeserializationContext;
import com.fasterxml.jackson.databind.JsonDeserializer;
import com.fasterxml.jackson.databind.annotation.JsonDeserialize;
import java.io.IOException;
import java.util.Objects;
import java.util.Optional;

@JsonDeserialize(using = Type.Deserializer.class)
public class Type {
  private final String data;
  private final Optional<Integer> other;

  public Type(
    final String data, final Optional<Integer> other
  ) {
    Objects.requireNonNull(data, "data");
    this.data = data;
    Objects.requireNonNull(other, "other");
    this.other = other;
  }

  public String getData() {
    return this.data;
  }

  public Optional<Integer> getOther() {
    return this.other;
  }

  @Override
  public int hashCode() {
    int result = 1;
    result = result * 31 + this.data.hashCode();
    result = result * 31 + this.other.hashCode();
    return result;
  }

  @Override
  public boolean equals(final Object other) {
    if (other == null) {
      return false;
    }

    if (!(other instanceof Type)) {
      return false;
    }

    @SuppressWarnings("unchecked")
    final Type o = (Type) other;

    if (!this.data.equals(o.data)) {
      return false;
    }

    if (!this.other.equals(o.other)) {
      return false;
    }

    return true;
  }

  @Override
  public String toString() {
    final StringBuilder b = new StringBuilder();

    b.append("Type");
    b.append("(");
    b.append("data=");
    b.append(this.data.toString());
    b.append(", ");
    b.append("other=");
    b.append(this.other.toString());
    b.append(")");

    return b.toString();
  }

  public static class Builder {
    private Optional<String> data = Optional.empty();
    private Optional<Integer> other = Optional.empty();

    public Builder data(final String data) {
      this.data = Optional.of(data);
      return this;
    }

    public Builder other(final int other) {
      this.other = Optional.of(other);
      return this;
    }

    public Type build() {
      final String data = this.data.orElseThrow(() -> new RuntimeException("data: is required"));
      final Optional<Integer> other = this.other;

      return new Type(data, other);
    }
  }

  public static class Model {
    private final String data;
    private final Optional<Integer> other;

    @JsonCreator
    public Model(
      @JsonProperty("data") final String data, 
      @JsonProperty("other") final Optional<Integer> other
    ) {
      this.data = data;
      this.other = other;
    }
  }

  public static class Deserializer extends JsonDeserializer<Type> {
    @Override
    public Type deserialize(final JsonParser parser, final DeserializationContext ctxt) throws IOException {
      if (parser.getCurrentToken() == JsonToken.VALUE_STRING && parser.getText() == "foo") {
        return new Type("foo", Optional.empty());
      }

      if (parser.getCurrentToken() == JsonToken.VALUE_STRING) {
        final String data = parser.getText();
        return new Type(data, Optional.empty());
      }

      final Model m = parser.readValueAs(Model.class);
      return new Type(m.data, m.other);
    }
  }
}
