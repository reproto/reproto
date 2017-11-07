package test;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonSubTypes;
import com.fasterxml.jackson.annotation.JsonTypeInfo;

@JsonTypeInfo(use=JsonTypeInfo.Id.NAME, include=JsonTypeInfo.As.PROPERTY, property="type")
@JsonSubTypes({
  @JsonSubTypes.Type(name="bar", value=Entry.Bar.class),
  @JsonSubTypes.Type(name="foo", value=Entry.Foo.class)
})
public interface Entry {
  public static class Bar implements Entry {
    @JsonCreator
    public Bar() {
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

      if (!(other instanceof Bar)) {
        return false;
      }

      @SuppressWarnings("unchecked")
      final Bar o = (Bar) other;

      return true;
    }

    @Override
    public String toString() {
      final StringBuilder b = new StringBuilder();

      b.append("Bar");
      b.append("(");
      b.append(")");

      return b.toString();
    }

    public static class Builder {
      public Bar build() {

        return new Bar();
      }
    }
  }

  public static class Foo implements Entry {
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
  }
}
