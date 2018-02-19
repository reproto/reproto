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
import java.util.Objects;

@JsonSerialize(using = RootTuple.Serializer.class)
@JsonDeserialize(using = RootTuple.Deserializer.class)
public class RootTuple {
  public RootTuple() {
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

    if (!(other instanceof RootTuple)) {
      return false;
    }

    @SuppressWarnings("unchecked")
    final RootTuple o = (RootTuple) other;

    return true;
  }

  @Override
  public String toString() {
    final StringBuilder b = new StringBuilder();

    b.append("RootTuple");
    b.append("(");
    b.append(")");

    return b.toString();
  }

  public static class Serializer extends JsonSerializer<RootTuple> {
    @Override
    public void serialize(final RootTuple value, final JsonGenerator jgen, final SerializerProvider provider) throws IOException {
      jgen.writeStartArray();
      jgen.writeEndArray();
    }
  }

  public static class Deserializer extends JsonDeserializer<RootTuple> {
    @Override
    public RootTuple deserialize(final JsonParser parser, final DeserializationContext ctxt) throws IOException {
      if (parser.getCurrentToken() != JsonToken.START_ARRAY) {
        throw ctxt.wrongTokenException(parser, JsonToken.START_ARRAY, null);
      }

      if (parser.nextToken() != JsonToken.END_ARRAY) {
        throw ctxt.wrongTokenException(parser, JsonToken.END_ARRAY, null);
      }

      return new RootTuple();
    }
  }

  public static class NestedType {
    @JsonCreator
    public NestedType() {
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

      if (!(other instanceof NestedType)) {
        return false;
      }

      @SuppressWarnings("unchecked")
      final NestedType o = (NestedType) other;

      return true;
    }

    @Override
    public String toString() {
      final StringBuilder b = new StringBuilder();

      b.append("NestedType");
      b.append("(");
      b.append(")");

      return b.toString();
    }

    public static class Builder {
      public NestedType build() {

        return new NestedType();
      }
    }
  }

  @JsonTypeInfo(use=JsonTypeInfo.Id.NAME, include=JsonTypeInfo.As.PROPERTY, property="type")
  @JsonSubTypes({
    @JsonSubTypes.Type(name="Foo", value=NestedInterface.Foo.class)
  })
  public interface NestedInterface {
    public static class Foo implements NestedInterface {
      @JsonCreator
      public Foo() {
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

        if (!(other instanceof Foo)) {
          return false;
        }

        @SuppressWarnings("unchecked")
        final Foo o = (Foo) other;

        return true;
      }

      @Override
      public String toString() {
        final StringBuilder b = new StringBuilder();

        b.append("Foo");
        b.append("(");
        b.append(")");

        return b.toString();
      }

      public static class Builder {
        public Foo build() {

          return new Foo();
        }
      }

      public static class Nested {
        @JsonCreator
        public Nested() {
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

          if (!(other instanceof Nested)) {
            return false;
          }

          @SuppressWarnings("unchecked")
          final Nested o = (Nested) other;

          return true;
        }

        @Override
        public String toString() {
          final StringBuilder b = new StringBuilder();

          b.append("Nested");
          b.append("(");
          b.append(")");

          return b.toString();
        }

        public static class Builder {
          public Nested build() {

            return new Nested();
          }
        }
      }
    }
  }

  public static enum NestedEnum {
    FOO("Foo");

    private final String value;

    private NestedEnum(
      final String value
    ) {
      Objects.requireNonNull(value, "value");
      this.value = value;
    }

    @JsonCreator
    public static NestedEnum fromValue(final String value) {
      for (final NestedEnum v_value : values()) {
        if (v_value.value.equals(value)) {
          return v_value;
        }
      }

      throw new IllegalArgumentException("value");
    }

    @JsonValue
    public String toValue() {
      return this.value;
    }

    public static class Nested {
      @JsonCreator
      public Nested() {
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

        if (!(other instanceof Nested)) {
          return false;
        }

        @SuppressWarnings("unchecked")
        final Nested o = (Nested) other;

        return true;
      }

      @Override
      public String toString() {
        final StringBuilder b = new StringBuilder();

        b.append("Nested");
        b.append("(");
        b.append(")");

        return b.toString();
      }

      public static class Builder {
        public Nested build() {

          return new Nested();
        }
      }
    }
  }

  @JsonSerialize(using = NestedTuple.Serializer.class)
  @JsonDeserialize(using = NestedTuple.Deserializer.class)
  public static class NestedTuple {
    public NestedTuple() {
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

      if (!(other instanceof NestedTuple)) {
        return false;
      }

      @SuppressWarnings("unchecked")
      final NestedTuple o = (NestedTuple) other;

      return true;
    }

    @Override
    public String toString() {
      final StringBuilder b = new StringBuilder();

      b.append("NestedTuple");
      b.append("(");
      b.append(")");

      return b.toString();
    }

    public static class Serializer extends JsonSerializer<NestedTuple> {
      @Override
      public void serialize(final NestedTuple value, final JsonGenerator jgen, final SerializerProvider provider) throws IOException {
        jgen.writeStartArray();
        jgen.writeEndArray();
      }
    }

    public static class Deserializer extends JsonDeserializer<NestedTuple> {
      @Override
      public NestedTuple deserialize(final JsonParser parser, final DeserializationContext ctxt) throws IOException {
        if (parser.getCurrentToken() != JsonToken.START_ARRAY) {
          throw ctxt.wrongTokenException(parser, JsonToken.START_ARRAY, null);
        }

        if (parser.nextToken() != JsonToken.END_ARRAY) {
          throw ctxt.wrongTokenException(parser, JsonToken.END_ARRAY, null);
        }

        return new NestedTuple();
      }
    }

    public static class Nested {
      @JsonCreator
      public Nested() {
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

        if (!(other instanceof Nested)) {
          return false;
        }

        @SuppressWarnings("unchecked")
        final Nested o = (Nested) other;

        return true;
      }

      @Override
      public String toString() {
        final StringBuilder b = new StringBuilder();

        b.append("Nested");
        b.append("(");
        b.append(")");

        return b.toString();
      }

      public static class Builder {
        public Nested build() {

          return new Nested();
        }
      }
    }
  }

  public interface NestedService {
  }
}
