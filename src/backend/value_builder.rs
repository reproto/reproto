//! # Helper trait to deal with value construction
//!
//! Value construction is when a literal value is encoded into the output.
//!
//! For example, when creating an instance of type `Foo(1, 2, 3)` in java could be translated to:
//!
//! ```java
//! new Foo(1, 2F, 3D)
//! ```
//!
//! In this example, the second field is a `float`, and the third field is a `double`.

use std::collections::HashMap;
use super::environment::Environment;
use super::errors::*;
use super::models as m;

pub type KnownValues<T> = HashMap<String, T>;

pub struct ValueBuilderEnv<'a> {
    pub package: &'a m::Package,
    pub variables: &'a m::Variables,
    pub value: &'a m::Token<m::Value>,
    pub ty: Option<&'a m::Type>,
}

fn new_env<'a>(package: &'a m::Package,
               variables: &'a m::Variables,
               value: &'a m::Token<m::Value>,
               ty: Option<&'a m::Type>)
               -> Box<ValueBuilderEnv<'a>> {
    Box::new(ValueBuilderEnv {
        package: package,
        variables: variables,
        value: value,
        ty: ty,
    })
}

pub trait ValueBuilder {
    type Output;
    type Type;

    fn env(&self) -> &Environment;

    fn signed(&self, number: &f64, _: &Option<usize>) -> Result<Self::Output> {
        self.number(number)
    }

    fn unsigned(&self, number: &f64, _: &Option<usize>) -> Result<Self::Output> {
        self.number(number)
    }

    fn float(&self, number: &f64) -> Result<Self::Output> {
        self.number(number)
    }

    fn double(&self, number: &f64) -> Result<Self::Output> {
        self.number(number)
    }

    fn string(&self, &str) -> Result<Self::Output>;

    fn boolean(&self, &bool) -> Result<Self::Output>;

    fn number(&self, &f64) -> Result<Self::Output>;

    fn array(&self, values: Vec<Self::Output>) -> Result<Self::Output>;

    fn optional_empty(&self) -> Result<Self::Output>;

    fn convert_type(&self, pos: &m::Pos, type_id: &m::TypeId) -> Result<Self::Type>;

    fn constant(&self, ty: Self::Type) -> Result<Self::Output>;

    fn instance(&self, ty: Self::Type, arguments: Vec<Self::Output>) -> Result<Self::Output>;

    fn identifier(&self, identifier: &str) -> Result<Self::Output>;

    fn value(&self, env: &ValueBuilderEnv) -> Result<Self::Output> {
        let value = env.value;
        let ty = env.ty;

        match (&**value, ty) {
            (&m::Value::String(ref string), Some(&m::Type::String)) |
            (&m::Value::String(ref string), None) => {
                return self.string(string);
            }
            (&m::Value::Boolean(ref boolean), Some(&m::Type::Boolean)) |
            (&m::Value::Boolean(ref boolean), None) => {
                return self.boolean(boolean);
            }
            (&m::Value::Number(ref number), None) => {
                return self.number(number);
            }
            (&m::Value::Number(ref number), Some(&m::Type::Signed(ref size))) => {
                return self.signed(number, size);
            }
            (&m::Value::Number(ref number), Some(&m::Type::Unsigned(ref size))) => {
                return self.unsigned(number, size);
            }
            (&m::Value::Number(ref number), Some(&m::Type::Float)) => {
                return self.float(number);
            }
            (&m::Value::Number(ref number), Some(&m::Type::Double)) => {
                return self.double(number);
            }
            (&m::Value::Array(ref values), expected) => {
                let inner = match expected {
                    Some(&m::Type::Array(ref inner)) => Some(&**inner),
                    Some(other) => {
                        return Err(Error::pos(format!("expected `{}`", other), value.pos.clone()))
                    }
                    None => None,
                };

                let mut array_values = Vec::new();

                for v in values {
                    let new_env = new_env(&env.package, &env.variables, v, inner);
                    array_values.push(self.value(&*new_env)?)
                }

                return self.array(array_values);
            }
            (&m::Value::Constant(ref constant), Some(&m::Type::Custom(ref target))) => {
                let reg_constant = self.env()
                    .constant(&value.pos, &env.package, constant, target)?;

                match *reg_constant {
                    m::Registered::EnumConstant { parent: _, value: _ } => {
                        let ty = self.convert_type(&value.pos, &env.package.into_type_id(target))?;
                        return self.constant(ty);
                    }
                    _ => return Err(Error::pos("not a valid constant".into(), value.pos.clone())),
                }
            }
            (&m::Value::Instance(ref instance), Some(&m::Type::Custom(ref target))) => {
                let (registered, known) = self.env()
                    .instance(&value.pos, env.package, instance, target)?;

                let mut arguments = Vec::new();

                for f in registered.fields()? {
                    if let Some(init) = known.get(&f.name) {
                        let new_env = new_env(env.package, env.variables, &init.value, Some(&f.ty));
                        arguments.push(self.value(&*new_env)?);
                    } else {
                        arguments.push(self.optional_empty()?);
                    }
                }

                let ty = self.convert_type(&value.pos, &env.package.into_type_id(target))?;
                return self.instance(ty, arguments);
            }
            // identifier with any type.
            (&m::Value::Identifier(ref identifier), _) => {
                if let Some(variable_type) = env.variables.get(identifier) {
                    // if expected is set
                    if let Some(ty) = ty {
                        if self.env().is_assignable_from(ty, variable_type)? {
                            return Err(Error::pos("not assignable".into(), value.pos.clone()));
                        }
                    }

                    return self.identifier(identifier);
                } else {
                    return Err(Error::pos("missing variable".into(), value.pos.clone()));
                }
            }
            _ => {}
        }

        if let Some(ty) = ty {
            Err(Error::pos(format!("expected `{}`", ty), value.pos.clone()))
        } else {
            Err(Error::pos("unexpected value".into(), value.pos.clone()))
        }
    }
}
