package test;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.core.JsonParser;
import com.fasterxml.jackson.databind.DeserializationContext;
import com.fasterxml.jackson.databind.JsonDeserializer;
import com.fasterxml.jackson.databind.annotation.JsonDeserialize;
import com.fasterxml.jackson.databind.node.ObjectNode;
import com.fasterxml.jackson.databind.node.TreeTraversingParser;
import java.io.IOException;
import java.util.HashSet;
import java.util.Iterator;
import java.util.Objects;
import java.util.Optional;
import java.util.Set;

@JsonDeserialize(using = RequiredFields.Deserializer.class)
public interface RequiredFields {
  String getShared();

  Optional<String> getSharedIgnore();

  @JsonDeserialize(using = JsonDeserializer.None.class)
  public static class A implements RequiredFields {
    @JsonProperty("shared")
    private final String shared;
    @JsonProperty("shared_ignore")
    private final Optional<String> sharedIgnore;
    @JsonProperty("a")
    private final String a;
    @JsonProperty("b")
    private final String b;
    @JsonProperty("ignore")
    private final Optional<String> ignore;

    @JsonCreator
    public A(
      @JsonProperty("shared") final String shared,
      @JsonProperty("shared_ignore") final Optional<String> sharedIgnore,
      @JsonProperty("a") final String a,
      @JsonProperty("b") final String b,
      @JsonProperty("ignore") final Optional<String> ignore
    ) {
      Objects.requireNonNull(shared, "shared");
      this.shared = shared;
      Objects.requireNonNull(sharedIgnore, "shared_ignore");
      this.sharedIgnore = sharedIgnore;
      Objects.requireNonNull(a, "a");
      this.a = a;
      Objects.requireNonNull(b, "b");
      this.b = b;
      Objects.requireNonNull(ignore, "ignore");
      this.ignore = ignore;
    }

    @Override
    @JsonProperty("shared")
    public String getShared() {
      return this.shared;
    }

    @Override
    @JsonProperty("shared_ignore")
    public Optional<String> getSharedIgnore() {
      return this.sharedIgnore;
    }

    @JsonProperty("a")
    public String getA() {
      return this.a;
    }

    @JsonProperty("b")
    public String getB() {
      return this.b;
    }

    @JsonProperty("ignore")
    public Optional<String> getIgnore() {
      return this.ignore;
    }

    @Override
    public int hashCode() {
      int result = 1;
      result = result * 31 + this.shared.hashCode();
      result = result * 31 + this.sharedIgnore.hashCode();
      result = result * 31 + this.a.hashCode();
      result = result * 31 + this.b.hashCode();
      result = result * 31 + this.ignore.hashCode();
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

      if (!this.shared.equals(o.shared)) {
        return false;
      }

      if (!this.sharedIgnore.equals(o.sharedIgnore)) {
        return false;
      }

      if (!this.a.equals(o.a)) {
        return false;
      }

      if (!this.b.equals(o.b)) {
        return false;
      }

      if (!this.ignore.equals(o.ignore)) {
        return false;
      }

      return true;
    }

    @Override
    public String toString() {
      final StringBuilder b = new StringBuilder();

      b.append("A");
      b.append("(");
      b.append("shared=");
      b.append(this.shared.toString());
      b.append(", ");
      b.append("shared_ignore=");
      b.append(this.sharedIgnore.toString());
      b.append(", ");
      b.append("a=");
      b.append(this.a.toString());
      b.append(", ");
      b.append("b=");
      b.append(this.b.toString());
      b.append(", ");
      b.append("ignore=");
      b.append(this.ignore.toString());
      b.append(")");

      return b.toString();
    }

    public static class Builder {
      private Optional<String> shared = Optional.empty();
      private Optional<String> sharedIgnore = Optional.empty();
      private Optional<String> a = Optional.empty();
      private Optional<String> b = Optional.empty();
      private Optional<String> ignore = Optional.empty();

      public Builder shared(final String shared) {
        this.shared = Optional.of(shared);
        return this;
      }

      public Builder sharedIgnore(final String sharedIgnore) {
        this.sharedIgnore = Optional.of(sharedIgnore);
        return this;
      }

      public Builder a(final String a) {
        this.a = Optional.of(a);
        return this;
      }

      public Builder b(final String b) {
        this.b = Optional.of(b);
        return this;
      }

      public Builder ignore(final String ignore) {
        this.ignore = Optional.of(ignore);
        return this;
      }

