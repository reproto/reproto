package test;

import com.fasterxml.jackson.databind.ObjectMapper;
import io.reproto.JacksonSupport;
import java.io.Closeable;
import java.io.IOException;
import java.util.Optional;
import java.util.concurrent.CompletableFuture;
import okhttp3.Call;
import okhttp3.Callback;
import okhttp3.HttpUrl;
import okhttp3.OkHttpClient;
import okhttp3.Request;
import okhttp3.Response;

public interface MyService {
  public class OkHttp implements Closeable {
    private final OkHttpClient client;
    private final HttpUrl baseUrl;
    private final ObjectMapper mapper;

    public OkHttp(
      final OkHttpClient client,
      final HttpUrl baseUrl,
      final ObjectMapper mapper
    ) {
      this.client = client;
      this.baseUrl = baseUrl;
      this.mapper = mapper;
    }

    public CompletableFuture<Void> unknown(final int id) {
      final HttpUrl url_ = this.baseUrl.newBuilder()
        .addPathSegment("unknown")
        .addPathSegment(Integer.toString(id))
        .build();

      final Request req_ = new Request.Builder()
        .url(url_)
        .method("GET", null)
        .build();

      final CompletableFuture<Void> future_ = new CompletableFuture<Void>();

      this.client.newCall(req_).enqueue(new Callback() {
        @Override
        public void onFailure(final Call call, final IOException e) {
          future_.completeExceptionally(e);
        }

        @Override
        public void onResponse(final Call call, final Response response) {
          if (!response.isSuccessful()) {
            future_.completeExceptionally(new IOException("bad response: " + response));
            return;
          }

          future_.complete(null);
        }
      });

      return future_;
    }

    public CompletableFuture<Entry> unknownReturn(final int id) {
      final HttpUrl url_ = this.baseUrl.newBuilder()
        .addPathSegment("unknown-return")
        .addPathSegment(Integer.toString(id))
        .build();

      final Request req_ = new Request.Builder()
        .url(url_)
        .method("GET", null)
        .build();

      final CompletableFuture<Entry> future_ = new CompletableFuture<Entry>();

      this.client.newCall(req_).enqueue(new Callback() {
        @Override
        public void onFailure(final Call call, final IOException e) {
          future_.completeExceptionally(e);
        }

        @Override
        public void onResponse(final Call call, final Response response) {
          if (!response.isSuccessful()) {
            future_.completeExceptionally(new IOException("bad response: " + response));
            return;
          }

          final Entry body;

          try {
            body = mapper.readValue(response.body().byteStream(), Entry.class);
          } catch(final Exception e) {
            future_.completeExceptionally(e);
            return;
          }

          future_.complete(body);
        }
      });

      return future_;
    }

    public CompletableFuture<Void> unknownArgument(final Entry request, final int id) {
      final HttpUrl url_ = this.baseUrl.newBuilder()
        .addPathSegment("unknown-argument")
        .addPathSegment(Integer.toString(id))
        .build();

      final Request req_ = new Request.Builder()
        .url(url_)
        .method("GET", private final ObjectMapper mapper.encode(request))
        .build();

      final CompletableFuture<Void> future_ = new CompletableFuture<Void>();

      this.client.newCall(req_).enqueue(new Callback() {
        @Override
        public void onFailure(final Call call, final IOException e) {
          future_.completeExceptionally(e);
        }

        @Override
        public void onResponse(final Call call, final Response response) {
          if (!response.isSuccessful()) {
            future_.completeExceptionally(new IOException("bad response: " + response));
            return;
          }

          future_.complete(null);
        }
      });

      return future_;
    }

    public CompletableFuture<Entry> unary(final Entry request, final int id) {
      final HttpUrl url_ = this.baseUrl.newBuilder()
        .addPathSegment("unary")
        .addPathSegment(Integer.toString(id))
        .build();

      final Request req_ = new Request.Builder()
        .url(url_)
        .method("GET", private final ObjectMapper mapper.encode(request))
        .build();

      final CompletableFuture<Entry> future_ = new CompletableFuture<Entry>();

      this.client.newCall(req_).enqueue(new Callback() {
        @Override
        public void onFailure(final Call call, final IOException e) {
          future_.completeExceptionally(e);
        }

        @Override
        public void onResponse(final Call call, final Response response) {
          if (!response.isSuccessful()) {
            future_.completeExceptionally(new IOException("bad response: " + response));
            return;
          }

          final Entry body;

          try {
            body = mapper.readValue(response.body().byteStream(), Entry.class);
          } catch(final Exception e) {
            future_.completeExceptionally(e);
            return;
          }

          future_.complete(body);
        }
      });

      return future_;
    }

    @Override
    public void close() throws IOException {
      client.dispatcher().executorService().shutdown();

      client.connectionPool().evictAll();

      if (client.cache() != null) {
        client.cache().close();
      }
    }
  }

  public static class OkHttpBuilder {
    private Optional<HttpUrl> baseUrl = Optional.empty();
    private Optional<ObjectMapper> mapper = Optional.empty();
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

    public OkHttpBuilder mapper(final ObjectMapper mapper) {
      this.mapper = Optional.of(mapper);
      return this;
    }

    public OkHttp build() {
      final HttpUrl baseUrl = this.baseUrl.orElseGet(() -> HttpUrl.parse("http://example.com"));
      final ObjectMapper mapper = this.mapper.orElseGet(() -> JacksonSupport.objectMapper());
      return new OkHttp(client, baseUrl, mapper);
    }
  }
}
