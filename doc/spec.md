# ReProto File Specification

* [Introduction](#introduction)
* [Built-In Types](#built-in-types)
* [Types](#types)
* [Interfaces](#interfaces)
* [Tuples](#tuples)
* [Enums](#enums)
* [Custom Code](#custom-code)
* [Extensions](#extensions)

# TODO

* General
  * ~~Treat named types as (named) simple types, like tuples.~~
  * ~~Enums.~~
  * Documentation syntax (preferably not comments).
  * Type aliases.
    * *Question:* Are they useful?
  * Validators
    * Tricky, too many extensions to DSL to be useful. Might just skip for now but push towards `2.0`.

* ~~Maven Plugin + Maven Artifacts w/ Static Builds~~~
  * For clean integration into Java ecosystem.

* HTML Backend
  * Generate static documentation.

* Java Backend
  * ~~Generate _good_ builder.~~
  * ~~Generated equals/hashCode (disabled when using lombok).~~
  * ~~Generated toString (disabled when using lombok).~~
  * ~~Tuple decoding (in `fasterxml`).~~

* Python Backend
  * ~~Encode support (e.g. `instance.encode()`)~~
  * ~~Relative import, especially with package prefixes.~~ (not needed with aliases)
  * ~~Create missing `__init__.py` files.~~
  * ~~Array decoding.~~
  * ~~Map decoding.~~
  * ~~Tuple decoding.~~
  * ~~Optional support.~~
  * ~~Encode `type` field.~~
  * ~~Enum support~~
  * Strict decoding/encoding where types are deeply verified.

## Introduction

ReProto is geared towards being an expressive and productive protocol specification.

The choice of using a DSL over something existing like JSON or YAML is an attempt to improve signal-to-noise ratio.
Concise markup, and relatively intuitive syntax should hopefully mean that more effort can be spent on designing good data models.

A .reproto file has the following general syntax:

```reproto
package proto.v1;

// Importing types from other module.
use common as c;

// A tuple.
tuple Sample {
  timestamp: unsigned/64;
  value: double;
}

// An interface.
interface Range {
  // A field that is inherited in all sub-types.
  unit: c.Unit;

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

// A plain type.
type Query {
  // A field with a custom type.
  range: Range;
  // An association (a.k.a. Map).
  extra: {string: string};
  // An optional field.
  id?: string;
}

type QueryResponse {
  // An array.
  samples: [Sample];
}
```

This could then be used to straight up serialize or deserialize an `Query` in Java:

```java
final ObjectMapper m = new ObjectMapper();
final byte[] message = /* aggregation as bytes */;
final Query aggregation = m.readValue(message, Query.class);
```

## Built-In Types

There are a number of built-in types available:

| Type               | Description |
|--------------------|-------------|
| `unsigned{/size}`  | Unsigned integer values which can store a given number of bits |
| `signed{/size}`    | Signed integer values which can store a given number of bits |
| `double`, `float`  | Floating point precision numbers |
| `string`           | UTF-8 encoded strings |
| `bytes`            | Arbitrary byte-arrays, are encoded as base64-strings in JSON |
| `[<type>]`         | Arrays which store the given type  |
| `{<type>: <type>}` | Associations with the given key and value (note: the `<type>` of the key currently _must_ be `string` |

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
interface Instant {
    reserved "deprecated", SomethingOld;

    RelativeToNow {
        name "relative", "r";
        offset: u32;
    }

    Absolute {
        name "absolute", "a";
        timestamp: unsigned/64;
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

## Tuples

Tuples are sequences of data, where each element has a known type.

```reproto
tuple Sample {
  time: unsigned/64;
  value: double;
}
```

All fields in a tuple are required, and are presented in the order that the field occurs in the sequence.

A single sample (e.g. `Sample(time: 1, value: 2.0)`) would be encoded like this in JSON:

```json
[1, 2.0]
```

## Enums

Enums can take on of a given set of constant values.

```
enum State {
    UNKNOWN("unknown"),
    START("start"),
    END("end");

    // select which field to serialize as.
    serialize_as value;

    value: string;
}
```

`State` would be serialized as a given value, for example `State.END` would become the following in
JSON:

```json
"end"
```

## Custom Code

A powerful mechanism for modifying the behaviour of your protocols is to embed code snippets.
This _only_ be done in [extensions](extensions), to adapt a given set of protocols into your
application.

```reproto
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

```reproto
// file: protos/foo.reproto
package foo;

type Foo {
  field: string;
}
```

You can now add additional fields or custom code snippets by doing the following:

```reproto
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
