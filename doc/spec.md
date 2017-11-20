# reproto specification

See [TODO](todo.md) for things that are work in progress.

* [Introduction](#introduction)
* [Manifests](#manifests)
* [File Structure](#file-structure)
* [Distribution](#distribution)
* [Specifications](#specifications)
  * [Versioned Specifications](#versioned-specifications)
  * [Ephemeral Specifications](#ephemeral-specifications)
* [Imports](#imports)
* [Built-In Types](#built-in-types)
* [Types](#types)
* [Interfaces](#interfaces)
* [Tuples](#tuples)
* [Enums](#enums)
* [Services](#services)
* [Reserved fields](#reserved-fields)
* [Extensions](#extensions)
* [Custom Code](#custom-code)

## Introduction

reproto is geared towards being an expressive and productive protocol specification.

The choice of using a DSL over something existing like JSON or YAML is an attempt to improve
signal-to-noise ratio.
Concise markup, and relatively intuitive syntax should hopefully mean that more effort can be spent
on designing good data models.

The following is an example specification for a simple time-series database:

```reproto
tuple Sample {
  timestamp: u64;
  value: double;
}

type Graph {
  samples: [Sample];
}

interface System {
  requests_per_second: Graph;

  WebServer {
    name "web-server";

    last_logged_in: c.User;
  }

  Database {
    name "database";

    transactions: Graph;
  }
}

type GraphsRequest {
  systems: [string];
}

type GraphsResponse {
  systems: [System];
}
```

When compiled, the generated objects can be used to serialize, and de-serialize models.
Like with the following example using [`fasterxml`][fasterxml].

```java
final ObjectMapper m = new ObjectMapper();

final GraphsRequest request =
  GraphsRequest.builder().systems(ImmutableList.of("database")).build();

final byte[] response = request(m.writeValueAsBytes(request));

final GraphsResponse response =
  m.readValue(message, GraphsResponse.class);
```

[fasterxml]: https://github.com/FasterXML/jackson-databind

## Manifests

reproto supports loading project manifests describing what should be built.

These can be stored with the project, and describes how and what should be built.

```toml
language = "java"

# Additional build paths, relative to this manifest.
paths = [
    "src/extra"
]

# Path to put generated sources
output = "target/generated"

# Packages to build.
[packages]
petstore = "*"

# Add a preset
[[presets]]
type = "maven"
```

### Packages

The `[packages]` section designate which packages should be built on `reproto build`.

```toml
[packages]
petstore = "*"
```

This can be specified in a more elaborate format to support more options:

```toml
[packages]
petstore = {version = "*"}
```

Or:

```toml
[packages.petstore]
version = "*"
```

### Files

The `[files]` section permits building a single, local file as some specific package and version.
This would typically be used to patch external manifests:

```toml
[files]
petstore = "patches/petstore.reproto"
```

This can be specified in a more elaborate format to support more options:

```toml
[files]
petstore = {path = "patches/petstore.reproto", version = "1.0.1"}
```

Or:

```toml
[files.petstore]
path = "patches/petstore.reproto"
version = "1.0.1"
```

### Publishing

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

Assuming you have a specification in `src/io/reproto/petstore.reproto`, you can now publish it
using:

```bash
$> reproto publish
```

Additional specifications can be added to `src/io/reproto`, and they will also be published with
the same version.

### Presets

Presets are bundles of configuration that can be activated through the `presets` key.

Activated presets are determined by their `type`.

The available types and their corresponding options are documented in this section.

### Doc

The `doc` keys control how documentation is generated:

```
[doc]
# See available themes with `reproto doc --list-syntax-themes`
syntax_theme = "ayu-mirage"
```

#### Maven `type = "maven"`

This preset is equivalent to the following manifest:

```toml
paths = ["src/main/reproto"]
output = "target/generated/reproto/java"
```

## File Structure

The compiler expects that multiple _paths_ are provided to it.

For each path, it expected the following package structure (example with package `foo.bar.baz`):

```
foo/bar/baz.reproto
foo/bar/baz-1.0.0.reproto
foo/bar/baz-1.0.1-beta1.reproto
```

Note that the file may be suffixed with a version number.
If this is present it is called a [versioned specification](#versioned-specifications).

Otherwise, it is known as an [ephemeral specification](#ephemeral-specifications).

### File Options

#### `endpoint_naming <naming>`

The default endpoint naming strategy to use.

Given a specification like the following, reproto needs to determine what to name the endpoints:

```reproto
service MyService {
  /// Put a Foo.
  put_foo(Foo);

  /// Get a Baz.
  get_baz() -> Baz;
}
```

With the default naming strategy, this would result in endpoints named `put_foo` and `get_baz`.

This option changes what endpoints are named by default.

Valid options are:

* `lower_camel`, fields would be named as `lowerCamel`.
* `upper_camel`, fields would be named as `UpperCamel`.
* `upper_snake`, fields would be named as `UPPER_SNAKE`.
* `lower_snake`, fields would be named as `lower_snake` (default).

This does _not_ affect explicitly named endpoinds using `as`.

```reproto
field_naming upper_camel;

service MyService {
  /// Would be named `put_foo`.
  put_foo(Foo) as "put_foo";

  /// Would be named `GET_BAZ`.
  get_baz() -> Baz;
}
```

#### `field_naming <naming>`

The default field naming strategy to use.

This option changes the format that a field will take, depending on its name.

Valid options are:

* `lower_camel`, fields would be serialized as `lowerCamel`.
* `upper_camel`, fields would be serialized as `UpperCamel`.
* `upper_snake`, fields would be serialized as `UPPER_SNAKE`.
* `lower_snake`, fields would be serialized as `lower_snake` (default).

## Specifications

A specification is a UTF-8 encoded file containing declarations.

Conceptually specifications belong to a package, and can have a version.

Specifications without a version are called _ephemeral_ specifications.

## Distribution

**WIP: this feature is not finished**

Specifications are intended to be distributed.

This can be done by uploading a specification to a repository, after which it can be pulled in for
use by other projects through reproto's repository system.

### Versioned specifications

A versioned specification is one that has a version in its filename.

The version string follows [Semantic Versioning][semver], but the following is a brief
description of what is permitted.

The version number must follow semantic versioning. For example, `1.2.0`.

Pre-releases are also supported by appending a hyphen and a series of dot-separated identifiers.
For example, `1.2.1-beta1`.

[semver]: https://semver.org

### Ephemeral specifications

An ephemeral specification is one that does _not_ have a version in its filename.

They can be used as a compiler target (e.g. `--package foo`), but can not be deployed to
a repository.

## Imports

Declarations can be imported from other specifications using the `use` keyword at the top of your
specification.

This may also include a local alias for the imported specification.

```
use foo.bar as b;
```

A version requirement may also be present during the import.

```
use foo.bar@^1 as b1;
use foo.bar@>=2.0.0 as b2;
```

If a version requirement is absent, the most recent version will be picked.

A full list of supported specification is documented in the [`semver` package
documentation][semver-package-requirements].

Note that multiple versions of the same package may be imported.

This would typically cause naming conflicts in most target languages, reproto addresses this by
building packages that are named according to which major version of the specification is used.

The following are a few examples for Java:

* `petstore-1.0.0`, would have the package `petstore.v1`.
* `petstore-0.1.0`, would have the package `petstore._0_1_0`.
* `petstore-0.0.1`, would have the package `petstore._0_0_1`.
* `petstore-0.0.1-alpha1`, would have the package `petstore._0_0_1`.

[semver-package-requirements]: https://docs.rs/semver/0.7.0/semver/#requirements

## Built-In Types

There are a number of built-in types available:

| Type               | Description |
|--------------------|-------------|
| `u32`, `u64`       | Unsigned integer values which can store a given number of bits |
| `i32`, `i64`       | Signed integer values which can store a given number of bits |
| `double`, `float`  | Floating point precision numbers |
| `string`           | UTF-8 encoded strings |
| `datetime`         | ISO-8601 dates encoded as strings. Combined date and time with timezone. |
| `bytes`            | Arbitrary byte-arrays, are encoded as base64-strings in JSON |
| `boolean`          | Boolean values, `true` or `false` |
| `[<type>]`         | Arrays which store the given type  |
| `{<type>: <type>}` | Associations with the given key and value (note: the `<type>` of the key currently _must_ be `string` due to limitations in JSON, but might be subject to change if other formats are supported in the future) |

## Types

Types are named types that are used to designate a data structure that is intended to be
serialized.

Types have a name which must be unique for the package in which it is defined.

The following is an example type declaration:

```reproto
type Foo {
    foo: string;
    bar: i32;
}
```

Types are encoded as objects.

For example (using `Foo`):

```json
{"bar": 42}
```

## Interfaces

Interfaces are special types providing field-based polymorphism.

Each interface lists all the types that it contains in the declaration.

The following is an example interface with two sub-types.

```reproto
/// Describes how a time series should be sampled.
///
/// Sampling is when a time series which is very dense is samples to reduce its size.
interface Sampling {
    /// size of the sample.
    sample_size: u32;
    /// unit of the sample.
    sample_unit: Unit;

    /// Take the average value for each sample.
    Average {
        name "average";
    }

    /// Take the first value encountered for each sample.
    First {
        name "first";
    }

    /// Take the last value encountered for each sample.
    Last {
        name "last";
    }

    /// Take the value which is in the given percentile for each sample.
    Percentile {
        name "percentile";

        /// Which percentile to sample, as a value between 0-1.0
        percentile: float;
    }
}

enum Unit: string {
     MILLISECONDS = "ms";
     SECONDS = "s";
     HOURS = "H";
     DAYS = "d";
     WEEKS = "w";
}
```

An interface is encoded as an object, with a special `type` field.

For example (using `new Sampling.Average(10, Unit.SECONDS)`):

```json
{
    "type": "average",
    "sample_size": 10,
    "sample_unit": "s"
}
```

The following options are supported by interfaces:

#### `type_info <identifier>`

Indicates the method of transferring type information.
Valid options are:

* `type_field` sub-types are serialized as objects, with a special field (given by
  `type_field_name`) containing its `name`.
* `array` sub-types will be serialized as arrays, where the first value is the `name`.
* `object_keys` sub-types will be serialized as objects with a single
    key, where the key is the `name` |

#### `type_field_name <string>`

Name of the type field indicating which sub-type it is.
This Option is only valid when `type_info type_field` is set. |

## Tuples

Tuples are sequences of data, where each element has a known type.

```reproto
tuple Sample {
  time: u64;
  value: double;
}
```

All fields in a tuple are required, and are presented in the order that the field occurs in the sequence.

A single sample (e.g. `new Sample(1, 2.0)`) would be encoded like this in JSON:

```json
[1, 2.0]
```

## Enums

Enums can take on of a given set of constant values.

```reproto
enum SI as string {
    NANO as "nano";
    MICRO as "micro";
    MILLI as "milli";
    KILO as "kilo";
    MEGA as "mega";
}
```

Using this, `SI.NANO` would be serialized as:

```json
"nano"
```

## Services

Service declarations describe a set of endpoints being exposed by a service.

Services are declared using the `service` keyword.

```reproto
/// My Service>
service MyService {
}
```

Inside of a service, endpoints can be declared.

Every endpoints must have a unique name.

```reproto
type Foo {
}

/// My Service.
service MyService {
  /// Get foo.
  get_foo() -> Foo;

  /// Set foo.
  set_foo(Foo);
}
```

Endpoints can have a set of options associated with them, by expanding their body.

These options might affect how code generation works for certain backends.

```reproto
type Foo {
}

/// My Service.
service MyService {
  /// Get foo.
  get_foo() -> Foo {
    http_status 200;
  }

  /// Set foo.
  set_foo(Foo);
}
```

Requests, responses, or both can be streamed. This permits sending multiple requests or multiple
responses.

You mark this relationship with the `stream` keyword.

```reproto
service MyService {
  /// Get many foos.
  get_foos() -> stream Foo;

  /// Write many foos.
  write_foos(stream Foo);
}
```

Endpoints can be explicitly named with the `as` keyword.

```reproto
service MyService {
  /// Get many foos.
  get_foos() -> stream Foo as "get_bars";
}
```

## Reserved fields

Fields can be reserved using a special option called `reserved`.
Fields which are reserved _cannot_ be added to the schema.

Attempting to do so will yield an error like the following:

```bash
examples/petstore.reproto:55:3-21:
 55:   no_can_do: string;
       ^^^^^^^^^^^^^^^^^^ - field reserved
examples/petstore.reproto:49:12-21:
 49:   reserved no_can_do;
       ^^^^^^^^^^^^^^^^^^^ - field reserved here
```

As long as the reserved statement is preserved, it prevents future introductions of a given field.

Clients decoding a reserved field should raise an error.

## Extensions

reproto permits all types and interfaces to be extended.

Extensions allow for additions, and is typically used to adapt a protocol specification to your
local environment.
They allow you to add additional information, as long as it doesn't conflict with any existing
declarations.

In a perfect world, extensions should not be necessary and the specification should be in sync with
the API, and there should be no additional configuration necessary to start using the generated
code.

Extensions may only be loaded through the `[files]` section in the manifest.

Assume you have a type called `Foo` in the `foo` package:

```reproto
// file: protos/foo.reproto
type Foo {
  field: string;
}
```

You can now add extend existing types by specifying the following as an extension:

```reproto
// file: ext/foo.reproto
type Foo {
  other?: string;

  java {{
    public boolean hasOther() {
      return this.other.isPresent();
    }
  }}
}
```

## Custom Code

A powerful mechanism for modifying the behaviour of your protocols is to embed code snippets.
This should _primarily_ be done in [extensions](extensions), to adapt a given set of protocols to
your application.

```reproto
type Foo {
  field: string;

  java {{
    public boolean isFieldOk() {
      return this.field.equals("ok");
    }
  }}

  python {{
    def is_field_ok(self):
      return self.field == "ok"
  }}
}
```
