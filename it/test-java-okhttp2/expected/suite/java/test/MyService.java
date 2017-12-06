package test;

import java.util.concurrent.CompletableFuture;

public interface MyService {
  CompletableFuture<Void> unknown();

  CompletableFuture<Entry> unknownReturn();

  CompletableFuture<Void> unknownArgument(final Entry request);

  CompletableFuture<Entry> unary(final Entry request);

  CompletableFuture<Entry> serverStreaming(final Entry request);

  CompletableFuture<Entry> clientStreaming(final Entry request);

  CompletableFuture<Entry> bidiStreaming(final Entry request);
}
