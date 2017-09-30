//! # Helper trait to deal with value construction
//!
//! RpValue construction is when a literal value is encoded into the output.
//!
//! For example, when creating an instance of type `Foo(1, 2, 3)` in java could be translated to:
//!
//! ```java
//! new Foo(1, 2F, 3D)
//! ```
//!
//! In this example, the second field is a `float`, and the third field is a `double`.

use converter::Converter;
use core::{Loc, RpCreator, RpNumber, RpRegistered, RpType, RpValue, RpVersionedPackage};
use environment::Environment;
use errors::*;
use variables::Variables;

pub struct ValueContext<'a> {
    package: &'a RpVersionedPackage,
    variables: &'a Variables<'a>,
    value: &'a Loc<RpValue>,
    expected: Option<&'a RpType>,
}

impl<'a> ValueContext<'a> {
    pub fn new(
        package: &'a RpVersionedPackage,
        variables: &'a Variables,
        value: &'a Loc<RpValue>,
        expected: Option<&'a RpType>,
    ) -> ValueContext<'a> {
        ValueContext {
            package: package,
            variables: variables,
            value: value,
            expected: expected,
        }
    }
}

/// Values which are being 'created', either through a new instance of an existing type or as an
/// enum constant.
pub struct CreatorContext<'a> {
    package: &'a RpVersionedPackage,
    variables: &'a Variables<'a>,
    creator: &'a Loc<RpCreator>,
    expected: Option<&'a RpType>,
}

impl<'a> CreatorContext<'a> {
    pub fn new(
        package: &'a RpVersionedPackage,
        variables: &'a Variables,
        creator: &'a Loc<RpCreator>,
        expected: Option<&'a RpType>,
    ) -> CreatorContext<'a> {
        CreatorContext {
            package: package,
            variables: variables,
            creator: creator,
            expected: expected,
        }
    }
}

pub trait ValueBuilder
where
    Self: Converter,
{
    fn env(&self) -> &Environment;

    fn signed(&self, number: &RpNumber, _: &Option<usize>) -> Result<Self::Stmt> {
        self.number(number)
    }

    fn unsigned(&self, number: &RpNumber, _: &Option<usize>) -> Result<Self::Stmt> {
        self.number(number)
    }

    fn float(&self, number: &RpNumber) -> Result<Self::Stmt> {
        self.number(number)
    }

    fn double(&self, number: &RpNumber) -> Result<Self::Stmt> {
        self.number(number)
    }

    fn string(&self, &str) -> Result<Self::Stmt>;

    fn boolean(&self, &bool) -> Result<Self::Stmt>;

    fn number(&self, &RpNumber) -> Result<Self::Stmt>;

    fn array(&self, values: Vec<Self::Stmt>) -> Result<Self::Stmt>;

    fn optional_of(&self, value: Self::Stmt) -> Result<Self::Stmt>;

    fn optional_empty(&self) -> Result<Self::Stmt>;

    fn constant(&self, ty: Self::Type) -> Result<Self::Stmt>;

    fn instance(&self, ty: Self::Type, arguments: Vec<Self::Stmt>) -> Result<Self::Stmt>;

    fn identifier(&self, identifier: &str) -> Result<Self::Stmt>;

    fn creator(&self, ctx: CreatorContext) -> Result<Self::Stmt> {
        use self::RpCreator::*;
        use self::RpRegistered::*;

        ctx.creator.as_ref().and_then(|creator| {
            match (creator, ctx.expected) {
                (&Constant(ref constant), Some(&RpType::Name { ref name })) => {
                    let reg_constant = self.env().constant(constant, name)?;

                    match *reg_constant {
                        EnumVariant(_, _) => {
                            return self.constant(self.convert_constant(constant.value())?);
                        }
                        _ => return Err("not a valid enum constant".into()),
                    }
                }
                (&Instance(ref instance), Some(&RpType::Name { ref name })) => {
                    let (registered, known) = self.env().instance(instance, name)?;

                    let mut arguments = Vec::new();

                    for f in registered.fields()? {
                        if let Some(init) = known.get(f.ident()) {
                            let ctx = ValueContext::new(
                                &ctx.package,
                                &ctx.variables,
                                &init.value,
                                Some(&f.ty),
                            );
                            let value = self.value(ctx)?;

                            let value = match f.is_optional() {
                                true => self.optional_of(value)?,
                                false => value,
                            };

                            arguments.push(value);
                        } else {
                            if !f.is_optional() {
                                return Err(
                                    format!("missing required parameter `{}`", f.ident()).into(),
                                );
                            }

                            arguments.push(self.optional_empty()?);
                        }
                    }

                    let ty = self.convert_type(&instance.name)?;
                    return self.instance(ty, arguments);
                }
                _ => {}
            }

            if let Some(expected) = ctx.expected {
                Err(format!("expected `{}`", expected).into())
            } else {
                Err("unexpected value".into())
            }
        })
    }

    fn value(&self, ctx: ValueContext) -> Result<Self::Stmt> {
        use self::RpValue::*;

        return ctx.value.as_ref().and_then(|value| {
            match (value, ctx.expected) {
                (&String(ref string), Some(&RpType::String)) |
                (&String(ref string), None) => {
                    return self.string(string);
                }
                (&Boolean(ref boolean), Some(&RpType::Boolean)) |
                (&Boolean(ref boolean), None) => return self.boolean(boolean),
                (&Number(ref number), None) => return self.number(number),
                (&Number(ref number), Some(&RpType::Signed { ref size })) => {
                    return self.signed(number, size);
                }
                (&Number(ref number), Some(&RpType::Unsigned { ref size })) => {
                    return self.unsigned(number, size);
                }
                (&Number(ref number), Some(&RpType::Float)) => {
                    return self.float(number);
                }
                (&Number(ref number), Some(&RpType::Double)) => {
                    return self.double(number);
                }
                (&Array(ref values), expected) => {
                    let inner = match expected {
                        Some(&RpType::Array { ref inner }) => Some(&**inner),
                        Some(other) => return Err(format!("expected `{}`", other).into()),
                        None => None,
                    };

                    let mut array_values = Vec::new();

                    for v in values {
                        let ctx = ValueContext::new(&ctx.package, &ctx.variables, v, inner);
                        array_values.push(self.value(ctx)?)
                    }

                    return self.array(array_values);
                }
                // identifier with any type.
                (&Identifier(ref identifier), expected) => {
                    if let Some(variable_type) = ctx.variables.get(identifier) {
                        // if expected is set
                        if let Some(expected) = expected {
                            if !self.env().is_assignable_from(
                                &ctx.package,
                                expected,
                                variable_type,
                            )?
                            {
                                return Err(format!("not assignable to `{}`", expected).into());
                            }
                        }

                        return self.identifier(identifier);
                    } else {
                        return Err("missing variable".into());
                    }
                }
                (&Creator(ref creator), expected) => {
                    return self.creator(CreatorContext::new(
                        &ctx.package,
                        &ctx.variables,
                        creator,
                        expected,
                    ));
                }
                _ => {}
            }

            if let Some(expected) = ctx.expected {
                Err(format!("expected `{}`", expected).into())
            } else {
                Err("unexpected value".into())
            }
        });
    }
}
