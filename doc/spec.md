# TODO

* Support validation?
  * How do we support complex stringy types without it?
  * Defer to external validator (keyword: `validate <from> <to>`)?
* Is Rust a suitable language?
  * Less bugs are nice.
  * Fairly advanced type system, allowing for better expressiveness.
  * Can provide statically compiled compiler for most major platforms.
  * LR(1) parser support is not very mature :(, I miss ANTLR.
* Per-project (not definition) language extensions might be needed
  * e.g. drop a heroic.reproto file in the project (in the right location), and pick up any extensions defined in it that allows the module to better integrate into the project.

# reProto Specification

## Introduction

reProto is geared towards being an expressive and productive protocol specification.

The choice of using a DSL over something existing like JSON or YAML is an attempt to improve the signal-to-noise ratio.
Concise markup, and relatively intuitive syntax means that more effort can be spent on designing good models.

A good benchmark for a DSL is to measure how easily it is to visualize both the JSON and the target source a given declaration corresponds to.

A .reproto file has the following general syntax:

```
package proto.v1;

// A single point.
type Point = (u64, double);

// A single event.
type Event = (u64, any);

// An interface for samples.
// Note that conflicting field names are fine as long as they belong to different sub types.
interface Samples {
    Points {
        name "points";
        required Point[] data;
    }

    Events {
        name "events";
        required Event[] data;
    }
}

// Inferred based on type of argument.
// Only one of each type may be present.
type Instant = string | number;

// Inferred based on type of argument.
type Duration = string | number;

// Aggregation, inferred based on objects with a 'type' field.
//
// Aggregations have a single shared field (size).
//
// Example: {"type", "sum"}
// Example 2: {"type", "quantile", "q": 0.1}
interface Aggregation {
    optional Duration size;

    Sum {
        name "sum";
    }

    Quantile {
        name "quantile";
        optional float q;
    }
}

// Range, inferred based on objects with a 'type' field.
interface Range {
    Relative {
        name "relative";
        required Duration duration;
    }

    Absolute {
        name "absolute";
        required Instant start;
        required Instant end;
    }
}

message Query {
    optional Aggregation aggregation;
    optional Range range;
}
```

This could then be used to straight up serialize or deserialize an `Aggregation` in Java:

```java
final ObjectMapper m = new ObjectMapper();
final byte[] message = /* aggregation as bytes */;
final Aggregation aggregation = m.readValue(message, Aggregation.class);
```

## Messages

Messages are named types that are used to designate a data structure that is intended to be sent as
a message.

Messages have a name which must be unique for the package in which it is defined.

The following is an example message declaration:

```
message Foo {
    required string foo;
    required i32 bar;
}
```

### As JSON

Messages are encoded as objects.

For example (using `Foo`):

```
{
    "bar": 42
}
```

### Java

In Java, each message generates exactly one concrete class.

This class then uses the configured serialization mechanism (depending on the backend) to determine
how it will be generated.

## Interfaces

Interfaces are named types that designate a message, whose type is determined by some other method.

These provide polymorphism, since they refer to instances that share a common type.

Each interface lists all the types that it contains in the declaration.

The following is an example interface with two subtypes.

```
interface Instant {
    reserved "deprecated", SomethingOld;

    RelativeToNow {
        name "relative", "r";
        required u32 offset;
    }

    Absolute {
        name "absolute", "a";
        required u64 timestamp;
    }
}
```

### As JSON

An interface is encoded as an object, with a special `type` field.

For example (using `Instant.RelativeToNow`):

```json
{
    "type": "relative",
    "offset": -1000
}
```

### Java

In Java, interfaces generate an `interface`, and zero or more implementations that implements that
interface.

This then uses a type-field resolution mechanism specific to the active backend to resolve which
sub-type of that interface a given message belongs to.

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

### As JSON

Enums are serialized as a string, or a number constant.

For example (using `State.START`):

```json
"start"
```

Or another example (using `StateNumeric.END`):

```json
2
```

### Java

Enums are modelled using a regular Java `enum`.

Serialization might be affected depending on the type the enum has.

# Backends

### `fasterxml` (Java)

Using the fasterxml (Java) backend would result in the following class being generated for
`Aggregation`:

```java
package com.company.proto.v1;

@JsonSubTypes({
  @JsonSubTypes.Type(Aggregation.Sum.class),
  @JsonSubTypes.Type(Aggregation.Quantile.class)
})
interface Aggregation {
    Sampling getSampling();

    <T> accept(Visitor<T> visitor);

    @JsonTypeName("sum")
    class Sum implements Aggregation {
        private final Sampling sampling;

        @JsonCreator
        public Quantile(
            @JsonProperty("sampling") final Sampling sampling
        ) {
            this.sampling = sampling;
        }

        @Override
        public Sampling getSampling() {
            return this.sampling;
        }

        @Override
        public <T> T accept(final Visitor<T> visitor) {
            return visitor.visitSum(this);
        }
    }

    @JsonTypeName("quantile")
    class Quantile implements Aggregation {
        private final Sampling sampling;
        private final float q;

        @JsonCreator
        public Quantile(
            @JsonProperty("sampling") final Sampling sampling,
            @JsonProperty("q") final float q
        ) {
            this.sampling = sampling;
            this.q = q;
        }

        @Override
        public Sampling getSampling() {
            return this.sampling;
        }

        @Override
        public float getQ() {
            return this.q;
        }

        @Override
        public <T> T accept(final Visitor<T> visitor) {
            return visitor.visitQuantile(this);
        }
    }

    interface Visitor<T> {
        default T visitSum(Sum sum) {
            return defaultAction();
        }

        default T visitQuantile(Quantile quantile) {
            return defaultAction();
        }

        T defaultAction();
    }
}
```
