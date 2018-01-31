pub use super::*;
use genco::Cons;
use genco::java::{Argument, Field, Method, Modifier};
use std::rc::Rc;

/// A single field.
#[derive(Debug, Clone)]
pub struct JavaField<'a> {
    pub name: Cons<'a>,
    pub camel_name: Rc<String>,
    pub spec: Field<'a>,
}

impl<'el> JavaField<'el> {
    pub fn setter(&self) -> Option<Method<'el>> {
        if self.spec.modifiers.contains(&Modifier::Final) {
            return None;
        }

        let argument = Argument::new(self.spec.ty(), self.spec.var());
        let mut m = Method::new(Rc::new(format!("set{}", self.camel_name)));

        m.arguments.push(argument.clone());

        m.body
            .push(toks!["this.", self.spec.var(), " = ", argument.var(), ";",]);

        Some(m)
    }

    pub fn getter_without_body(&self) -> Method<'el> {
        let mut method = Method::new(Rc::new(format!("get{}", self.camel_name)));
        method.returns = self.spec.ty().as_field();
        method
    }

    pub fn getter(&self) -> Method<'el> {
        let mut m = self.getter_without_body();
        m.body.push(toks!["return this.", self.spec.var(), ";"]);
        m
    }
}
