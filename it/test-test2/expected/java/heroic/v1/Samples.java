package heroic.v1;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonSubTypes;
import com.fasterxml.jackson.annotation.JsonTypeInfo;
import java.util.List;
import java.util.Objects;
import java.util.Optional;

@JsonTypeInfo(use=JsonTypeInfo.Id.NAME, include=JsonTypeInfo.As.PROPERTY, property="type")
@JsonSubTypes({@JsonSubTypes.Type(name="events", value=Samples.Events.class), @JsonSubTypes.Type(name="points", value=Samples.Points.class)})
public interface Samples {
  public String getName();

  public static class Events implements Samples {
    private final String name;
    private final List<Event> data;

    @JsonCreator
    public Events(
      @JsonProperty("name") final String name, 
      @JsonProperty("data") final List<Event> data
    ) {
      Objects.requireNonNull(name, "name");
      this.name = name;
      Objects.requireNonNull(data, "data");
      this.data = data;
    }

    @Override
    public String getName() {
      return this.name;
    }

    public List<Event> getData() {
      return this.data;
    }

    @Override
    public int hashCode() {
      int result = 1;
      result = result * 31 + this.name.hashCode();
      result = result * 31 + this.data.hashCode();
      return result;
    }

    @Override
    public boolean equals(final Object other) {
      if (other == null) {
        return false;
      }

      if (!(other instanceof Samples.Events)) {
        return false;
      }

      @SuppressWarnings("unchecked")
      final Samples.Events o = (Samples.Events) other;

      if (!this.name.equals(o.name)) {
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

      b.append("Samples.Events");
      b.append("(");
      b.append("name=");
      b.append(this.name.toString());
      b.append(", ");
      b.append("data=");
      b.append(this.data.toString());
      b.append(")");

      return b.toString();
    }

    public static class Builder {
      private Optional<String> name = Optional.empty();
      private Optional<List<Event>> data = Optional.empty();

      public Builder name(final String name) {
        this.name = Optional.of(name);
        return this;
      }

      public Builder data(final List<Event> data) {
        this.data = Optional.of(data);
        return this;
      }

      public Samples.Events build() {
        final String name = this.name.orElseThrow(() -> new RuntimeException("name: is required"));
        final List<Event> data = this.data.orElseThrow(() -> new RuntimeException("data: is required"));

        return new Samples.Events(name, data);
      }
    }
  }

  public static class Points implements Samples {
    private final String name;
    private final List<Point> data;

    @JsonCreator
    public Points(
      @JsonProperty("name") final String name, 
      @JsonProperty("data") final List<Point> data
    ) {
      Objects.requireNonNull(name, "name");
      this.name = name;
      Objects.requireNonNull(data, "data");
      this.data = data;
    }

    @Override
    public String getName() {
      return this.name;
    }

    public List<Point> getData() {
      return this.data;
    }

    @Override
    public int hashCode() {
      int result = 1;
      result = result * 31 + this.name.hashCode();
      result = result * 31 + this.data.hashCode();
      return result;
    }

    @Override
    public boolean equals(final Object other) {
      if (other == null) {
        return false;
      }

      if (!(other instanceof Samples.Points)) {
        return false;
      }

      @SuppressWarnings("unchecked")
      final Samples.Points o = (Samples.Points) other;

      if (!this.name.equals(o.name)) {
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

      b.append("Samples.Points");
      b.append("(");
      b.append("name=");
      b.append(this.name.toString());
      b.append(", ");
      b.append("data=");
      b.append(this.data.toString());
      b.append(")");

      return b.toString();
    }

    public static class Builder {
      private Optional<String> name = Optional.empty();
      private Optional<List<Point>> data = Optional.empty();

      public Builder name(final String name) {
        this.name = Optional.of(name);
        return this;
      }

      public Builder data(final List<Point> data) {
        this.data = Optional.of(data);
        return this;
      }

      public Samples.Points build() {
        final String name = this.name.orElseThrow(() -> new RuntimeException("name: is required"));
        final List<Point> data = this.data.orElseThrow(() -> new RuntimeException("data: is required"));

        return new Samples.Points(name, data);
      }
    }
  }
}
