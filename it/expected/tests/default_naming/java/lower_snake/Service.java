package lower_snake;

import java.util.concurrent.CompletableFuture;

public interface Service {
  CompletableFuture<Void> fooBar();
}
