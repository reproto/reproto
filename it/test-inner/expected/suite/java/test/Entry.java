package test;

import java.util.Objects;
import java.util.Optional;

public class Entry {
  private final Entry.A a;
  private final Entry.A.B b;

  public Entry(
    final Entry.A a,
    final Entry.A.B b
  ) {
    Objects.requireNonNull(a, "a");
    this.a = a;
    Objects.requireNonNull(b, "b");
    this.b = b;
  }

  public Entry.A getA() {
    return this.a;
  }

  public Entry.A.B getB() {
    return this.b;
  }

  @Override
  public int hashCode() {
    int result = 1;
    result = result * 31 + this.a.hashCode();
    result = result * 31 + this.b.hashCode();
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

    if (!this.a.equals(o.a)) {
      return false;
    }

    if (!this.b.equals(o.b)) {
      return false;
    }

    return true;
  }

  @Override
  public String toString() {
    final StringBuilder b = new StringBuilder();

    b.append("Entry");
    b.append("(");
    b.append("a=");
    b.append(this.a.toString());
    b.append(", ");
    b.append("b=");
    b.append(this.b.toString());
    b.append(")");

    return b.toString();
  }

  public static class Builder {
    private Optional<Entry.A> a = Optional.empty();
    private Optional<Entry.A.B> b = Optional.empty();

    public Builder a(final Entry.A a) {
      this.a = Optional.of(a);
      return this;
    }

    public Builder b(final Entry.A.B b) {
      this.b = Optional.of(b);
      return this;
    }

    public Entry build() {
      final Entry.A a = this.a.orElseThrow(() -> new RuntimeException("a: is required"));
      final Entry.A.B b = this.b.orElseThrow(() -> new RuntimeException("b: is required"));

      return new Entry(a, b);
    }
  }

  public static class A {
    private final Entry.A.B b;

    public A(
      final Entry.A.B b
    ) {
      Objects.requireNonNull(b, "b");
      this.b = b;
    }

    public Entry.A.B getB() {
      return this.b;
    }

    @Override
    public int hashCode() {
      int result = 1;
      result = result * 31 + this.b.hashCode();
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

      if (!this.b.equals(o.b)) {
        return false;
      }

      return true;
    }

    @Override
    public String toString() {
      final StringBuilder b = new StringBuilder();

      b.append("A");
      b.append("(");
      b.append("b=");
      b.append(this.b.toString());
      b.append(")");

      return b.toString();
    }

    public static class Builder {
      private Optional<Entry.A.B> b = Optional.empty();

      public Builder b(final Entry.A.B b) {
        this.b = Optional.of(b);
        return this;
      }

      public A build() {
        final Entry.A.B b = this.b.orElseThrow(() -> new RuntimeException("b: is required"));

        return new A(b);
      }
    }

    public static class B {
      private final String field;

      public B(
        final String field
      ) {
        Objects.requireNonNull(field, "field");
        this.field = field;
      }

      public String getField() {
        return this.field;
      }

      @Override
      public int hashCode() {
        int result = 1;
        result = result * 31 + this.field.hashCode();
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

        if (!this.field.equals(o.field)) {
          return false;
        }

        return true;
      }

      @Override
      public String toString() {
        final StringBuilder b = new StringBuilder();

        b.append("B");
        b.append("(");
        b.append("field=");
        b.append(this.field.toString());
        b.append(")");

        return b.toString();
      }

      public static class Builder {
        private Optional<String> field = Optional.empty();

        public Builder field(final String field) {
          this.field = Optional.of(field);
          return this;
        }

        public B build() {
          final String field = this.field.orElseThrow(() -> new RuntimeException("field: is required"));

          return new B(field);
        }
      }
    }
  }
}
