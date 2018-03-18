package test;

import java.util.Optional;
import java.util.concurrent.CompletableFuture;
import okhttp3.HttpUrl;
import okhttp3.OkHttpClient;
import okhttp3.Request;

public interface MyService {
  /**
   * <pre>
   * UNKNOWN
   * </pre>
   */
  CompletableFuture<Void> unknown(final int id);

  /**
   * <pre>
   * UNKNOWN
   * </pre>
   */
  CompletableFuture<Entry> unknownReturn(final int id);

  /**
   * <pre>
   * UNKNOWN
   * </pre>
   */
  CompletableFuture<Void> unknownArgument(final Entry request, final int id);

  /**
   * <pre>
   * UNARY
   * </pre>
   */
  CompletableFuture<Entry> unary(final Entry request, final int id);

  public class OkHttp implements MyService {
    private final OkHttpClient client;
    private final Optional<String> baseUrl;

    public OkHttp(
      final OkHttpClient client,
      final Optional<String> baseUrl
    ) {
      this.client = client;
      this.baseUrl = baseUrl;
    }

    @Override
    public CompletableFuture<Void> unknown(final int id) {
      final HttpUrl url = new HttpUrl.Builder()
        .addPathSegment("unknown")
        .addPathSegment(Integer.toString(id))
        .build();
      new Request.Builder()
        .url(url)
        .build();
      throw new IllegalStateException("not implemented");
    }

    @Override
    public CompletableFuture<Entry> unknownReturn(final int id) {
      final HttpUrl url = new HttpUrl.Builder()
        .addPathSegment("unknown-return")
        .addPathSegment(Integer.toString(id))
        .build();
      new Request.Builder()
        .url(url)
        .build();
      throw new IllegalStateException("not implemented");
    }

    @Override
    public CompletableFuture<Void> unknownArgument(final Entry request, final int id) {
      final HttpUrl url = new HttpUrl.Builder()
        .addPathSegment("unknown-argument")
        .addPathSegment(Integer.toString(id))
        .build();
      new Request.Builder()
        .url(url)
        .build();
      throw new IllegalStateException("not implemented");
    }

    @Override
    public CompletableFuture<Entry> unary(final Entry request, final int id) {
      final HttpUrl url = new HttpUrl.Builder()
        .addPathSegment("foo")
        .addPathSegment(Integer.toString(id))
        .build();
      new Request.Builder()
        .url(url)
        .build();
      throw new IllegalStateException("not implemented");
    }
  }

  public static class OkHttpBuilder {
    private Optional<String> baseUrl = Optional.empty();
    private final OkHttpClient client;

    public OkHttpBuilder(
      final OkHttpClient client
    ) {
      this.client = client;
    }

    public OkHttpBuilder baseUrl(final String baseUrl) {
      this.baseUrl = Optional.of(baseUrl);
      return this;
    }

    public OkHttp build() {
      return new OkHttp(client, this.baseUrl);
    }
  }
}
