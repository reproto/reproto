# Build manifests

 * [Build paths](#build-paths)
 * [Package discovery](#package-discovery)
 * [`packages` section](#packages)
 * [`files` section](#files-section)
 * [`publish` section](#publish-section)
 * [`presets` section](#presets-section)
   * [`maven` preset](#maven-preset)
   * [`swift` preset](#swift-preset)
   * [`go` preset](#go-preset)
 * [`doc` section](#doc)

You tell `reproto` what to do by writing build manifests.
The default build manifest that reproto looks for is `reproto.toml` in the current directory.
These are stored with the project, and describe _what_ should be built.

The following is an example manifest:

```toml
# File: reproto.toml

language = "java"

# Use a maven preset.
[presets.maven]
```

This specifies that reproto should built _any_ available version of the toystore package, suitable
for a [Maven project].

[Maven project]: #maven-preset

## Build paths

Build paths is specified under the `paths` key.
These are the directories where reproto will look for local specification when they are [built],
imported, or [published].

```toml
paths = ["src"]
```

[built]: #package-section
[published]: #publish-section
[ephemeral specifications]: spec.md#ephemeral-specifications

## Package discovery

reproto supports automatically identifying packages from your build paths.

Any `.reproto` file discovered in a build path will be treated as a reproto specification, whose
package corresponds to the _path_ that the package is located in.

For example, assume we have the following structure.

Manifest:

```toml
paths = [
  "src/main/reproto",
  "reproto"
]
```

Files:

* `src/main/reproto/io/reproto/example.reproto`
* `src/main/reproto/io/reproto/other-0.0.1.reproto`
* `proto/io/reproto/example-2.0.1.reproto`

The build system would then identify the following packages to be built:

* `io.reproto.example` (no version)
* `io.reproto.other-0.0.1`
* `io.reproto.example-2.0.1`

## `packages` section

The `[packages]` section is an optional section which can be used to designate which packages
should be built by reproto.

```toml
[packages]
"io.reproto.toystore" = "*"
```

This can be specified in a more elaborate format to support more options:

```toml
[packages]
"io.reproto.toystore" = {version = "*"}
```

Or:

```toml
[packages."io.reproto.toystore"]
version = "*"
```

## `files` section

The `[files]` section permits building a single, local file as some specific package and version.
This would typically be used to patch external manifests:

```toml
[files]
"io.reproto.toystore" = "patches/toystore.reproto"
```

This can be specified in a more elaborate format to support more options:

```toml
[files]
"io.reproto.toystore" = {path = "patches/toystore.reproto", version = "1.0.1"}
```

Or:

```toml
[files."io.reproto.toystore"]
path = "patches/toystore.reproto"
version = "1.0.1"
```

## `publish` section

In order to publish packages, the version of the package needs to be known.

Since specifications would typically be unversioned during development, reproto supports
a `[publish]` section where you can map what version a local specification belongs to.

The package specified is a prefix. The version will apply to any contained packages.

```toml
paths = ["src"]

[publish]
"io.reproto" = "1.0.1"
```

These can be specified in a more elaborate format:

```toml
[publish]
"io.reproto" = {version = "1.0.1"}
```

Or:

```toml
[publish."io.reproto"]
version = "1.0.1"
```

Assuming you have a specification in `src/io/reproto/toystore.reproto`, you can now publish it
using:

```bash
$> reproto publish
```

Additional specifications can be added to `src/io/reproto`, and they will also be published with
the same version.

## `presets` section

Presets are bundles of configuration that can be activated through the `presets` key.

Activated presets are determined by their `type`.

The available types and their corresponding options are documented in this section.

### `maven` preset

Sets default options suitable for building with a default Maven project.

```toml
# File: reproto.toml

[presets.maven]
```

This preset is equivalent to the following manifest:

```toml
# File: reproto.toml

paths = ["src/main/reproto"]
output = "target/generated/reproto/java"
```

### `swift` preset

Sets default options suitable for building a Swift project.

```toml
# File: reproto.toml

[presets.swift]
```

This preset is equivalent to the following manifest:

```toml
# File: reproto.toml

paths = ["proto"]
output = "Sources/Modules"
```

### `go` preset

Sets default options suitable for building a Go project.

```toml
# File: reproto.toml

[presets.go]
```

This preset is equivalent to the following manifest:

```toml
# File: reproto.toml

paths = ["reproto"]
output = "modules"
```

## `doc`

The `doc` keys control how documentation is generated:

```toml
[doc]
# See available themes with `reproto doc --list-themes`.
theme = "light"

# See available themes with `reproto doc --list-syntax-themes`.
syntax_theme = "ayu-mirage"
```

