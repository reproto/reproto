package heroic.v1;

import heroic.common.Date;
import java.util.Map;
import java.util.Objects;
import java.util.Optional;

public class Query {
  private final Optional<String> query;
  private final Optional<Aggregation> aggregation;
  private final Optional<Date> date;
  private final Optional<Map<String, String>> parameters;

  public Query(
    final Optional<String> query, final Optional<Aggregation> aggregation, final Optional<Date> date, final Optional<Map<String, String>> parameters
  ) {
    Objects.requireNonNull(query, "query");
    this.query = query;
    Objects.requireNonNull(aggregation, "aggregation");
    this.aggregation = aggregation;
    Objects.requireNonNull(date, "date");
    this.date = date;
    Objects.requireNonNull(parameters, "parameters");
    this.parameters = parameters;
  }

  public Optional<String> getQuery() {
    return this.query;
  }

  public Optional<Aggregation> getAggregation() {
    return this.aggregation;
  }

  public Optional<Date> getDate() {
    return this.date;
  }

  public Optional<Map<String, String>> getParameters() {
    return this.parameters;
  }

  @Override
  public int hashCode() {
    int result = 1;
    result = result * 31 + this.query.hashCode();
    result = result * 31 + this.aggregation.hashCode();
    result = result * 31 + this.date.hashCode();
    result = result * 31 + this.parameters.hashCode();
    return result;
  }

  @Override
  public boolean equals(final Object other) {
    if (other == null) {
      return false;
    }

    if (!(other instanceof Query)) {
      return false;
    }

    @SuppressWarnings("unchecked")
    final Query o = (Query) other;

    if (!this.query.equals(o.query)) {
      return false;
    }

    if (!this.aggregation.equals(o.aggregation)) {
      return false;
    }

    if (!this.date.equals(o.date)) {
      return false;
    }

    if (!this.parameters.equals(o.parameters)) {
      return false;
    }

    return true;
  }

  @Override
  public String toString() {
    final StringBuilder b = new StringBuilder();

    b.append("Query");
    b.append("(");
    b.append("query=");
    b.append(this.query.toString());
    b.append(", ");
    b.append("aggregation=");
    b.append(this.aggregation.toString());
    b.append(", ");
    b.append("date=");
    b.append(this.date.toString());
    b.append(", ");
    b.append("parameters=");
    b.append(this.parameters.toString());
    b.append(")");

    return b.toString();
  }

  public static class Builder {
    private Optional<String> query = Optional.empty();
    private Optional<Aggregation> aggregation = Optional.empty();
    private Optional<Date> date = Optional.empty();
    private Optional<Map<String, String>> parameters = Optional.empty();

    public Builder query(final String query) {
      this.query = Optional.of(query);
      return this;
    }

    public Builder aggregation(final Aggregation aggregation) {
      this.aggregation = Optional.of(aggregation);
      return this;
    }

    public Builder date(final Date date) {
      this.date = Optional.of(date);
      return this;
    }

    public Builder parameters(final Map<String, String> parameters) {
      this.parameters = Optional.of(parameters);
      return this;
    }

    public Query build() {
      final Optional<String> query = this.query;
      final Optional<Aggregation> aggregation = this.aggregation;
      final Optional<Date> date = this.date;
      final Optional<Map<String, String>> parameters = this.parameters;

      return new Query(query, aggregation, date, parameters);
    }
  }
}
