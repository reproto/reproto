# ReProto File Specification

See [TODO](todo.md) for things that are work in progress.

* [Introduction](#introduction)
* [Built-In Types](#built-in-types)
* [Types](#types)
* [Interfaces](#interfaces)
* [Tuples](#tuples)
* [Enums](#enums)
* [Custom Code](#custom-code)
* [Extensions](#extensions)

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
interface Instant {
    reserved old_field;

    RelativeToNow {
        name "relative", "r";
        // Offset in milliseconds.
        offset: signed/32;
    }

    Absolute {
        name "absolute", "a";
        // Absolute timestamp since unix epoch.
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
enum SI {
    NANO("nano", "n", 1e-9),
    MICRO("micro", "μ", 1e-6),
    MILLI("milli", "m", 1e-3),
    KILO("kilo", "k", 1e3),
    MEGA("mega", "M", 1e6);

    // select which field to serialize as.
    serialize_as value;

    name: string;
    symbol: string;
    factor: double;
}
```

Using this, SI.NANO would be serialized as:

```json
"nano"
```

Associating data with the enum permits less specialized code for dealing with them.

```java
public class Entry {
  public static void main(String[] argv) {
    final SI si = ...;
    System.out.println(52 + si.factor + "s");
  }
}
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

In a perfect world, extensions should not be necessary.
The specification should be in sync with the API, and there should be no additional configuration
necessary to start using the generated code.

Extensions allow you to add additional information, as long as it doesn't conflict with any
existing declarations.

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

You can now add extend existing types by specifying the same type somewhere else in your path.

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