      public A build() {
        final String shared = this.shared.orElseThrow(() -> new RuntimeException("shared: is required"));
        final Optional<String> sharedIgnore = this.sharedIgnore;
        final String a = this.a.orElseThrow(() -> new RuntimeException("a: is required"));
        final String b = this.b.orElseThrow(() -> new RuntimeException("b: is required"));
        final Optional<String> ignore = this.ignore;

        return new A(shared, sharedIgnore, a, b, ignore);
      }
    }
  }

  @JsonDeserialize(using = JsonDeserializer.None.class)
  public static class B implements RequiredFields {
    @JsonProperty("shared")
    private final String shared;
    @JsonProperty("shared_ignore")
    private final Optional<String> sharedIgnore;
    @JsonProperty("a")
    private final String a;
    @JsonProperty("ignore")
    private final Optional<String> ignore;

    @JsonCreator
    public B(
      @JsonProperty("shared") final String shared,
      @JsonProperty("shared_ignore") final Optional<String> sharedIgnore,
      @JsonProperty("a") final String a,
      @JsonProperty("ignore") final Optional<String> ignore
    ) {
      Objects.requireNonNull(shared, "shared");
      this.shared = shared;
      Objects.requireNonNull(sharedIgnore, "shared_ignore");
      this.sharedIgnore = sharedIgnore;
      Objects.requireNonNull(a, "a");
      this.a = a;
      Objects.requireNonNull(ignore, "ignore");
      this.ignore = ignore;
    }

    @Override
    @JsonProperty("shared")
    public String getShared() {
      return this.shared;
    }

    @Override
    @JsonProperty("shared_ignore")
    public Optional<String> getSharedIgnore() {
      return this.sharedIgnore;
    }

    @JsonProperty("a")
    public String getA() {
      return this.a;
    }

    @JsonProperty("ignore")
    public Optional<String> getIgnore() {
      return this.ignore;
    }

    @Override
    public int hashCode() {
      int result = 1;
      result = result * 31 + this.shared.hashCode();
      result = result * 31 + this.sharedIgnore.hashCode();
      result = result * 31 + this.a.hashCode();
      result = result * 31 + this.ignore.hashCode();
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

      if (!this.shared.equals(o.shared)) {
        return false;
      }

      if (!this.sharedIgnore.equals(o.sharedIgnore)) {
        return false;
      }

      if (!this.a.equals(o.a)) {
        return false;
      }

      if (!this.ignore.equals(o.ignore)) {
        return false;
      }

      return true;
    }

    @Override
    public String toString() {
      final StringBuilder b = new StringBuilder();

      b.append("B");
      b.append("(");
      b.append("shared=");
      b.append(this.shared.toString());
      b.append(", ");
      b.append("shared_ignore=");
      b.append(this.sharedIgnore.toString());
      b.append(", ");
      b.append("a=");
      b.append(this.a.toString());
      b.append(", ");
      b.append("ignore=");
      b.append(this.ignore.toString());
      b.append(")");

      return b.toString();
    }

    public static class Builder {
      private Optional<String> shared = Optional.empty();
      private Optional<String> sharedIgnore = Optional.empty();
      private Optional<String> a = Optional.empty();
      private Optional<String> ignore = Optional.empty();

      public Builder shared(final String shared) {
        this.shared = Optional.of(shared);
        return this;
      }

      public Builder sharedIgnore(final String sharedIgnore) {
        this.sharedIgnore = Optional.of(sharedIgnore);
        return this;
      }

      public Builder a(final String a) {
        this.a = Optional.of(a);
        return this;
      }

      public Builder ignore(final String ignore) {
        this.ignore = Optional.of(ignore);
        return this;
      }

      public B build() {
        final String shared = this.shared.orElseThrow(() -> new RuntimeException("shared: is required"));
        final Optional<String> sharedIgnore = this.sharedIgnore;
        final String a = this.a.orElseThrow(() -> new RuntimeException("a: is required"));
        final Optional<String> ignore = this.ignore;

        return new B(shared, sharedIgnore, a, ignore);
      }
    }
  }

  @JsonDeserialize(using = JsonDeserializer.None.class)
  public static class C implements RequiredFields {
    @JsonProperty("shared")
    private final String shared;
    @JsonProperty("shared_ignore")
    private final Optional<String> sharedIgnore;
    @JsonProperty("b")
    private final String b;
    @JsonProperty("ignore")
    private final Optional<String> ignore;

