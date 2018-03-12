package test;

import java.util.Optional;
import java.util.concurrent.CompletableFuture;
import okhttp3.OkHttpClient;

public interface MyService {
  /**
   * <pre>
   * UNKNOWN
   * </pre>
   */
  CompletableFuture<Void> unknown();

  /**
   * <pre>
   * UNKNOWN
   * </pre>
   */
  CompletableFuture<Entry> unknownReturn();

  /**
   * <pre>
   * UNKNOWN
   * </pre>
   */
  CompletableFuture<Void> unknownArgument(final Entry request);

  /**
   * <pre>
   * UNARY
   * </pre>
   */
  CompletableFuture<Entry> unary(final Entry request);

  /**
   * <pre>
   * SERVER_STREMAING
   * </pre>
   */
  CompletableFuture<Entry> serverStreaming(final Entry request);

  /**
   * <pre>
   * CLIENT_STREAMING
   * </pre>
   */
  CompletableFuture<Entry> clientStreaming(final Entry request);

  /**
   * <pre>
   * BIDI_STREAMING
   * </pre>
   */
  CompletableFuture<Entry> bidiStreaming(final Entry request);

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
    public CompletableFuture<Void> unknown() {
      throw new RuntimeException("endpoint does not support HTTP");
    }

    @Override
    public CompletableFuture<Entry> unknownReturn() {
      throw new RuntimeException("endpoint does not support HTTP");
    }

    @Override
    public CompletableFuture<Void> unknownArgument(final Entry request) {
      throw new RuntimeException("endpoint does not support HTTP");
    }

    @Override
    public CompletableFuture<Entry> unary(final Entry request) {
      throw new RuntimeException("endpoint does not support HTTP");
    }

    @Override
    public CompletableFuture<Entry> serverStreaming(final Entry request) {
      throw new RuntimeException("endpoint does not support HTTP");
    }

    @Override
    public CompletableFuture<Entry> clientStreaming(final Entry request) {
      throw new RuntimeException("endpoint does not support HTTP");
    }

    @Override
    public CompletableFuture<Entry> bidiStreaming(final Entry request) {
      throw new RuntimeException("endpoint does not support HTTP");
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
