# codegen

codegen is a simple code generator for rust, specifically written for use in [reproto][reproto].

This project is inspired by JavaPoet (https://github.com/square/javapoet)

[reproto]: https://github.com/udoprog/reproto

## Example

```rust
#[macro_use]
extern crate codegen;

use codegen::java::*;

fn main() {
  let string_type = Type::class("java.lang", "String");
  let list_type = Type::class("java.util", "List");
  let json_creator_type = Type::class("com.fasterxml.jackson.annotation", "JsonCreator");
  let list_of_strings = list_type.with_arguments(vec![&string_type]);

  let values_field = FieldSpec::new(java_mods![Modifier::Private, Modifier::Final],
                                    &list_of_strings,
                                    "values");

  let values_argument =
      ArgumentSpec::new(java_mods![Modifier::Final], &list_of_strings, "values");

  let mut constructor = ConstructorSpec::new(java_mods![Modifier::Public]);
  constructor.push_annotation(AnnotationSpec::new(json_creator_type));
  constructor.push_argument(&values_argument);
  constructor.push(java_stmt!["this.values = ", values_argument]);

  let mut values_getter = MethodSpec::new(java_mods![Modifier::Public], "getValues");
  values_getter.returns(&list_of_strings);
  values_getter.push(java_stmt!["return this.", &values_field]);

  let mut class = ClassSpec::new(java_mods![Modifier::Public], "Test");
  class.push_field(&values_field);
  class.push_constructor(&constructor);
  class.push_method(&values_getter);

  let mut file = FileSpec::new("se.tedro");
  file.push_class(&class);

  let result = file.format();
}
```
