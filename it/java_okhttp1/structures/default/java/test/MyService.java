package test;

import io.reproto.Observer;
import java.util.Optional;
import okhttp3.HttpUrl;
import okhttp3.OkHttpClient;

public interface MyService {
  /**
   * <pre>
   * UNKNOWN
   * </pre>
   */
  Observer<Void> unknown();

  /**
   * <pre>
   * UNKNOWN
   * </pre>
   */
  Observer<Entry> unknownReturn();

  /**
   * <pre>
   * UNKNOWN
   * </pre>
   */
  Observer<Void> unknownArgument(final Entry request);

  /**
   * <pre>
   * UNARY
   * </pre>
   */
  Observer<Entry> unary(final Entry request);

  /**
   * <pre>
   * SERVER_STREMAING
   * </pre>
   */
  Observer<Entry> serverStreaming(final Entry request);

  /**
   * <pre>
   * CLIENT_STREAMING
   * </pre>
   */
  Observer<Entry> clientStreaming(final Entry request);

  /**
   * <pre>
   * BIDI_STREAMING
   * </pre>
   */
  Observer<Entry> bidiStreaming(final Entry request);

  public class OkHttp implements MyService {
    private final OkHttpClient client;
    private final HttpUrl baseUrl;

    public OkHttp(
      final OkHttpClient client,
      final HttpUrl baseUrl
    ) {
      this.client = client;
      this.baseUrl = baseUrl;
    }

    @Override
    public Observer<Void> unknown() {
      throw new RuntimeException("endpoint does not support HTTP");
    }

    @Override
    public Observer<Entry> unknownReturn() {
      throw new RuntimeException("endpoint does not support HTTP");
    }

    @Override
    public Observer<Void> unknownArgument(final Entry request) {
      throw new RuntimeException("endpoint does not support HTTP");
    }

    @Override
    public Observer<Entry> unary(final Entry request) {
      throw new RuntimeException("endpoint does not support HTTP");
    }

    @Override
    public Observer<Entry> serverStreaming(final Entry request) {
      throw new RuntimeException("endpoint does not support HTTP");
    }

    @Override
    public Observer<Entry> clientStreaming(final Entry request) {
      throw new RuntimeException("endpoint does not support HTTP");
    }

    @Override
    public Observer<Entry> bidiStreaming(final Entry request) {
      throw new RuntimeException("endpoint does not support HTTP");
    }
  }

  public static class OkHttpBuilder {
    private Optional<HttpUrl> baseUrl = Optional.empty();
    private final OkHttpClient client;

    public OkHttpBuilder(
      final OkHttpClient client
    ) {
      this.client = client;
    }

    public OkHttpBuilder baseUrl(final HttpUrl baseUrl) {
      this.baseUrl = Optional.of(baseUrl);
      return this;
    }

    public OkHttp build() {
      final HttpUrl baseUrl = this.baseUrl.orElseThrow(() -> new RuntimeException("baseUrl: is a required field"));
      return new OkHttp(client, baseUrl);
    }
  }
}
