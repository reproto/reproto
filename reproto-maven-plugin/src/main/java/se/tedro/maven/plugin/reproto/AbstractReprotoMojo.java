package se.tedro.maven.plugin.reproto;

import java.io.File;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.Collections;
import java.util.List;
import java.util.Set;
import org.apache.maven.artifact.Artifact;
import org.apache.maven.artifact.factory.ArtifactFactory;
import org.apache.maven.artifact.repository.ArtifactRepository;
import org.apache.maven.artifact.resolver.ArtifactResolutionRequest;
import org.apache.maven.artifact.resolver.ArtifactResolutionResult;
import org.apache.maven.artifact.resolver.ResolutionErrorHandler;
import org.apache.maven.artifact.versioning.VersionRange;
import org.apache.maven.execution.MavenSession;
import org.apache.maven.plugin.AbstractMojo;
import org.apache.maven.plugin.MojoExecutionException;
import org.apache.maven.plugins.annotations.Component;
import org.apache.maven.plugins.annotations.Parameter;
import org.apache.maven.project.MavenProject;
import org.apache.maven.project.MavenProjectHelper;
import org.apache.maven.repository.RepositorySystem;
import org.codehaus.plexus.util.StringUtils;
import org.sonatype.plexus.build.incremental.BuildContext;

public abstract class AbstractReprotoMojo extends AbstractMojo {
  @Parameter(defaultValue = "${project}", readonly = true)
  private MavenProject project;

  @Parameter(defaultValue = "${session}", readonly = true)
  private MavenSession session;

  @Component
  private MavenProjectHelper projectHelper;

  @Component
  private RepositorySystem repositorySystem;

  @Component
  private ResolutionErrorHandler resolutionErrorHandler;

  @Component
  private ArtifactFactory artifactFactory;

  @Component
  private BuildContext buildContext;

  @Parameter(required = false, property = "reproto.executable")
  private String reprotoExecutable;

  @Parameter(required = false, property = "reproto.artifact")
  private String reprotoArtifact;

  @Parameter(required = true, readonly = true, property = "localRepository")
  private ArtifactRepository localRepository;

  @Parameter(required = true, readonly = true,
    defaultValue = "${project.remoteArtifactRepositories}")
  private List<ArtifactRepository> remoteRepositories;

  /**
   * When {@code true}, skip the execution.
   */
  @Parameter(required = false, property = "reproto.skip", defaultValue = "false")
  private boolean skip;

  /**
   * A directory where native launchers for java protoc plugins will be generated.
   */
  @Parameter(required = false, defaultValue = "${project.build.directory}/reproto-plugins")
  private File reprotoPluginsDirectory;

  /**
   * Package prefix to use when generating packages.
   */
  @Parameter(required = false, property = "reproto.packagePrefix",
    defaultValue = "${project.groupId}")
  private String packagePrefix;

  /**
   * Packages to compile.
   */
  @Parameter(required = true)
  private Set<String> targets = Collections.emptySet();

  /**
   * Modules to enable.
   */
  @Parameter(required = false)
  private List<String> modules = Collections.emptyList();

  /**
   * Override to specify output directory.
   */
  protected abstract Path getOutputDirectory();

  /**
   * Override to specify source directory.
   */
  protected abstract Path getSourceRoot();

  /**
   * Check if execution should be skipped.
   */
  private boolean isSkipped() {
    if (skip) {
      getLog().info("Skipping execution");
      return true;
    }

    if ("pom".equals(this.project.getPackaging())) {
      getLog().info("Skipping mojo execution for packaging 'pom'");
      return true;
    }

    return false;
  }

  @Override
  public void execute() throws MojoExecutionException {
    try {
      doExecute();
    } catch (final Exception e) {
      throw new MojoExecutionException(e.getMessage(), e);
    }
  }

