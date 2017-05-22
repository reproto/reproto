package se.tedro.maven.plugin.reproto;

import java.io.File;
import java.nio.file.Path;
import org.apache.maven.plugins.annotations.LifecyclePhase;
import org.apache.maven.plugins.annotations.Mojo;
import org.apache.maven.plugins.annotations.Parameter;
import org.apache.maven.plugins.annotations.ResolutionScope;

@Mojo(name = "compile", defaultPhase = LifecyclePhase.GENERATE_SOURCES,
  requiresDependencyResolution = ResolutionScope.COMPILE, threadSafe = true)
public class CompileReprotoMojo extends AbstractReprotoMojo {
  @Parameter(required = true,
    defaultValue = "${project.build.directory}/generated-sources/reproto/java")
  private File outputDirectory;

  @Parameter(required = true, defaultValue = "${basedir}/src/main/reproto")
  private File reprotoSourceRoot;

  @Override
  protected Path getOutputDirectory() {
    return outputDirectory.toPath();
  }

  @Override
  protected Path getSourceRoot() {
    return reprotoSourceRoot.toPath();
  }
}
