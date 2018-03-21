package test;

import io.reproto.Observer;
import java.io.IOException;
import java.util.Optional;
import okhttp3.Call;
import okhttp3.Callback;
import okhttp3.HttpUrl;
import okhttp3.OkHttpClient;
import okhttp3.Request;
import okhttp3.Response;

public interface MyService {
  /**
   * <pre>
   * UNKNOWN
   * </pre>
   */
  Observer<Void> unknown(final int id);

  /**
   * <pre>
   * UNKNOWN
   * </pre>
   */
  Observer<Entry> unknownReturn(final int id);

  /**
   * <pre>
   * UNKNOWN
   * </pre>
   */
  Observer<Void> unknownArgument(final Entry request, final int id);

  /**
   * <pre>
   * UNARY
   * </pre>
   */
  Observer<Entry> unary(final Entry request, final int id);

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
    public Observer<Void> unknown(final int id) {
      final HttpUrl url_ = this.baseUrl.newBuilder()
        .addPathSegment("unknown")
        .addPathSegment(Integer.toString(id))
        .build();

      final Request req_ = new Request.Builder()
        .url(url_)
        .method("GET", null)
        .build();

      final Observer<Void> future_ = new Observer<Void>();

      this.client.newCall(req_).enqueue(new Callback() {
        @Override
        public void onFailure(final Call call, final IOException e) {
          future_.completeExceptionally(e);
        }

        @Override
        public void onResponse(final Call call, final Response response) {
          if (!response.isSuccessful()) {
            future_.completeExceptionally(new IOException("bad response: " + response));
          } else {
            future_.complete(null);
          }
        }
      });

      return future_;
    }

    @Override
    public Observer<Entry> unknownReturn(final int id) {
      final HttpUrl url_ = this.baseUrl.newBuilder()
        .addPathSegment("unknown-return")
        .addPathSegment(Integer.toString(id))
        .build();

      final Request req_ = new Request.Builder()
        .url(url_)
        .method("GET", null)
        .build();

      final Observer<Entry> future_ = new Observer<Entry>();

      this.client.newCall(req_).enqueue(new Callback() {
        @Override
        public void onFailure(final Call call, final IOException e) {
          future_.completeExceptionally(e);
        }

        @Override
        public void onResponse(final Call call, final Response response) {
          if (!response.isSuccessful()) {
            future_.completeExceptionally(new IOException("bad response: " + response));
          } else {
            future_.complete(null);
          }
        }
      });

      return future_;
    }

    @Override
    public Observer<Void> unknownArgument(final Entry request, final int id) {
      final HttpUrl url_ = this.baseUrl.newBuilder()
        .addPathSegment("unknown-argument")
        .addPathSegment(Integer.toString(id))
        .build();

      final Request req_ = new Request.Builder()
        .url(url_)
        .method("GET", null)
        .build();

      final Observer<Void> future_ = new Observer<Void>();

      this.client.newCall(req_).enqueue(new Callback() {
        @Override
        public void onFailure(final Call call, final IOException e) {
          future_.completeExceptionally(e);
        }

        @Override
        public void onResponse(final Call call, final Response response) {
          if (!response.isSuccessful()) {
            future_.completeExceptionally(new IOException("bad response: " + response));
          } else {
            future_.complete(null);
          }
        }
      });

      return future_;
    }

    @Override
    public Observer<Entry> unary(final Entry request, final int id) {
      final HttpUrl url_ = this.baseUrl.newBuilder()
        .addPathSegment("foo")
        .addPathSegment(Integer.toString(id))
        .build();

      final Request req_ = new Request.Builder()
        .url(url_)
        .method("GET", null)
        .build();

      final Observer<Entry> future_ = new Observer<Entry>();

      this.client.newCall(req_).enqueue(new Callback() {
        @Override
        public void onFailure(final Call call, final IOException e) {
          future_.completeExceptionally(e);
        }

        @Override
        public void onResponse(final Call call, final Response response) {
          if (!response.isSuccessful()) {
            future_.completeExceptionally(new IOException("bad response: " + response));
          } else {
            future_.complete(null);
          }
        }
      });

      return future_;
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
