# ReProto File Specification

See [TODO](todo.md) for things that are work in progress.

* [Introduction](#introduction)
* [Built-In Types](#built-in-types)
* [Version Requirements](#version-requirements)
* [Types](#types)
* [Interfaces](#interfaces)
* [Tuples](#tuples)
* [Enums](#enums)
* [Match](#match)
* [Reserved Fields](#reserved-fields)
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
use common as c;

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

## Version Requirements

Every specification can be versioned in reproto.

To specify which version a particular specification belongs to, move the file to a location that
corresponds to `<package>/<version>.reproto`. For example: `foo/bar/1.0.0.reproto`.

To import a particular version of a specification, a similar syntax can be used for `use`
statements.

```reproto
use foo.bar@^1.0;

// ...
```

Multiple versions of the same package can co-exist.
For a given version specification, the latest matching version will be pulled in.
Packages for versioned specifications will be mangled so that they do not conflict, using a method
specific to what is supported by the target language.

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
/**
 * Describes how a time series should be sampled.
 *
 * Sampling is when a time series which is very dense is samples to reduce its size.
 */
interface Sampling {
    // size of the sample.
    sample_size: unsigned/32;
    // unit of the sample.
    sample_unit: Unit;

    /**
     * Take the average value for each sample.
     */
    Average {
        name "average";
    }

    /**
     * Take the first value encountered for each sample.
     */
    First {
        name "first";
    }

    /**
     * Take the last value encountered for each sample.
     */
    Last {
        name "last";
    }

    /**
     * Take the value which is in the given percentile for each sample.
     */
    Percentile {
        name "percentile";

        // Which percentile to sample, as a value between 0-1.0
        percentile: float;
    }
}

enum Unit {
     MILLISECONDS("ms");
     SECONDS("s");
     HOURS("H");
     DAYS("d");
     WEEKS("w");

     short: string;

     serialized_as short;
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
enum SI {
    NANO("nano", "n", 1e-9);
    MICRO("micro", "μ", 1e-6);
    MILLI("milli", "m", 1e-3);
    KILO("kilo", "k", 1e3);
    MEGA("mega", "M", 1e6);

    // select which field to serialize as.
    serialized_as unit_name;

    unit_name: string;
    symbol: string;
    factor: double;
}
```

*Note*: it is recommended to avoid the fields `name` and `value`, since these are used in some
        languages by enums natively.

Using this, `SI.NANO` would be serialized as:

```json
"nano"
```

Associating data with enums permit less specialized code for dealing with them:

```java
final SI si = deserialize("nano");
System.out.println(Math.floor((1000.0 / si.factor)) + si.symbol + "s");
```

The following options are supported by enums:

#### `serialized_as <identifier>`

Indicates that the enum should be serialized as the given field |

#### `serialized_as_name`

Indicates that the enum should be serialized as its `name`.

## Match

Match declarations exist to allow types to be created from non-objects.

With the example below, `Foo` can now be created from a `string`, or a `number` as well as an
object.

```reproto
type Foo {
    match {
        s: string => Foo(name: s);
        n: unsigned => Foo(name: "from unsigned", value: n);
        true => Foo(name: "from true", value: 1),
    }

    name: string;
    value?: unsigned;
}
```

When a match declaration is present, _only_ the variants listed in it are permitted ways of
decoding that object.

To permit `Foo` from being created from an object again, the following must be added.

```reproto
type Foo {
    match {
        /* omitted */
        foo: Foo => foo;
    }

    name: string;
    value?: unsigned;
}
```

## Reserved fields

Fields can be reserved using a special option called `reserved`.

Fields which are reserved _cannot_ be added again.

Attempting to do so will yield an error like the following:

```bash
examples/heroic/v1.reproto:55:3-21:
 55:   no_can_do: string;
       ^^^^^^^^^^^^^^^^^^ - field reserved
examples/heroic/v1.reproto:49:12-21:
 49:   reserved no_can_do;
                ^^^^^^^^^ - field reserved here
```

As long as the reserved statement is preserved, it prevents future or current introductions of
reserved field.

It can also be used to reserve future fields, that you intend to introduce at some point.

Clients who decode a reserved field should ignore them.

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
