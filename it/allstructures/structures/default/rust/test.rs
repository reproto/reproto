#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Entry {
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RootType {
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum RootInterface {
  Foo(RootInterface_Foo),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct RootInterface_Foo {
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum RootEnum {
  Foo,
}

impl RootEnum {
  pub fn value(&self) -> &'static str {
    use self::RootEnum::*;
    match *self {
      Foo => "Foo",
    }
  }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RootTuple();

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RootType_NestedType {
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum RootType_NestedInterface {
  Foo(RootType_NestedInterface_Foo),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct RootType_NestedInterface_Foo {
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum RootType_NestedEnum {
  Foo,
}

impl RootType_NestedEnum {
  pub fn value(&self) -> &'static str {
    use self::RootType_NestedEnum::*;
    match *self {
      Foo => "Foo",
    }
  }
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RootType_NestedTuple();

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RootInterface_Foo_NestedType {
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum RootInterface_Foo_NestedInterface {
  NestedFoo(RootInterface_Foo_NestedInterface_NestedFoo),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct RootInterface_Foo_NestedInterface_NestedFoo {
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum RootInterface_Foo_NestedEnum {
  Foo,
}

impl RootInterface_Foo_NestedEnum {
  pub fn value(&self) -> &'static str {
    use self::RootInterface_Foo_NestedEnum::*;
    match *self {
      Foo => "Foo",
    }
  }
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RootInterface_Foo_NestedTuple();

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RootTuple_NestedType {
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum RootTuple_NestedInterface {
  Foo(RootTuple_NestedInterface_Foo),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct RootTuple_NestedInterface_Foo {
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum RootTuple_NestedEnum {
  Foo,
}

impl RootTuple_NestedEnum {
  pub fn value(&self) -> &'static str {
    use self::RootTuple_NestedEnum::*;
    match *self {
      Foo => "Foo",
    }
  }
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RootTuple_NestedTuple();

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RootService_NestedType {
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum RootService_NestedInterface {
  Foo(RootService_NestedInterface_Foo),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct RootService_NestedInterface_Foo {
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum RootService_NestedEnum {
  Foo,
}

impl RootService_NestedEnum {
  pub fn value(&self) -> &'static str {
    use self::RootService_NestedEnum::*;
    match *self {
      Foo => "Foo",
    }
  }
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RootService_NestedTuple();

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RootType_NestedInterface_Foo_Nested {
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RootType_NestedTuple_Nested {
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RootType_NestedService_Nested {
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RootInterface_Foo_NestedInterface_NestedFoo_Nested {
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RootInterface_Foo_NestedTuple_Nested {
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RootInterface_Foo_NestedService_Nested {
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RootTuple_NestedInterface_Foo_Nested {
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RootTuple_NestedTuple_Nested {
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RootTuple_NestedService_Nested {
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RootService_NestedInterface_Foo_Nested {
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RootService_NestedTuple_Nested {
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RootService_NestedService_Nested {
}
