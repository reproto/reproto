package io.reproto;

import com.fasterxml.jackson.annotation.JsonInclude.Include;
import com.fasterxml.jackson.databind.ObjectMapper;
import com.fasterxml.jackson.datatype.jdk8.Jdk8Module;
import com.fasterxml.jackson.datatype.jsr310.JavaTimeModule;

public final class MapperProvider {
  private static volatile ObjectMapper mapper = null;
  private static final Object lock = new Object();

  public static ObjectMapper get() {
    if (mapper != null) {
      return mapper;
    }

    synchronized (lock) {
      if (mapper != null) {
        return mapper;
      }

      final ObjectMapper m = new ObjectMapper();
      m.setSerializationInclusion(Include.NON_ABSENT);
      m.registerModule(new Jdk8Module());
      m.registerModule(new JavaTimeModule());

      MapperProvider.mapper = m;
      return m;
    }
  }
}
