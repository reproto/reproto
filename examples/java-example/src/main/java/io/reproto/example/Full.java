package io.reproto.example;

import com.fasterxml.jackson.databind.ObjectMapper;
import ex.github.gists.v3.Gist;
import ex.github.service.v3.Github;
import io.reproto.JacksonSupport;
import okhttp3.OkHttpClient;

public class Full {
    public static void main(String[] argv) throws Exception {
        final OkHttpClient client = new OkHttpClient.Builder().build();

        final ObjectMapper m = JacksonSupport.objectMapper();

        try (final Github.OkHttp github = new Github.OkHttpBuilder(client).mapper(m).build()) {
            System.out.println(github.getRateLimit().get());

            for (final Gist g : github.getUserGists("udoprog").get()) {
                System.out.println(g);
            }
        }
    }
}
