# ReProto File Specification

See [TODO](todo.md) for things that are work in progress.

* [Introduction](#introduction)
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

ReProto is geared towards being an expressive and productive protocol specification.

The choice of using a DSL over something existing like JSON or YAML is an attempt to improve
signal-to-noise ratio.
Concise markup, and relatively intuitive syntax should hopefully mean that more effort can be spent
on designing good data models.

The following is an example specification for a simple time-series database:

```reproto
tuple Sample {
  timestamp: unsigned/64;
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
use by other projects through ReProto's repository system.

### Versioned specifications

A versioned specification is one that has a version in its filename.

The version string follows [Semantic Versioning][semver], but the following is a brief
description of what is permitted.

The version number must follow semantic versioning (`1.2.0`).
Pre-releases are also supported by appending a hyphen and a series of dot-separated identifiers
(e.g. `1.2.1-beta1`).

[semver-2]: https://semver.org

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

A full list of supported specification is documented in the [`semver` package
documentation][semver-package-requirements].

Note that multiple versions of the same package may be imported.

[semver-package-requirements]: https://docs.rs/semver/0.7.0/semver/#requirements

## Built-In Types

There are a number of built-in types available:

| Type               | Description |
|--------------------|-------------|
| `unsigned{/size}`  | Unsigned integer values which can store a given number of bits |
| `signed{/size}`    | Signed integer values which can store a given number of bits |
| `double`, `float`  | Floating point precision numbers |
| `string`           | UTF-8 encoded strings |
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
    bar: signed/32;
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
    sample_size: unsigned/32;
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
  time: unsigned/64;
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
enum SI: string {
    NANO = "nano";
    MICRO = "micro";
    MILLI = "milli";
    KILO = "kilo";
    MEGA = "mega";
}
```

Using this, `SI.NANO` would be serialized as:

```json
"nano"
```

## Services

Service declarations describe endpoints that HTTP-requests can be sent against.

Services are declared using the `service` keyword.

```reproto
/// My Service
service MyService {
   // ...
}
```

Inside of a service, endpoints are declared by describing it with one of the HTTP-method keywords
(`GET`, `POST`, `DELETE`, `PUT`, or `UPDATE`).
Optionally they can also be declared with a path.

```reproto
service MyService {
  /// Default endpoint
  GET {
    // ...
  }

  GET "posts" {
    // ...
  }
}
```

Paths are composed in hierarchies. Any declaration that is nested inside of another inherits the
path from the parent.

They can also capture variable if the are declared using back-ticks, like `` `posts/{id:string}` ``.

```reproto
service MyService {
  "posts" {
    /// Get all posts.
    GET {
      // the complete path for this resource is "posts"
    }

    /// Get the post with the id `id`
    GET `{id:string}` {
      // the complete path for this resource is `posts/{id:string}`
    }
  }
}
```

Return values are declared using the `returns` keyword.

```reproto
service MyService {
  GET "posts" {
    returns [Post] {
      status 200;

      // multiple mime types are supported
      mime "text/yaml";
      mime "application/json";
    }
  }
}

type Post {
  title: string;
  author: string;
}
```

Return statements can be inherited. As an example, any error type returned by any endpoint in the
service can be declared in the root.

```
service MyService {
  returns Error {
    status 500;
    mime "application/json";
  }

  // ...
}

type Error {
  message: string;
}
```

If your endpoint accepts a body, this is declared using the `accepts` keyword.

```reproto
service MyService {
  POST "posts" {
    accepts Post {
      // multiple mime types are supported.
      accept "application/json";
      accept "text/yaml";
    }

    returns any {
      status 200;
    }
  }
}

type Post {
  title: string;
  author: string;
}
```

## Reserved fields

Fields can be reserved using a special option called `reserved`.
Fields which are reserved _cannot_ be added to the schema.

Attempting to do so will yield an error like the following:

```bash
examples/heroic/v1.reproto:55:3-21:
 55:   no_can_do: string;
       ^^^^^^^^^^^^^^^^^^ - field reserved
examples/heroic/v1.reproto:49:12-21:
 49:   reserved no_can_do;
       ^^^^^^^^^^^^^^^^^^^ - field reserved here
```

As long as the reserved statement is preserved, it prevents future introductions of a given field.

Clients decoding a reserved field should raise an error.

## Extensions

ReProto permits all types and interfaces to be extended.

Extensions allow for additions, and is typically used to adapt a protocol specification to
your local environment.
They allow you to add additional information, as long as it doesn't conflict with any
existing declarations.

In a perfect world, extensions should not be necessary and the specification should be in sync with
the API, and there should be no additional configuration necessary to start using the generated
code.

An extension is loaded when a when an identical package and type declaration is present in the
path.

Assume you have a type called `Foo` in the `foo` package.

```reproto
// file: protos/foo.reproto
type Foo {
  field: string;
}
```

You can now add extend existing types by specifying the same type somewhere else in your path.

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