  private void doExecute() throws Exception {
    if (isSkipped()) {
      return;
    }

    Path executable = null;

    if (this.reprotoExecutable != null) {
      executable = Paths.get(this.reprotoExecutable);
    }

    if (executable == null && reprotoArtifact != null) {
      final Artifact artifact = createDependencyArtifact(reprotoArtifact);
      executable = resolveBinaryArtifact(artifact);
    }

    if (executable == null) {
      throw new IllegalArgumentException(
        "Could not find a reproto executable. Specify either `-Dreproto.executable=<path>` or " +
          "`-Dreproto.artifactId=<artifact>`");
    }

    if (!Files.isExecutable(executable)) {
      throw new IllegalArgumentException("Not executable: " + executable);
    }

    final Reproto.Builder reproto = new Reproto.Builder(executable, getOutputDirectory());

    reproto.path(getSourceRoot());

    for (final String module : modules) {
      reproto.module(module);
    }

    for (final String target : targets) {
      reproto.target(target);
    }

    if (packagePrefix != null && !StringUtils.isBlank(packagePrefix)) {
      reproto.packagePrefix(packagePrefix);
    }

    reproto.build().execute(getLog());

    final Path outputDirectory = getOutputDirectory();

    if (Files.isDirectory(outputDirectory)) {
      project.addCompileSourceRoot(outputDirectory.toAbsolutePath().toString());
      buildContext.refresh(outputDirectory.toFile());
    }
  }

  private Artifact createDependencyArtifact(
    final String groupId, final String artifactId, final String version, final String type,
    final String classifier
  ) throws Exception {
    final VersionRange versionSpec = VersionRange.createFromVersionSpec(version);

    return artifactFactory.createDependencyArtifact(groupId, artifactId, versionSpec, type,
      classifier, Artifact.SCOPE_RUNTIME);
  }

  private Artifact createDependencyArtifact(final String artifactSpec) throws Exception {
    final String[] parts = artifactSpec.split(":");

    if (parts.length < 3 || parts.length > 5) {
      throw new IllegalArgumentException("Invalid artifact specification (" + artifactSpec + "), " +
        "expected: groupId:artifactId:version[:type[:classifier]]");
    }

    final String type = parts.length >= 4 ? parts[3] : "exe";
    final String classifier = parts.length == 5 ? parts[4] : null;
    return createDependencyArtifact(parts[0], parts[1], parts[2], type, classifier);
  }

  private Path resolveBinaryArtifact(final Artifact artifact) throws Exception {
    final ArtifactResolutionResult result;

    final ArtifactResolutionRequest request = new ArtifactResolutionRequest()
      .setArtifact(project.getArtifact())
      .setResolveRoot(false)
      .setResolveTransitively(false)
      .setArtifactDependencies(Collections.singleton(artifact))
      .setManagedVersionMap(Collections.emptyMap())
      .setLocalRepository(localRepository)
      .setRemoteRepositories(remoteRepositories)
      .setOffline(session.isOffline())
      .setForceUpdate(session.getRequest().isUpdateSnapshots())
      .setServers(session.getRequest().getServers())
      .setMirrors(session.getRequest().getMirrors())
      .setProxies(session.getRequest().getProxies());

    result = repositorySystem.resolve(request);

    resolutionErrorHandler.throwErrors(request, result);

    final Set<Artifact> artifacts = result.getArtifacts();

    if (artifacts == null || artifacts.isEmpty()) {
      throw new RuntimeException("Unable to resolve plugin artifact");
    }

    final Artifact resolvedBinaryArtifact = artifacts.iterator().next();

    if (getLog().isDebugEnabled()) {
      getLog().debug("Resolved artifact: " + resolvedBinaryArtifact);
    }

    final Path pluginsDirectory = reprotoPluginsDirectory.toPath();

    final Path sourceFile = resolvedBinaryArtifact.getFile().toPath();
    final Path targetFile = pluginsDirectory.resolve(sourceFile.getFileName());

    Files.createDirectories(pluginsDirectory);
    Files.copy(sourceFile, targetFile);

    if (getLog().isDebugEnabled()) {
      getLog().debug("Executable file: " + targetFile);
    }

    return targetFile;
  }
}
