# Comparison with other systems

reproto has _plenty_ of overlap with other projects.

This section is not intended as a criticism of other projects, but rather a reflection on how
reproto differs from them.

As a general note, the [semantic versioning/semck][semck] story with reproto is tightly coupled
with dependency management.
Any system that doesn't address versioning and dependency management, can't check for violations in
semantic versioning either.

[semck]: /doc/semck.md

## OpenAPI (previously: Swagger)

Is primarily [a specification][openapi-spec], leading to inertia to develop further.
reproto aims to be self-contained in a single project that does everything you need.

Suffers from over-specification without sensible defaults, making it more verbose.

[Has a solution for field-based polymorphism](https://swagger.io/docs/specification/data-models/inheritance-and-polymorphism/), very verbose.

Compare:

```yaml
components:
  schemas:
    InterfaceObject:
      oneOf:
        - $ref: '#/components/schemas/SimpleObject'
        - $ref: '#/components/schemas/ComplexObject'
      discriminator:
        propertyName: type
    SimpleObject:
      type: object
      required:
        - type
      properties:
        type:
          type: string
    ComplexObject:
      type: object
      required:
        - type
      properties:
        type:
          type: string
```

With:

```reproto
interface InterfaceObject {
  SimpleObject;
  ComplexObject;
}
```

Doesn't solve dependency management or versioning.

Is based on YAML, which is harder to read.

[openapi-spec]: https://github.com/OAI/OpenAPI-Specification

## Protocol Buffers

Is a great system (one of the big inspirations), but interacts poorly with JSON.
JSON is still the de-facto way of interacting with browsers.
This leads to developers targeting the web to steer clear of it.
This in turn severely limits adoption and causes integration pains where protobuf is used in the
backend.

Because of the limiting factors in supporting a c/c++ backend, reproto probably never will.
For protobuf this has colored the protobuf implementation for other platforms, which has lead to it
generating code which exposes an excessive amount of unsafe APIs.
This is unsuitable for exposing through client libraries and frequently leads to an additional
layer of safe wrapping to make it palatable.

Very poor support for field-based polymorphism.

Doesn't solve dependency management or versioning. Google has no motivation to address it due to
their approach with [monorepos][google-monorepos].

**Note:** reproto might one day be taught how to interact with protocol buffers and translate them
to and from JSON.

[google-monorepos]: https://cacm.acm.org/magazines/2016/7/204032-why-google-stores-billions-of-lines-of-code-in-a-single-repository/fulltext

## GraphQL

I _really dig_ GraphQL.

Doesn't support field-based polymorphism without extensive hackery.

Doesn't solve dependency management or versioning.

## JSON Schema

Very poor ecosystem. JSON-based. Doesn't solve dependency management.
