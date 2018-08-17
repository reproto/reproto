//! Path specifications

use errors::Result;
use std::fmt;
use std::vec;
use {Diagnostics, Flavor, RpEndpointArgument, Translate, Translator};

/// A part of a step.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(bound = "F::Type: ::serde::Serialize")]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum RpPathPart<F: 'static>
where
    F: Flavor,
{
    Variable(RpEndpointArgument<F>),
    Segment(String),
}

impl<F: 'static> fmt::Display for RpPathPart<F>
where
    F: Flavor,
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RpPathPart::Segment(ref segment) => {
                fmt.write_str(segment)?;
            }
            RpPathPart::Variable(ref var) => {
                fmt.write_str("{")?;
                fmt.write_str(var.safe_ident())?;
                fmt.write_str("}")?;
            }
        }

        Ok(())
    }
}

impl<F: 'static, T> Translate<T> for RpPathPart<F>
where
    F: Flavor,
    T: Translator<Source = F>,
{
    type Source = F;
    type Out = RpPathPart<T::Target>;

    /// Translate into different flavor.
    fn translate(self, diag: &mut Diagnostics, translator: &T) -> Result<RpPathPart<T::Target>> {
        use self::RpPathPart::*;

        let out = match self {
            Variable(arg) => Variable(arg.translate(diag, translator)?),
            Segment(segment) => Segment(segment),
        };

        Ok(out)
    }
}

/// A step in a path specification.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(bound = "F::Type: ::serde::Serialize")]
pub struct RpPathStep<F: 'static>
where
    F: Flavor,
{
    pub parts: Vec<RpPathPart<F>>,
}

impl<F: 'static> fmt::Display for RpPathStep<F>
where
    F: Flavor,
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("/")?;

        for p in &self.parts {
            p.fmt(fmt)?;
        }

        Ok(())
    }
}

impl<F: 'static, T> Translate<T> for RpPathStep<F>
where
    F: Flavor,
    T: Translator<Source = F>,
{
    type Source = F;
    type Out = RpPathStep<T::Target>;

    /// Translate into different flavor.
    fn translate(self, diag: &mut Diagnostics, translator: &T) -> Result<RpPathStep<T::Target>> {
        Ok(RpPathStep {
            parts: self.parts.translate(diag, translator)?,
        })
    }
}

/// A path specification.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(bound = "F::Type: ::serde::Serialize")]
pub struct RpPathSpec<F: 'static>
where
    F: Flavor,
{
    pub steps: Vec<RpPathStep<F>>,
}

#[derive(Debug)]
pub struct Vars<'a, F: 'static>
where
    F: Flavor,
{
    iter: vec::IntoIter<&'a RpEndpointArgument<F>>,
}

impl<'a, F: 'static> Iterator for Vars<'a, F>
where
    F: Flavor,
{
    type Item = &'a RpEndpointArgument<F>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl<F: 'static> RpPathSpec<F>
where
    F: Flavor,
{
    /// List all variables in the path spec.
    pub fn vars(&self) -> Vars<F> {
        let mut vars = Vec::new();

        for step in &self.steps {
            for part in &step.parts {
                if let RpPathPart::Variable(ref var) = *part {
                    vars.push(var);
                }
            }
        }

        Vars {
            iter: vars.into_iter(),
        }
    }
}

impl<F: 'static> fmt::Display for RpPathSpec<F>
where
    F: Flavor,
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        for s in &self.steps {
            s.fmt(fmt)?;
        }

        Ok(())
    }
}

impl<F: 'static, T> Translate<T> for RpPathSpec<F>
where
    F: Flavor,
    T: Translator<Source = F>,
{
    type Source = F;
    type Out = RpPathSpec<T::Target>;

    /// Translate into different flavor.
    fn translate(self, diag: &mut Diagnostics, translator: &T) -> Result<RpPathSpec<T::Target>> {
        Ok(RpPathSpec {
            steps: self.steps.translate(diag, translator)?,
        })
    }
}
