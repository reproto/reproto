# reProto Specification

* [Introduction](#introduction)
* [Types](#types)
* [Interfaces](#interfaces)
* [Enums](#enums)
* [Custom Code](#custom-code)
* [Extensions](#extensions)
* [Tuples](#tuples)

# TODO

* Support validation?
  * How do we support complex stringy types without it?
  * Defer to external validator (keyword: `validate <from> <to>`)?
* Is Rust a suitable language?
  * Less bugs are nice.
  * Fairly advanced type system, allowing for better expressiveness.
  * Can provide statically compiled compiler for most major platforms.
  * ~~LR(1) parser support is not very mature :(, I miss ANTLR.~~ [pest][pest] works really well!
* Per-project (not definition) language extensions might be needed
  * e.g. drop a heroic.reproto file in the project (in the right location), and pick up any extensions defined in it that allows the module to better integrate into the project.
  * This is now supported! Add a local directory to your path, and match the package you'd like to
      extend.

[pest]: https://github.com/pest-parser/pest

# Missing Features

* Maven Plugin + Maven Artifacts w/ Static Builds
  * For clean integration into Java ecosystem.

* HTML
  * Generate static documentation.

* Java
  * ~~Generate _good_ builder.~~
  * ~~Generated equals/hashCode (disabled when using lombok).~~
  * ~~Generated toString (disabled when using lombok).~~
  * Type aliases.
  * Tuple decoding (in `fasterxml`).

* Python
  * ~~Encode support (e.g. `instance.encode()`)~~
  * ~~Relative import, especially with package prefixes.~~ (not needed with aliases)
  * ~~Create missing `__init__.py` files.~~
  * ~~Array decoding.~~
  * ~~Map decoding.~~
  * ~~Tuple decoding.~~
  * ~~Optional support.~~
  * ~~Encode `type` field.~~
  * Type aliases.
  * Strict decoding/encoding where types are deeply verified.

* General
  * ~~Treat named types as (named) simple types, like tuples.~~
  * Enum support.
  * Documentation syntax (preferably not comments).

## Introduction

reProto is geared towards being an expressive and productive protocol specification.

The choice of using a DSL over something existing like JSON or YAML is an attempt to improve the signal-to-noise ratio.
Concise markup, and relatively intuitive syntax means that more effort can be spent on designing good models.

A good benchmark for a DSL is to measure how easily it is to visualize both the JSON and the target source a given declaration corresponds to.

A .reproto file has the following general syntax:

```
package proto.v1;

use common as c;

// A single point.
type Sample = (timestamp: u64, value: double);

// Inferred based on type of argument.
// Only one of each type may be present.
type Instant = string | number;

// Aggregation, inferred based on objects with a 'type' field.
// Aggregations have a single shared field (size).
interface Aggregation {
    size?: c.Duration;

    Sum {
        name "sum";
    }

    Quantile {
        name "quantile";
        q: float;
    }
}

// Range, inferred based on objects with a 'type' field.
interface Range {
    Relative {
        name "relative";

        duration: c.Duration;
    }

    Absolute {
        name "absolute";
        start: Instant;
        end: Instant;
    }
}

// A single type.
type Query {
    range: Range;
    aggregation?: Aggregation;
}
```

This could then be used to straight up serialize or deserialize an `Query` in Java:

```java
final ObjectMapper m = new ObjectMapper();
final byte[] message = /* aggregation as bytes */;
final Query aggregation = m.readValue(message, Query.class);
```

## Types

Types are named types that are used to designate a data structure that is intended to be
serialized.

Types have a name which must be unique for the package in which it is defined.

The following is an example type declaration:

```
type Foo {
    foo: string;
    bar: i32;
}
```

Types are encoded as objects.

For example (using `Foo`):

```
{
    "bar": 42
}
```

## Interfaces

Interfaces are special types providing field-based polymorphism.

Each interface lists all the types that it contains in the declaration.

The following is an example interface with two sub-types.

```
interface Instant {
    reserved "deprecated", SomethingOld;

    RelativeToNow {
        name "relative", "r";
        offset: u32;
    }

    Absolute {
        name "absolute", "a";
        timestamp: u64;
    }
}
```

An interface is encoded as an object, with a special `type` field.

For example (using `Instant.RelativeToNow(offset: -1000)`):

```json
{
    "type": "relative",
    "offset": -1000
}
```

## Enums

Enums can take on of a given set of constants.

It can be serialized either as a string, or as a number.

```
enum State(string) {
    UNKNOWN = "unknown";
    START = "start";
    END = "end";
}
```

For numerical enums:

```
enum StateNumeric(number) {
    UNKNOWN = 0;
    START = 1;
    END = 2;
}
```

Enums are serialized as a string, or a number constant.

For example (using `State.START`):

```json
"start"
```

Or another example (using `StateNumeric.END`):

```json
2
```

## Custom Code

A powerful mechanism for modifying the behaviour of your protocols is to embed code snippets.
This _only_ be done in [extensions](extensions), to adapt a given set of protocols into your
application.

```
package foo;

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

## Extensions

reProto allows all types and interfaces to be extended.

Extensions allow for additions, and is typically used to adapt a protocol specification to
your local environment.

An extension is loaded when a when an identical package and type declaration is present in the
path.

Assume you have a type called `Foo` in the `foo` package.

```
// file: protos/foo.reproto
package foo;

type Foo {
  field: string;
}
```

You can now add additional fields or custom code snippets by doing the following:

```
// file: ext/foo.reproto
package foo;

type Foo {
  other?: string;

  java {{
    public boolean hasOther() {
      return this.other.isPresent();
    }
  }}
}
```

## Tuples

Tuples are sequences of data, where each element has a known type.

```
tuple Sample {
  time: u64;
  value: double;
}
```

A single sample (e.g. `Sample(time: 1, value: 2.0)`) would be encoded like this in JSON:

```json
[1, 2.0]
```

The naming is used for languages which do not natively support tuples, like python:

```python
class Sample:
  def __init__(self, time, value):
    self.time = time
    self.value = value

  @staticmethod
  def decode(data):
    time = data[0]
    value = data[1]
    return Sample(time, value)

  def encode(self):
    return (self.time, self.value)
```

# Backends

## java (`-b java`)

Supported modules:

* `fasterxml` - Generates annotations and serializers suitable for FasterXML Jackson.
* `mutable` - Generates classes where fields can be mutated (default is immutable).

## Python (`-b python`)
