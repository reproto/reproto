package io.reproto;

import com.fasterxml.jackson.annotation.JsonInclude;
import com.fasterxml.jackson.databind.DeserializationFeature;
import com.fasterxml.jackson.databind.ObjectMapper;
import com.fasterxml.jackson.databind.SerializationFeature;
import com.fasterxml.jackson.datatype.jdk8.Jdk8Module;
import com.fasterxml.jackson.datatype.jsr310.JavaTimeModule;

public interface JacksonSupport {
  /**
   * Build an object mapper which has the required configuration and modules installed.
   */
  public static ObjectMapper objectMapper() {
    final ObjectMapper m = new ObjectMapper();
    m.disable(SerializationFeature.FAIL_ON_EMPTY_BEANS);
    m.disable(DeserializationFeature.FAIL_ON_UNKNOWN_PROPERTIES);
    m.setSerializationInclusion(JsonInclude.Include.NON_ABSENT);
    m.registerModule(new Jdk8Module());
    m.registerModule(new JavaTimeModule());
    return m;
  }
}
