package test;

import com.fasterxml.jackson.annotation.JsonCreator;
import java.util.Objects;
import java.util.Optional;

public class Entry {
  private final Optional<Data> data;
  private final Optional<Point> point;
  private final Optional<Interface> interfaceField;
  private final Optional<Type> typeField;

  @JsonCreator
  public Entry(
    @JsonProperty("data") final Optional<Data> data, 
    @JsonProperty("point") final Optional<Point> point, 
    @JsonProperty("interface_field") final Optional<Interface> interfaceField, 
    @JsonProperty("type_field") final Optional<Type> typeField
  ) {
    Objects.requireNonNull(data, "data");
    this.data = data;
    Objects.requireNonNull(point, "point");
    this.point = point;
    Objects.requireNonNull(interfaceField, "interfaceField");
    this.interfaceField = interfaceField;
    Objects.requireNonNull(typeField, "typeField");
    this.typeField = typeField;
  }

  public Optional<Data> getData() {
    return this.data;
  }

  public Optional<Point> getPoint() {
    return this.point;
  }

  public Optional<Interface> getInterfaceField() {
    return this.interfaceField;
  }

  public Optional<Type> getTypeField() {
    return this.typeField;
  }

  @Override
  public int hashCode() {
    int result = 1;
    result = result * 31 + this.data.hashCode();
    result = result * 31 + this.point.hashCode();
    result = result * 31 + this.interfaceField.hashCode();
    result = result * 31 + this.typeField.hashCode();
    return result;
  }

  @Override
  public boolean equals(final Object other) {
    if (other == null) {
      return false;
    }

    if (!(other instanceof Entry)) {
      return false;
    }

    @SuppressWarnings("unchecked")
    final Entry o = (Entry) other;

    if (!this.data.equals(o.data)) {
      return false;
    }

    if (!this.point.equals(o.point)) {
      return false;
    }

    if (!this.interfaceField.equals(o.interfaceField)) {
      return false;
    }

    if (!this.typeField.equals(o.typeField)) {
      return false;
    }

    return true;
  }

  @Override
  public String toString() {
    final StringBuilder b = new StringBuilder();

    b.append("Entry");
    b.append("(");
    b.append("data=");
    b.append(this.data.toString());
    b.append(", ");
    b.append("point=");
    b.append(this.point.toString());
    b.append(", ");
    b.append("interfaceField=");
    b.append(this.interfaceField.toString());
    b.append(", ");
    b.append("typeField=");
    b.append(this.typeField.toString());
    b.append(")");

    return b.toString();
  }

  public static class Builder {
    private Optional<Data> data = Optional.empty();
    private Optional<Point> point = Optional.empty();
    private Optional<Interface> interfaceField = Optional.empty();
    private Optional<Type> typeField = Optional.empty();

    public Builder data(final Data data) {
      this.data = Optional.of(data);
      return this;
    }

    public Builder point(final Point point) {
      this.point = Optional.of(point);
      return this;
    }

    public Builder interfaceField(final Interface interfaceField) {
      this.interfaceField = Optional.of(interfaceField);
      return this;
    }

    public Builder typeField(final Type typeField) {
      this.typeField = Optional.of(typeField);
      return this;
    }

    public Entry build() {
      final Optional<Data> data = this.data;
      final Optional<Point> point = this.point;
      final Optional<Interface> interfaceField = this.interfaceField;
      final Optional<Type> typeField = this.typeField;

      return new Entry(data, point, interfaceField, typeField);
    }
  }
}
