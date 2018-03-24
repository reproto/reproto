//! Path specifications

use {Flavor, RpEndpointArgument};
use std::rc::Rc;
use std::vec;

/// A part of a step.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum RpPathPart<F: 'static>
where
    F: Flavor,
{
    Variable(Rc<RpEndpointArgument<F>>),
    Segment(String),
}

/// A step in a path specification.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RpPathStep<F: 'static>
where
    F: Flavor,
{
    pub parts: Vec<RpPathPart<F>>,
}

/// A path specification.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
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
                    vars.push(Rc::as_ref(var));
                }
            }
        }

        Vars {
            iter: vars.into_iter(),
        }
    }
}
