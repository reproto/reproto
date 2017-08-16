package service._1_0_0;

import java.util.concurrent.CompletableFuture;

public interface MyService {
  CompletableFuture<Void> v1Foo(final Object request);
}
