# ReProto Compiler 
[![Build Status](https://travis-ci.org/reproto/reproto.svg?branch=master)](https://travis-ci.org/reproto/reproto)
[![crates.io](https://img.shields.io/crates/v/reproto.svg)](https://crates.io/crates/reproto)

The ReProto project is a language-neutral protocol specification, aimed towards describing and
generating code for handling messages exchanged through JSON-based APIs.

ReProto specifies a [DSL][dsl] that described JSON types and services.
Using this, models can be generated using multiple different target languages.

* See [Specification][spec] for details on what the syntax of `.reproto` files is.
* See [TODO][todo] for details on things that still needs to be done.
* See [Examples][examples] for some example protocol specifications.
* See [Config][config] for how to configure ReProto.
* See [Integration Tests][it] for some examples of how protocol specifications can be used.

**Note:** This project is in an Alpha-stage. Things will change a lot.

[dsl]: #the-reproto-dsl
[spec]: /doc/spec.md
[todo]: /doc/todo.md
[config]: /doc/config.md
[examples]: /examples
[it]: /it

# Supported Backends

* Java (`java`)
  * Data models using [fasterxml jackson][jackson] (`-m fasterxml`), and/or
    [lombok][lombok] (`-m lombok`).
* JavaScript (`js`)
  * ES2015 classes, that can be transpiled using babel (see [Integration Test][js-it]).
* Python (`python`)
  * Plain-python classes, compatible with 2 and 3 for databinding.
* Rust (`rust`)
  * Serde-based serialization.
* Doc (`doc`)
  * HTML-based documentation, based from contextual markdown comments.

[lombok]: https://projectlombok.org/
[jackson]: https://github.com/FasterXML/jackson-databind
[js-it]: /it/js

# Examples

Make you have [gotten started with Rust][rust-get-started].

Build ReProto using cargo:

```bash
$> cargo build
```

This will install the command into `~/.cargo/bin`, make sure it is in your `$PATH`.

Build documentation:

```bash
$> target/debug/reproto compile doc -o target/doc --path it/test-service/proto \
  --package test \
  --package service@1.0.0 \
  --package service@2.0.0
$> open target/doc/index.html
```

For more example, please have a look at our [integration tests][it].

[rust-get-started]: https://doc.rust-lang.org/book/getting-started.html
[it]: /it

## [Maven Plugin][maven-plugin]

A Maven plugin that integrates reproto into the build lifecycle of a maven project.

[maven-plugin]: https://github.com/reproto/reproto-maven-plugin

## [VIM Plugin][vim]

A VIM plugin that provides syntax highlighting.

[vim]: https://github.com/reproto/reproto-vim

# Testing

This project includes an extensive set of integration tests.

See `make help` for documentation on what can be done.

Suites are tests which compiled a given set of rules, and compares with expected output.

Projects are complete project tests.
These are projects written for various programming languages, and are generally harder to build.

The tool [`check-project-deps`](tools/check-project-deps) is used to determine
which projects your local system can build.

To run all tests, do:

```bash
$> make clean all
```

# The ReProto DSL

The goal is to provide an intuitive and productive specification language.
For this reason, ReProto uses a DSL that is not based on existing markup (JSON, YAML, ...).
This is also reduces the signal to noise ratio.

As a comparison, the following is a specification using [OpenAPI 2.0][openapi-2], compared with ReProto.

```json
{
  "swagger": "2.0",
  "info": {
    "version": "1.0.0",
    "title": "Swagger Petstore",
    "description": "A sample API that uses a petstore as an example to demonstrate features in the swagger-2.0 specification",
    "termsOfService": "http://swagger.io/terms/",
    "contact": {
      "name": "Swagger API Team"
    },
    "license": {
      "name": "MIT"
    }
  },
  "host": "petstore.swagger.io",
  "basePath": "/api",
  "schemes": [
    "http"
  ],
  "consumes": [
    "application/json"
  ],
  "produces": [
    "application/json"
  ],
  "paths": {
    "/pets": {
      "get": {
        "description": "Returns all pets from the system that the user has access to",
        "produces": [
          "application/json"
        ],
        "responses": {
          "200": {
            "description": "A list of pets.",
            "schema": {
              "type": "array",
              "items": {
                "$ref": "#/definitions/Pet"
              }
            }
          }
        }
      }
    }
  },
  "definitions": {
    "Pet": {
      "type": "object",
      "required": [
        "id",
        "name"
      ],
      "properties": {
        "id": {
          "type": "integer",
          "format": "int64"
        },
        "name": {
          "type": "string"
        },
        "tag": {
          "type": "string"
        }
      }
    }
  }
}
```

```
/// # ReProto Petstore
///
/// A sample API that uses a petstore as an example to demonstrate features in the ReProto
/// specification
service Petstore {
  "api" {
    /// Returns all pets from the system that the user has access to.
    GET "pets" {
      /// A list of pets.
      returns [Pet] {
        status 200;
        produces "application/json";
      }
    }
  }
}

type Pet {
  id: unsigned/64;
  name: string;
  tag?: string;
}
```

You can compile the above into documentation using the following command:

```bash
$> target/debug/reproto --debug compile doc --out target/petstore \
  --path examples/petstore \
  --package petstore@1.0.0 \
```

If you miss JSON, you can compile the specification to JSON as well.

```bash
$> target/debug/reproto --debug compile json --out target/petstore-json \
  --path examples/petstore \
  --package petstore@1.0.0 \
```

[openapi-2]: https://github.com/OAI/OpenAPI-Specification
