# reproto specification

* [Specifications](#specifications)
* [Directory Structure](#directory-structure)
* [File Options](#file-options)
* [Distribution](#distribution)
* [Versioning](#versioning)
  * [Versioned Specifications](#versioned-specifications)
  * [Ephemeral Specifications](#ephemeral-specifications)
* [The reproto language](#the-reproto-language)
  * [Imports](#imports)
  * [Built-In Types](#built-in-types)
  * [Documentation](#documentation)
  * [Types](#types)
  * [Enums](#enums)
  * [Interfaces](#interfaces)
    * [Interface sub-types](#interface-sub-types)
  * [Tuples](#tuples)
  * [Services](#services)
  * [Reserved fields](#reserved-fields)
  * [Custom Code](#custom-code)
* [Language Support](#language-support)
  * [Java](#java)
  * [Rust](#rust)
  * [Python](#python)
  * [Javascript](#javascript)

# Specifications

Specifications are written in UTF-8, the file ending of reproto specifications must be `.reproto`.

Each specification contains declarations.

The following declarations are currently supported:

 * [`type`], which specifies the structure of a JSON object.
 * [`enum`], which specifies a discrete set of valid string values.
 * [`interface`], which specifies a polymorhic JSON object, whose type is determined from the
   structure.
 * [`tuple`], which specifies a JSON array, where each index has a specific type.
 * [`service`], which specifies [bi-directional] services with endpoints, suitable for use with
   rpc systems like `gRPC`.

[`type`]: #types
[`enum`]: #enums
[`interface`]: #interfaces
[`tuples`]: #tuples
[bi-directional]: #bi-directional-services
[`service`]: #services

## Directory Structure

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

## File Options

File options are specification-global options that affect the default behavior of the compiler.

They are specified in the root of the specification like this:

```reproto
use foo as bar;

option field_naming = upper_camel;

// snip
```

The following are legal file options.

#### `option endpoint_naming = <ident>`

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
option endpoint_naming upper_camel;

service MyService {
  /// Would be named `put_foo`.
  put_foo(Foo) as "put_foo";

  /// Would be named `GET_BAZ`.
  get_baz() -> Baz;
}
```

#### `option field_naming <naming>`

The default field naming strategy to use.

This option changes the format that a field will take, depending on its name.

Valid options are:

* `lower_camel`, fields would be serialized as `lowerCamel`.
* `upper_camel`, fields would be serialized as `UpperCamel`.
* `upper_snake`, fields would be serialized as `UPPER_SNAKE`.
* `lower_snake`, fields would be serialized as `lower_snake` (default).

## Distribution

Specifications are intended to be distributed through the package management system of `reproto`.

This can be done by uploading a specification to a repository, after which it can be pulled in for
use by other projects through the repository system.

## Versioning

### Ephemeral specifications

An ephemeral specification is one that does _not_ have a version.
For example, `src/io/reproto/toystore.reproto` is an ephemeral specification because it does not
have a version suffix in its filename.

The specification can be used as compiler target.
Like, `--package io.reproto.toystore`.

The specification can only be deployed to a repository if their version has been specified in a
[`[publish]`][publish] section in [`reproto.toml`].

Ephemeral specifications are the default way to store specifications.
They are preferred over versioned specifications because bumping the version number for ephemeral
specifications is a change in [`reproto.toml`] and not renaming a file.

[publish]: manifest.md#publish

### Versioned specifications

A versioned specification is one that has a version in its filename.
For example, `src/io/reproto/toystore-1.0.0.reproto` is a versioned specification because it has a
version number in its filename.

The version string must follow [Semantic Versioning][semver].

Storing versioned specifications permit depending on the directly in [`reproto.toml`], you can
think of them as 'lightweight' repositories.

```toml
[packages]
"io.reproto.toystore" = "1.0.0"
```

Versioned specifications would primarily be used to store out-of-tree specifications which hasn't
made it to central (yet), but that you need to depend on for some reason.

[semver]: https://semver.org

## The reproto language

reproto is designed to be an expressive and productive interface description language.

Specifications describe the _structure_ of JSON values.
This is exactly what is required to build an API that interfaces using JSON.
Having this structure permits performing code generation for various languages.

The choice of using a domain-specific language over something existing like JSON or YAML is an
attempt to improve signal-to-noise ratio.
Concise syntax and intuitive syntax should hopefully lead to more effort that can be spent on
designing good data models instead of worrying about markup.

The following is a simple data model describing a toy:

```reproto
/// A toy in a toy store.
type Toy {
  /// Identifier of the toy.
  id: u64;
  /// Name of the toy.
  name: string;
  /// Category of the toy.
  category?: Category;
  /// Tags of the toy.
  tags: [Tag];
  /// Toy status in the store
  status: Status;
}

/// The status of the toy in the store.
enum Status as string {
  Available as "available";
  Pending as "pending";
  Sold as "sold";
}

/// The category of the toy.
type Category {
  id: u64;
  name?: string;
}

/// The tag of the toy.
type Tag {
  id: u64;
  name: string;
}

/// Simple toystore.
service ToyStore {
  /// Get a stream of all available toys.
  get_toys() -> stream Toy;

  /// Get a single toy by its identifier.
  get_toy(u64) -> Toy;
}
```

Note: More examples can be found in the [`examples`] project.

When compiled, the generated objects can be used to serialize, and deserialize objects.

Like with the following example using [`fasterxml`][fasterxml].

```java
final Toy toy = Toy.builder()
  .id(42)
  .name("Adventure Island")
  .category(new Category(1, "Lego"))
  .tags(ImmutableList.of(new Tag(0, "Plastic")))
  .status(Status.AVAILABLE)
  .build();

final ObjectMapper m = /*  */;
final String json = m.writeValueAsString(toy);
```

[`examples`]: /examples
[fasterxml]: https://github.com/FasterXML/jackson-databind

### Specification Files

A specification is a UTF-8 encoded file containing declarations.

Every file implicitly belong to a package, which depends on where it is located relative to the
[build path].

Conceptually specifications belong to a package, and can have a version.

Specifications without a version are called _ephemeral_ specifications.

[build path]: #build-path

### Imports

Declarations can be imported from other specifications using the `use` keyword at the top of your
specification.

This may also include a local alias for the imported specification.

```reproto
use foo.bar as b;
```

A version requirement may also be present during the import.

```reproto
use foo.bar "^1" as b1;
use foo.bar ">=2.0.0" as b2;
```

If a version requirement is absent, the most recent version will be picked.

A full list of supported specification is documented in the [`semver` package
documentation][semver-package-requirements].

Note that multiple versions of the same package may be imported.

This would typically cause naming conflicts in most target languages, reproto addresses this by
building packages that are named according to which major version of the specification is used.

The following are a few examples for Java:

* `toystore-1.0.0`, would have the package `toystore.v1`.
* `toystore-0.1.0`, would have the package `toystore._0_1_0`.
* `toystore-0.0.1`, would have the package `toystore._0_0_1`.
* `toystore-0.0.1-alpha1`, would have the package `toystore._0_0_1`.

[semver-package-requirements]: https://docs.rs/semver/0.7.0/semver/#requirements

### Built-In Types

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

## Documentation

Documentation can be written for most items in the specification.
This is primarily used when generating documentation.

Documentation is specified using special documentation comments written in [markdown].
For package-level documentation `//!` is used.
For declaration-level documentation `///` is used.
Syntax highlighting is supported with a wide variety of languages using triple backticks.

```reproto
//! A package containing blog-related things

/// Type-level documentation.
///
/// # Examples
///
/// /* code sample here */
type Post {
  /// Field-level documentation.
  title: string;
}
```

[markdown]: https://daringfireball.net/projects/markdown/syntax

See the [hosted documentation examples] to get an idea of what this could look like.

[hosted documentation examples]: https://reproto.github.io/reproto/doc-examples/

### Types

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

### Interfaces

Interfaces are special types providing field-based polymorphism.

Each interface lists all the types that it contains in the declaration.

The following is an example interface with two sub-types.

```reproto
/// Describes how a time series should be sampled.
///
/// Sampling is when a time series which is very dense is samples to reduce its size.
interface Sampling {
    option type_info = type_field;

    /// size of the sample.
    sample_size: u32;
    /// unit of the sample.
    sample_unit: Unit;

    /// Take the average value for each sample.
    Average as "average";

    /// Take the first value encountered for each sample.
    First as "first";

    /// Take the last value encountered for each sample.
    Last as "last";

    /// Take the value which is in the given percentile for each sample.
    Percentile as "percentile" {
        /// Which percentile to sample, as a value between 0-1.0
        percentile: float;
    }
}

enum Unit as string {
     Milliseconds as "ms";
     Seconds as "s";
     Hours as "H";
     Days as "d";
     Weeks as "w";
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

#### Interface sub-types

Sub-types can be specified in two different ways:

```reproto
interface Foo {
  /// No body, empty sub-type.
  Bar;

  /// With body.
  Baz {
  }
}
```

The body of the sub-type can contain fields, and options:

```reproto
interface Foo {
  Bar {
    option my_option = "hello";

    name: string;
  }
}
```

The name of the sub-type is determined using the `as` keyword, it can take a string or an array of
strings, like this:

```reproto
interface Foo {
  Bar as "bar";

  Baz as ["Baz", "baz"];
}
```

All of these are legal JSON-objects for this declaration:

```json
{"type": "bar"}
{"type": "baz"}
{"type": "Baz"}
```

#### `option type_info = <ident>`

Indicates the method of transferring type information.
Valid options are:

* `type_field` sub-types are serialized as objects, with a special field (given by
  `type_field_name`) containing its `name` (default).
* `array` sub-types will be serialized as arrays, where the first value is the `name`.
* `object_keys` sub-types will be serialized as objects with a single
   key, where the key is the `name`.

#### `option type_field_name = <string>`

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

### Enums

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

### Services

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

## Bi-directional services

You might have noticed the `stream` keyword in the above examples.
This means that services are _bi-directional_.
Zero or more requests or responses of the given types may be sent, _in any order_.

This paradigm is more general than your typical unary request-response.

Calls against endpoints may also be long-lived, which would be useful for use-cases like streaming.

## Reserved fields

Fields can be reserved using a special option called `reserved`.
Fields which are reserved _cannot_ be added to the schema.

Attempting to do so will yield an error like the following:

```bash
examples/toystore.reproto:55:3-21:
 55:   no_can_do: string;
       ^^^^^^^^^^^^^^^^^^ - field reserved
examples/toystore.reproto:49:12-21:
 49:   reserved no_can_do;
       ^^^^^^^^^^^^^^^^^^^ - field reserved here
```

As long as the reserved statement is preserved, it prevents future introductions of a given field.

Clients decoding a reserved field should raise an error.

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

## Language Support

This section is dedicated towards describing language-specific behaviors provided by `reproto`.

### Java

```toml
# File: reproto.toml

language = "java"
paths = ["src"]
output = "target"
```

Java classes are generated using _nested_ classes that matches the hierarchy specified in the
specification.

The following specification:

```reproto
// file: src/io/reproto/example.reproto

type Foo {
  // skipped

  type Bar {
    // skipped
  }
}
```

Would result in the following Java classes:

```java
// File: target/io/reproto/example/Foo.java

package io.reproto.example;

public class Foo {
  // skipped

  public static class Bar {
    // skipped
  }
}
```

#### Module: `jackson`

```toml
# reproto.toml

language = "java"
paths = ['src']

[modules.jackson]
```

Adds [jackson] annotations to generated classes and generates support classes for handling tuples.

[jackson]: https://github.com/FasterXML/jackson

#### Module: `lombok`

```toml
# reproto.toml

language = "java"
paths = ['src']

[modules.lombok]
```

Adds [lombok] annotations to generated classes.

[lombok]: https://projectlombok.org

#### Module: `builder`

```toml
# reproto.toml

language = "java"
paths = ['src']

[modules.builder]
```

Generates builders for all data classes.

The following:

```reproto
// File: src/io/reproto/examples.reproto

type Foo {
  field: string;
}
```

Would generate:

```java
package io.reproto.examples;

public class Foo {
  // skipped

  public static class Builder {
    // skipped

    public Builder field(final String field) {
      // skipped
    }

    public Foo build() {
      // skipped
    }
  }
}
```

### Rust

```toml
# reproto.toml

language = "rust"
paths = ["src"]
output = "target"
```

Code generation for rust relies entirely on [Serde].

You'll need to add the following dependencies to your project:

```toml
[dependencies]
serde_json = "1"
serde = "1"
serde_derive = "1"
```

And the following extern declarations:

```rust
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde;
```

Rust does not support nested structs, so generated types follow a naming strategy like the
following:

```reproto
// File: src/io/reproto/example.reproto

type Foo {
  // skipped

  type Bar {
    // skipped
  }
}
```

Would generate:

```rust
// File: target/io/reproto/example.rs

struct Foo {
  // skipped
}

struct Foo_Bar {
  // skipped
}
```

[Serde]: https://serde.rs

#### Module: `chrono`

```toml
# reproto.toml

language = "rust"
paths = ["src"]

[modules.chrono]
```

Rust doesn't have a native type to represent `datetime`, so the `chrono` module is used to
support that through the [`chrono` crate].

You'll need to add the following dependency to your `Cargo.toml`:

```toml
[dependencies]
chrono = {version = "0.4", features = ["serde"]}
```

[`chrono` crate]: https://crates.io/crates/chrono

### Python

```toml
# File: reproto.toml

language = "python"
paths = ["src"]
output = "target"
```

In python, generated types follow a naming strategy like the following:

```reproto
// File: src/io/reproto/example.reproto

type Foo {
  // skipped

  type Bar {
    // skipped
  }
}
```

Would generate:

```python
# File: target/io/reproto/example.py

class Foo:
  pass

class Foo_Bar:
  pass
```

### Javascript

```toml
# File: reproto.toml

language = "js"
paths = ["src"]
output = "target"
```

In Javascript, generated types follow a naming strategy like the following:

```reproto
// File: src/io/reproto/example.reproto

type Foo {
  // skipped

  type Bar {
    // skipped
  }
}
```

Would generate:

```javascript
// File: target/io/reproto/example.js

class Foo {
  // skipped
}

class Foo_Bar {
  // skipped
}
```

[`reproto.toml`]: manifest.md
