# reproto manifest files (`reproto.toml`)

 * [Build Path](#build-path)
 * [`packages` section](#packages)
 * [`files` section](#files)
 * [`publish` section](#publish)
 * [`presets` section](#presets)
 * [`doc` section](#doc)

You tell `reproto` what to do by writing manifests.

The default build manifest that reproto looks for is `reproto.toml` in the current directory.
This can be modified with the `--manifest-path <path>` option.

These are stored with the project, and describe among other things _what_ should be built:

```toml
# File: reproto.toml

language = "java"

# Additional build paths, relative to this manifest.
paths = [
    "src/extra"
]

# Path to put generated sources
output = "target/generated"

# Packages to build.
[packages]
toystore = "*"

# Add a preset
[[presets]]
type = "maven"
```

## Build Path

The build path is specified in the `paths` key.

```toml
paths = ["src"]
```

This determines where the compiler should look for specifications when they are being built or
imported.

## `packages`

The `[packages]` section designate which packages should be built on `reproto build`.

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

## `files`

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

## `publish`

In order to publish packages, the version of the package needs to be known.

Since specifications would typically be unversioned during development, reproto supports
a `[publish]` section where you can map what version local specifications belong to.

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

## `presets`

Presets are bundles of configuration that can be activated through the `presets` key.

Activated presets are determined by their `type`.

The available types and their corresponding options are documented in this section.

### `maven` preset

Sets default options suitable for building with a default Maven project.

```toml
# File: reproto.toml

[[presets]]
type = "maven"
```

This preset is equivalent to the following manifest:

```toml
# File: reproto.toml

paths = ["src/main/reproto"]
output = "target/generated/reproto/java"
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