    @JsonCreator
    public C(
      @JsonProperty("shared") final String shared,
      @JsonProperty("shared_ignore") final Optional<String> sharedIgnore,
      @JsonProperty("b") final String b,
      @JsonProperty("ignore") final Optional<String> ignore
    ) {
      Objects.requireNonNull(shared, "shared");
      this.shared = shared;
      Objects.requireNonNull(sharedIgnore, "shared_ignore");
      this.sharedIgnore = sharedIgnore;
      Objects.requireNonNull(b, "b");
      this.b = b;
      Objects.requireNonNull(ignore, "ignore");
      this.ignore = ignore;
    }

    @Override
    @JsonProperty("shared")
    public String getShared() {
      return this.shared;
    }

    @Override
    @JsonProperty("shared_ignore")
    public Optional<String> getSharedIgnore() {
      return this.sharedIgnore;
    }

    @JsonProperty("b")
    public String getB() {
      return this.b;
    }

    @JsonProperty("ignore")
    public Optional<String> getIgnore() {
      return this.ignore;
    }

    @Override
    public int hashCode() {
      int result = 1;
      result = result * 31 + this.shared.hashCode();
      result = result * 31 + this.sharedIgnore.hashCode();
      result = result * 31 + this.b.hashCode();
      result = result * 31 + this.ignore.hashCode();
      return result;
    }

    @Override
    public boolean equals(final Object other) {
      if (other == null) {
        return false;
      }

      if (!(other instanceof C)) {
        return false;
      }

      @SuppressWarnings("unchecked")
      final C o = (C) other;

      if (!this.shared.equals(o.shared)) {
        return false;
      }

      if (!this.sharedIgnore.equals(o.sharedIgnore)) {
        return false;
      }

      if (!this.b.equals(o.b)) {
        return false;
      }

      if (!this.ignore.equals(o.ignore)) {
        return false;
      }

      return true;
    }

    @Override
    public String toString() {
      final StringBuilder b = new StringBuilder();

      b.append("C");
      b.append("(");
      b.append("shared=");
      b.append(this.shared.toString());
      b.append(", ");
      b.append("shared_ignore=");
      b.append(this.sharedIgnore.toString());
      b.append(", ");
      b.append("b=");
      b.append(this.b.toString());
      b.append(", ");
      b.append("ignore=");
      b.append(this.ignore.toString());
      b.append(")");

      return b.toString();
    }

    public static class Builder {
      private Optional<String> shared = Optional.empty();
      private Optional<String> sharedIgnore = Optional.empty();
      private Optional<String> b = Optional.empty();
      private Optional<String> ignore = Optional.empty();

      public Builder shared(final String shared) {
        this.shared = Optional.of(shared);
        return this;
      }

      public Builder sharedIgnore(final String sharedIgnore) {
        this.sharedIgnore = Optional.of(sharedIgnore);
        return this;
      }

      public Builder b(final String b) {
        this.b = Optional.of(b);
        return this;
      }

      public Builder ignore(final String ignore) {
        this.ignore = Optional.of(ignore);
        return this;
      }

      public C build() {
        final String shared = this.shared.orElseThrow(() -> new RuntimeException("shared: is required"));
        final Optional<String> sharedIgnore = this.sharedIgnore;
        final String b = this.b.orElseThrow(() -> new RuntimeException("b: is required"));
        final Optional<String> ignore = this.ignore;

        return new C(shared, sharedIgnore, b, ignore);
      }
    }
  }

  public static class Deserializer extends JsonDeserializer<RequiredFields> {
    @Override
    public RequiredFields deserialize(final JsonParser parser, final DeserializationContext context) throws IOException {
      final ObjectNode object = parser.readValueAs(ObjectNode.class);

      final Set<String> shared = new HashSet<String>();
      shared.add("shared");

      final Set<String> tags = new HashSet<String>();
      final Iterator<String> it = object.fieldNames();
      while (it.hasNext()) {
        tags.add(it.next());
      }

      final Set<String> compared = new HashSet<String>();

      compared.clear();
      compared.add("shared");
      compared.add("a");
      compared.add("b");

      if (tags.containsAll(compared)) {
        return new TreeTraversingParser(object, parser.getCodec()).readValueAs(RequiredFields.A.class);
      }

      compared.clear();
      compared.add("shared");
      compared.add("a");

      if (tags.containsAll(compared)) {
        return new TreeTraversingParser(object, parser.getCodec()).readValueAs(RequiredFields.B.class);
      }

      compared.clear();
      compared.add("shared");
      compared.add("b");

      if (tags.containsAll(compared)) {
        return new TreeTraversingParser(object, parser.getCodec()).readValueAs(RequiredFields.C.class);
      }

      throw context.mappingException("no legal combination of fields available");
    }
  }
}
