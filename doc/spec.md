# Specification

* [Specification files](#specification-files)
* [Directory structure](#directory-structure)
* [File attributes](#file-attributes)
* [Distribution](#distribution)
* [Versioning](#versioning)
  * [Ephemeral specifications](#ephemeral-specifications)
  * [Versioned specifications](#versioned-specifications)
* [The reproto language](#the-reproto-language)
  * [Specification files](#specification-files)
  * [Imports](#imports)
  * [Built-in types](#built-in-types)
  * [Attributes](#attributes)
  * [Documentation](#documentation)
  * [Types](#types)
  * [Enums](#enums)
  * [Interfaces](#interfaces)
    * [Interface sub-types](#interface-sub-types)
    * [Interface attributes](#interface-attributes)
  * [Tuples](#tuples)
  * [Services](#services)
    * [Endpoints](#endpoints)
    * [HTTP services](#http-services)
    * [HTTP paths](#http-paths)
    * [Bi-directional services](#bi-directional-services)
  * [Reserved fields](#reserved-fields)
  * [Custom Code](#custom-code)

# Specification files

Specification files have the extension `.reproto`.

Each specification contains a number of _declarations_.

 * [`type`], the structure of a [JSON object].
 * [`enum`], a discrete set of [string] values.
 * [`interface`], a polymorhic [JSON object], whose type is determined from the structure.
 * [`tuple`], a [JSON array], where each index has a specific type.
 * [`service`], which specifies [bi-directional] services with endpoints, suitable for use with
   rpc systems like `gRPC`.

[`type`]: #types
[`enum`]: #enums
[`interface`]: #interfaces
[`tuple`]: #tuples
[bi-directional]: #bi-directional-services
[`service`]: #services
[string]: https://www.json.org/
[JSON Object]: https://www.json.org/
[JSON Array]: https://www.json.org/

# Directory structure

The compiler is provided with a number of [build paths].
For each build path, it is expected to have a directory structure like the following:

```
io/reproto/example.reproto
io/reproto/example-1.0.0-beta1.reproto
io/foo-0.2.0.reproto
```

Each directory indicate a package component.
The name of the file has the structure `<name>[-<version>].reproto`.
The version is optional, and if it is left unspecified the file is an [ephemeral specification].
If the version is present, it's called a [versioned specification].

For the example above, we can see _three_ specifications.

 * An ephemeral specification, for the package `io.reproto.example`.
 * A versioned specification, for the package `io.reproto.example` and version `1.0.0-beta1`.
 * A versioned specification, for the package `io.foo` and version `0.2.0`.

[ephemeral specification]: #ephemeral-specifications
[versioned specification]: #versioned-specifications

# File attributes

File attributes are specification-global attributes that affect the default behavior of the
compiler for the given file.

They are specified in the root of the specification like this:

```reproto
#![field_naming(upper_camel)]

use foo as bar;

// snip
```

The following are legal file attributes.

## `#![endpoint_naming(<naming>)]`

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

This attribute changes what endpoints are named by default.

Valid arguments are:

* `lower_camel`, fields would be named as `lowerCamel`.
* `upper_camel`, fields would be named as `UpperCamel`.
* `upper_snake`, fields would be named as `UPPER_SNAKE`.
* `lower_snake`, fields would be named as `lower_snake` (default).

This does _not_ affect explicitly named endpoints using `as`.

```reproto
#![endpoint_naming(upper_camel)]

service MyService {
  /// Would be named `put_foo`.
  put_foo(Foo) as "put_foo";

  /// Would be named `GET_BAZ`.
  get_baz() -> Baz;
}
```

## `#![field_naming(<naming>)]`

The default field naming strategy to use.

This attribute changes the format that a field will take, depending on its name.

Valid arguments are:

* `lower_camel`, fields would be serialized as `lowerCamel`.
* `upper_camel`, fields would be serialized as `UpperCamel`.
* `upper_snake`, fields would be serialized as `UPPER_SNAKE`.
* `lower_snake`, fields would be serialized as `lower_snake` (default).

# Distribution

Specifications are intended to be distributed through the package management system of `reproto`.

This can be done by uploading a specification to a repository, after which it can be pulled in for
use by other projects through the repository system.

# Versioning

## Ephemeral specifications

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

[`reproto.toml`]: manifest.md
[publish]: manifest.md#publish

## Versioned specifications

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

[`reproto.toml`]: manifest.md
[semver]: https://semver.org

# The reproto language

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

## Specification files

A specification is a UTF-8 encoded file containing declarations.

Every file implicitly belong to a package, which depends on where it's located relative to the
[build path].

Conceptually specifications belong to a package, and can have a version.

Specifications without a version are called _ephemeral_ specifications.

[build path]: #build-path

## Imports

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

## Built-in types

There are a number of built-in types available:

| Type               | Description |
|--------------------|-------------|
| `u32`, `u64`       | Unsigned integer values which can store a given number of bits |
| `i32`, `i64`       | Signed integer values which can store a given number of bits |
| `double`, `float`  | Floating point precision numbers |
| `string`           | UTF-8 encoded strings |
| `datetime`         | ISO-8601 dates encoded as strings. Combined date and time with timezone. Only supports full timestamps normalized to the `Z` timezone, like `2017-10-14T11:42:06Z`. |
| `bytes`            | Byte arrays, are encoded as base64-strings in JSON using `+`, and `/` as supplementary characters and `=` for padding |
| `boolean`          | Boolean values, `true` or `false` |
| `[<type>]`         | Arrays which store the given type  |
| `{<type>: <type>}` | Associations with the given key and value (note: the `<type>` of the key currently _must_ be `string` due to limitations in JSON, but might be subject to change if other formats are supported in the future) |

## Attributes

Attributes are elements associated with declarations, fields, or sub-types in reproto.
They were inspired by the same language construct in [Rust].

They provide an extensible way to add additional configuration to the language, without being too
heavy-weight.

The following is an example service using attributes to configure it to [support HTTP]:

```reproto
#[http(url = "http://example.com")]
service MyService {
  #[http(url = "/")]
  get() -> string;
}
```

All attributes take the form `#[...]`, and are associated with one element in the specification.
Selection attributes look like `#[http(...)]`, while words look like `#[foo, bar, baz]`
Selections can also contain words, like: `#[allow(unused)]`.

They were introduced to provide a lightweight mechanism to extend the language, without always
having to introduce specialized syntax.

[Rust]: https://rust-lang.org
[support HTTP]: #http-services

### Working with attributes

Attributes should be converted in [into_model.rs] into the appropriate intermediate representation.
Every attribute should be checked with `check_attributes!`, and `check_selections!` to provide
warnings if some attributes are not used.

For new attributes, it might be necessary to introduce new data structures in [core].

[into_model.rs]: /lib/backend/src/into_model.rs
[core]: /lib/core/src/

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

## Types

Types are named types that are used to designate a data structure that is intended to be
serialized.

Types have a name which must be unique for the package in which it's defined.

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

Interfaces are special types providing property-based polymorphism.

Each interface lists all the types that it contains in the declaration and has a strategy which
maps to JSON for resolving sub-types.

The following is an example interface with two sub-types.

```reproto
/// Describes an animal.
interface Animal {
    /// Name of the animal.
    name: string;

    /// Age of the animal in years.
    age: u32;

    /// A cat.
    Cheetah as "cheetah" {
        /// How quickly can this cheetah run?.
        landspeed: u32;
    }

    /// An eagle.
    Eagle as "eagle" {
        /// The wingspan of the eagle.
        wingspan: u32;
    }
}
```

By default, sub-types are encoded as objects with a special _tag field_.
This behavior can be controlled using the [`type_info`] attribute.

For example using `new Animal.Eagle("George", 8, 213)`:

```json
{"type": "eagle", "name": "George", "age": 8, "wingspan": 213}
```

### Interface sub-types

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

The body of the sub-type can contain fields, and attributes:

```reproto
interface Foo {
  Bar {
    name: string;
  }
}
```

### Interface attributes

The following are all attributes which can be added to interfaces.

* [`#[type_info(..)]`](#type-info) attribute, controls how type information is encoded in the JSON
  object.

The following is an example controlling which tag is used to encode the sub-type:

```reproto
#[type_info(strategy = "tagged", tag = "@class")]
interface Foo {
    /// Would be encoded as `{"@class": "bar"}`
    Bar as "bar";

    /// Would be encoded as `{"@class": "baz"}`
    Baz as "baz";
}
```

#### <a id="type-info" />`#[type_info(strategy = <string>, ...)]`

This attribute controls which strategy is used for determining sub-types in [interfaces].

Valid strategies are:

* [`tagged`], encode as an object with a special `tag` field indicating the sub-type.
* [`required_fields`], determine sub-type by its unique combination of required fields.

[interfaces]: #interfaces
[`tagged`]: #type-info-tagged
[`required_fields`]: #type-info-required-fields

#### <a id="type-info-tagged" />`#[type_info(strategy = "tagged", tag = <string>)]`

The default sub-type strategy.

Sub-types are encoded as objects, with a special tag field indicated by the `tag` selector.

By default interfaces are _internally tagged_, in that the field indicating the type lives in the
same level as the fields.

When interfaces are internally tagged, no field may conflict with the tag field as showcased here:

```reproto
interface Example {
    Foo as "foo" {
        type: string;
    }
}
```

The following is an example specification and the JSON it corresponds to:

```reproto
#[type_info(strategy = "tagged", tag = "@type")]
interface Example {
  Foo as "foo" {
    foo_field: u32;
  }

  Bar as "bar" {
    bar_field: u32;
  }
}
```

```json
{"@type": "foo", "foo_field": 42}
{"@type": "bar", "bar_field": 42}
```

#### <a id="type-info-required-fields" />`#[type_info(strategy = "required_fields")]`

Sub-types are encoded as objects, where the required fields of each sub-type determined which
sub-type is being encoded.

This strategy has the following restrictions:

 * Any given sub-type is not permitted to have an optional field that is a _required_ field in any
   other sub-type.
 * Sub-types are matched _in order_, so if two eligible sub-types are available, the first one
   will be used.

The following is an example specification and the JSON it corresponds to:

```reproto
#[type_info(strategy = "required_fields")]
interface Example {
  Foo {
    foo_field: u32;
  }

  Bar {
    bar_field: u32;
  }

  Baz;
}
```

```json
{"foo_field": 42}
{"bar_field": 42}
{}
```

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

Enums are types that can take on of a given set of constant values.
They convert loosely typed data like `"strings"` into something that can be checked by the
compiler or a decoder implementation.

Only `string` is currently supported as an enum type.

```reproto
enum Si as string {
    Nano as "nano";
    Micro as "micro";
    Milli as "milli";
    Kilo as "kilo";
    Mega as "mega";
}
```

Using this, `Si.Nano` would be serialized as:

```json
"nano"
```

Enum values do not have to be explicitly specified, but can be generated from the variant name:

```reproto
enum CarModel as string {
    /// JSON: `"Toyota"`
    Toyota;
    /// JSON: `"Mercedes"`
    Mercedes;
    /// JSON: `"Ford"`
    Ford;
    /// JSON: `"Volvo"`
    Volvo;
}
```

## Services

Services in reproto are currently modeled after [gRPC][grpc]
This means that they primarily operate on streams of requests and responses, see the
[next section](#bi-directional-services) for more details on what this means.

> **Note:** HTTP support has been punted, because the problem is much less _constrained_ than gRPC.
> Attempting to model all possible interactions you can have with HTTP services correctly is hard.
> Consistently generating code for them is even harder.

Service declarations describe a set of endpoints being exposed by a service.
Services are declared using the `service` keyword.

[grpc]: https://grpc.io

```reproto
/// My Service>
service MyService {
}
```

### Endpoints

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

Endpoints can have a set of attributes associated with them, by expanding their body.

These attributes might affect how code generation works for certain backends.

```reproto
type Foo {
}

/// My Service.
service MyService {
  /// Get foo.
  #[http(method = "POST")]
  get_foo() -> Foo;

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

### HTTP services

HTTP is supported in reproto through [attributes].

The primary attribute in use is the `#[http(...)]` selection.
This can be applied to [services] and [endpoints].

For services, the following attributes are available:

* `#[http(url = <string>)]`, configure the default URL for this service.

For endpoints, the following attributes are available:

* `#[http(path = <string>)]`, configure which path the endpoint uses. For example, `/post/{id}`.
  This attribute is _required_. See [HTTP paths] for more information.
* `#[http(method = <string>)]`, configure which method the endpoint uses. Defaults to `GET`.

[HTTP paths]: #http-paths
[services]: #services
[endpoints]: #endpoints
[attributes]: #attributes

### HTTP paths

Paths are specified using the `#[http(path = <string>)]` attribute on [endpoints].

Variables can be capture in paths using the `{var}` syntax.
This requires the variable to be declared in the endpoint, like the following:

```reproto
service MyService {
  #[http(path = "/posts/{id}", method = "DELETE")]
  delete_post(id: string);
}
```

Specifying a captured variable which is not present in the endpoint results in an error.

[endpoints]: #endpoints

### Bi-directional services

You might have noticed the `stream` keyword in the above examples.
This means that services are _bi-directional_.
Zero or more requests or responses of the given types may be sent, _in any order_.

This paradigm is more general than your typical unary request-response.

Calls against endpoints may also be long-lived, which would be useful for use-cases like streaming:

```reproto
type VideoId {
  id: u64;
}

type VideoFrame {
  blob: bytes;
}

service MyStreamingService {
  /// Stream frames for the given `VideoId`.
  stream_video(VideoId) -> stream VideoFrame;
}
```

**Note:** This is an example, JSON might not be suitable for streaming data like this.
This might be more viable if reproto supported other formats in the future.

## Reserved fields

Fields can be reserved using the `#[reserved(<field>)]` attribute.
Fields which are reserved _cannot_ be added to the schema.
Fields can be reserved on [types], [interfaces], and [sub-types].

```reproto
#[reserved(author, no_can_do)]
type Post {
  id: string;
}
```

Attempting to use a reserved field will result in an error:

```
it/ui/proto/reserved_fields_type_by_name.reproto:3:3-24:
  3:   foo: string as "bar";
       ^^^^^^^^^^^^^^^^^^^^^ - field with name `bar` is reserved
it/ui/proto/reserved_fields_type_by_name.reproto:1:12-17:
  1: #[reserved("bar")]
                ^^^^^ - reserved here
```

As long as the reserved statement is preserved, it prevents future introductions of a given field.

Clients decoding a reserved field should raise an error.

[types]: #types
[interfaces]: #interfaces
[sub-types]: #interface-sub-types

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
