package test;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonSubTypes;
import com.fasterxml.jackson.annotation.JsonTypeInfo;

@JsonTypeInfo(use=JsonTypeInfo.Id.NAME, include=JsonTypeInfo.As.PROPERTY, property="type")
@JsonSubTypes({
  @JsonSubTypes.Type(name="foo", value=Entry.A.class),
  @JsonSubTypes.Type(name="b", value=Entry.B.class),
  @JsonSubTypes.Type(name="Bar", value=Entry.Bar.class),
  @JsonSubTypes.Type(name="Baz", value=Entry.Baz.class)
})
public interface Entry {
  public static class A implements Entry {
    @JsonCreator
    public A() {
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

      if (!(other instanceof A)) {
        return false;
      }

      @SuppressWarnings("unchecked")
      final A o = (A) other;

      return true;
    }

    @Override
    public String toString() {
      final StringBuilder b = new StringBuilder();

      b.append("A");
      b.append("(");
      b.append(")");

      return b.toString();
    }

    public static class Builder {
      public A build() {

        return new A();
      }
    }
  }

  public static class B implements Entry {
    @JsonCreator
    public B() {
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

      if (!(other instanceof B)) {
        return false;
      }

      @SuppressWarnings("unchecked")
      final B o = (B) other;

      return true;
    }

    @Override
    public String toString() {
      final StringBuilder b = new StringBuilder();

      b.append("B");
      b.append("(");
      b.append(")");

      return b.toString();
    }

    public static class Builder {
      public B build() {

        return new B();
      }
    }
  }

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

  public static class Baz implements Entry {
    @JsonCreator
    public Baz() {
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

      if (!(other instanceof Baz)) {
        return false;
      }

      @SuppressWarnings("unchecked")
      final Baz o = (Baz) other;

      return true;
    }

    @Override
    public String toString() {
      final StringBuilder b = new StringBuilder();

      b.append("Baz");
      b.append("(");
      b.append(")");

      return b.toString();
    }

    public static class Builder {
      public Baz build() {

        return new Baz();
      }
    }
  }
}
