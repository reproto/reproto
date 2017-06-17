package test;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonSubTypes;
import com.fasterxml.jackson.annotation.JsonTypeInfo;
import java.util.Objects;
import java.util.Optional;

@JsonTypeInfo(use=JsonTypeInfo.Id.NAME, include=JsonTypeInfo.As.PROPERTY, property="type")
@JsonSubTypes({@JsonSubTypes.Type(name="one", value=Interface.One.class), @JsonSubTypes.Type(name="two", value=Interface.Two.class)})
public interface Interface {
  public String getName();

  public Optional<Integer> getOther();

  public static class One implements Interface {
    private final String name;
    private final Optional<Integer> other;
    private final Data data;

    @JsonCreator
    public One(
      @JsonProperty("name") final String name, 
      @JsonProperty("other") final Optional<Integer> other, 
      @JsonProperty("data") final Data data
    ) {
      Objects.requireNonNull(name, "name");
      this.name = name;
      Objects.requireNonNull(other, "other");
      this.other = other;
      Objects.requireNonNull(data, "data");
      this.data = data;
    }

    @Override
    public String getName() {
      return this.name;
    }

    @Override
    public Optional<Integer> getOther() {
      return this.other;
    }

    public Data getData() {
      return this.data;
    }

    @Override
    public int hashCode() {
      int result = 1;
      result = result * 31 + this.name.hashCode();
      result = result * 31 + this.other.hashCode();
      result = result * 31 + this.data.hashCode();
      return result;
    }

    @Override
    public boolean equals(final Object other) {
      if (other == null) {
        return false;
      }

      if (!(other instanceof Interface.One)) {
        return false;
      }

      @SuppressWarnings("unchecked")
      final Interface.One o = (Interface.One) other;

      if (!this.name.equals(o.name)) {
        return false;
      }

      if (!this.other.equals(o.other)) {
        return false;
      }

      if (!this.data.equals(o.data)) {
        return false;
      }

      return true;
    }

    @Override
    public String toString() {
      final StringBuilder b = new StringBuilder();

      b.append("Interface.One");
      b.append("(");
      b.append("name=");
      b.append(this.name.toString());
      b.append(", ");
      b.append("other=");
      b.append(this.other.toString());
      b.append(", ");
      b.append("data=");
      b.append(this.data.toString());
      b.append(")");

      return b.toString();
    }

    public static class Builder {
      private Optional<String> name = Optional.empty();
      private Optional<Integer> other = Optional.empty();
      private Optional<Data> data = Optional.empty();

      public Builder name(final String name) {
        this.name = Optional.of(name);
        return this;
      }

      public Builder other(final int other) {
        this.other = Optional.of(other);
        return this;
      }

      public Builder data(final Data data) {
        this.data = Optional.of(data);
        return this;
      }

      public Interface.One build() {
        final String name = this.name.orElseThrow(() -> new RuntimeException("name: is required"));
        final Optional<Integer> other = this.other;
        final Data data = this.data.orElseThrow(() -> new RuntimeException("data: is required"));

        return new Interface.One(name, other, data);
      }
    }
  }

  public static class Two implements Interface {
    private final String name;
    private final Optional<Integer> other;
    private final Data data;

    @JsonCreator
    public Two(
      @JsonProperty("name") final String name, 
      @JsonProperty("other") final Optional<Integer> other, 
      @JsonProperty("data") final Data data
    ) {
      Objects.requireNonNull(name, "name");
      this.name = name;
      Objects.requireNonNull(other, "other");
      this.other = other;
      Objects.requireNonNull(data, "data");
      this.data = data;
    }

    @Override
    public String getName() {
      return this.name;
    }

    @Override
    public Optional<Integer> getOther() {
      return this.other;
    }

    public Data getData() {
      return this.data;
    }

    @Override
    public int hashCode() {
      int result = 1;
      result = result * 31 + this.name.hashCode();
      result = result * 31 + this.other.hashCode();
      result = result * 31 + this.data.hashCode();
      return result;
    }

    @Override
    public boolean equals(final Object other) {
      if (other == null) {
        return false;
      }

      if (!(other instanceof Interface.Two)) {
        return false;
      }

      @SuppressWarnings("unchecked")
      final Interface.Two o = (Interface.Two) other;

      if (!this.name.equals(o.name)) {
        return false;
      }

      if (!this.other.equals(o.other)) {
        return false;
      }

      if (!this.data.equals(o.data)) {
        return false;
      }

      return true;
    }

    @Override
    public String toString() {
      final StringBuilder b = new StringBuilder();

      b.append("Interface.Two");
      b.append("(");
      b.append("name=");
      b.append(this.name.toString());
      b.append(", ");
      b.append("other=");
      b.append(this.other.toString());
      b.append(", ");
      b.append("data=");
      b.append(this.data.toString());
      b.append(")");

      return b.toString();
    }

    public static class Builder {
      private Optional<String> name = Optional.empty();
      private Optional<Integer> other = Optional.empty();
      private Optional<Data> data = Optional.empty();

      public Builder name(final String name) {
        this.name = Optional.of(name);
        return this;
      }

      public Builder other(final int other) {
        this.other = Optional.of(other);
        return this;
      }

      public Builder data(final Data data) {
        this.data = Optional.of(data);
        return this;
      }

      public Interface.Two build() {
        final String name = this.name.orElseThrow(() -> new RuntimeException("name: is required"));
        final Optional<Integer> other = this.other;
        final Data data = this.data.orElseThrow(() -> new RuntimeException("data: is required"));

        return new Interface.Two(name, other, data);
      }
    }
  }
}
