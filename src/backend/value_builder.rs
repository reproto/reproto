use std::collections::HashMap;
use super::environment::Environment;
use super::errors::*;
use super::models as m;

pub type KnownValues<T> = HashMap<String, T>;

pub struct ValueBuilderEnv<'a> {
    pub package: &'a m::Package,
    pub variables: &'a m::Variables,
    pub value: &'a m::Token<m::Value>,
    pub ty: &'a m::Type,
}

fn new_env<'a>(package: &'a m::Package,
               variables: &'a m::Variables,
               value: &'a m::Token<m::Value>,
               ty: &'a m::Type)
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

    fn convert_type(&self,
                    pos: &m::Pos,
                    pkg: &m::Package,
                    custom: &m::Custom)
                    -> Result<Self::Type>;

    fn constant(&self, ty: Self::Type) -> Result<Self::Output>;

    fn instance(&self, ty: Self::Type, arguments: Vec<Self::Output>) -> Result<Self::Output>;

    fn identifier(&self, identifier: &str) -> Result<Self::Output>;

    fn value(&self, env: &ValueBuilderEnv) -> Result<Self::Output> {
        let value = env.value;
        let ty = env.ty;

        match (&**value, ty) {
            (&m::Value::String(ref string), &m::Type::String) => {
                return self.string(string);
            }
            (&m::Value::Boolean(ref boolean), &m::Type::Boolean) => {
                return self.boolean(boolean);
            }
            (&m::Value::Number(ref number), &m::Type::Signed(ref size)) => {
                return self.signed(number, size);
            }
            (&m::Value::Number(ref number), &m::Type::Unsigned(ref size)) => {
                return self.unsigned(number, size);
            }
            (&m::Value::Number(ref number), &m::Type::Float) => {
                return self.float(number);
            }
            (&m::Value::Number(ref number), &m::Type::Double) => {
                return self.double(number);
            }
            (&m::Value::Array(ref values), &m::Type::Array(ref inner)) => {
                let mut array_values = Vec::new();

                for v in values {
                    let new_env = new_env(&env.package, &env.variables, v, inner);
                    array_values.push(self.value(&*new_env)?)
                }

                return self.array(array_values);
            }
            (&m::Value::Constant(ref constant), &m::Type::Custom(ref target)) => {
                let reg_constant = self.env()
                    .constant(&value.pos, &env.package, constant, target)?;

                match *reg_constant {
                    m::Registered::EnumConstant { parent: _, value: _ } => {
                        let ty = self.convert_type(&value.pos, &env.package, target)?;
                        return self.constant(ty);
                    }
                    _ => return Err(Error::pos("not a valid constant".into(), value.pos.clone())),
                }
            }
            (&m::Value::Instance(ref instance), &m::Type::Custom(ref target)) => {
                let (registered, known) = self.env()
                    .instance(&value.pos, env.package, instance, target)?;

                let mut arguments = Vec::new();

                for f in registered.fields()? {
                    if let Some(init) = known.get(&f.name) {
                        let new_env = new_env(env.package, env.variables, &init.value, &f.ty);
                        arguments.push(self.value(&*new_env)?);
                    } else {
                        arguments.push(self.optional_empty()?);
                    }
                }

                let ty = self.convert_type(&value.pos, env.package, target)?;
                return self.instance(ty, arguments);
            }
            // identifier with any type.
            (&m::Value::Identifier(ref identifier), _) => {
                if let Some(variable_type) = env.variables.get(identifier) {
                    if self.env().is_assignable_from(ty, variable_type)? {
                        return Err(Error::pos("not assignable".into(), value.pos.clone()));
                    }

                    return self.identifier(identifier);
                } else {
                    return Err(Error::pos("missing variable".into(), value.pos.clone()));
                }
            }
            _ => {}
        }

        Err(Error::pos(format!("expected `{}`", ty), value.pos.clone()))
    }
}
