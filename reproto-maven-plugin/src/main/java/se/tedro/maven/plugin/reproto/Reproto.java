package se.tedro.maven.plugin.reproto;

import java.nio.file.Path;
import java.util.ArrayList;
import java.util.List;
import java.util.Optional;
import lombok.NonNull;
import lombok.RequiredArgsConstructor;
import org.apache.maven.plugin.logging.Log;
import org.codehaus.plexus.util.cli.CommandLineUtils;
import org.codehaus.plexus.util.cli.Commandline;
import org.codehaus.plexus.util.cli.StreamConsumer;

@RequiredArgsConstructor
public class Reproto {
  private final Path executable;
  private final Path out;
  private final List<Path> paths;
  private final List<String> modules;
  private final List<String> targets;
  private final Optional<String> packagePrefix;

  public void execute(final Log log) throws Exception {
    final LineConsumer output = new LineConsumer();
    final LineConsumer error = new LineConsumer();

    final Commandline command = new Commandline();

    command.setExecutable(executable.toAbsolutePath().toString());
    command.addArguments(arguments());

    final int result = CommandLineUtils.executeCommandLine(command, null, output, error);

    for (final String line : output.lines) {
      log.info("reproto: " + line);
    }

    for (final String line : output.lines) {
      log.error("reproto: " + line);
    }

    if (result != 0) {
      throw new RuntimeException(executable + ": exited with non-zero status");
    }
  }

  public String[] arguments() {
    final List<String> result = new ArrayList<String>();

    result.add("--backend");
    result.add("java");

    for (final Path path : this.paths) {
      result.add("--path");
      result.add(path.toAbsolutePath().toString());
    }

    for (final String module : this.modules) {
      result.add("--module");
      result.add(module);
    }

    result.add("--out");
    result.add(out.toAbsolutePath().toString());

    packagePrefix.ifPresent(packagePrefix -> {
      result.add("--package-prefix");
      result.add(packagePrefix);
    });

    for (final String target : this.targets) {
      result.add(target);
    }

    return result.toArray(new String[0]);
  }

  @RequiredArgsConstructor
  public static class Builder {
    @NonNull
    private final Path executable;
    @NonNull
    private final Path out;

    private final List<Path> paths = new ArrayList<>();
    private final List<String> modules = new ArrayList<>();
    private final List<String> targets = new ArrayList<>();
    private Optional<String> packagePrefix = Optional.empty();

    public Builder path(final Path path) {
      this.paths.add(path);
      return this;
    }

    public Builder module(final String module) {
      this.modules.add(module);
      return this;
    }

    public Builder target(final String target) {
      this.targets.add(target);
      return this;
    }

    public Builder packagePrefix(final String packagePrefix) {
      this.packagePrefix = Optional.of(packagePrefix);
      return this;
    }

    public Reproto build() {
      return new Reproto(executable, out, new ArrayList<>(paths), new ArrayList<>(modules),
        new ArrayList<>(targets), packagePrefix);
    }
  }

  public static class LineConsumer implements StreamConsumer {
    private final List<String> lines = new ArrayList<String>();

    @Override
    public void consumeLine(final String line) {
      lines.add(line);
    }
  }
}
