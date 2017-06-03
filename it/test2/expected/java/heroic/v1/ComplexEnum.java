package heroic.v1;

import com.google.common.collect.ImmutableList;
import java.util.Objects;

public enum ComplexEnum {
  FIRST(new Sampling(None, 42, None), SI.NANO, new Samples.Points("points", ImmutableList.of())),
  SECOND(new Sampling(None, 9, None), SI.MILLI, new Samples.Points("b", ImmutableList.of()));

  private final Sampling si;
  private final SI other;
  private final Samples samples;

  private ComplexEnum(final Sampling si, final SI other, final Samples samples) {
    Objects.requireNonNull(si, "si");
    this.si = si;
    Objects.requireNonNull(other, "other");
    this.other = other;
    Objects.requireNonNull(samples, "samples");
    this.samples = samples;
  }

  public Sampling getSi() {
    return this.si;
  }

  public SI getOther() {
    return this.other;
  }

  public Samples getSamples() {
    return this.samples;
  }
}
