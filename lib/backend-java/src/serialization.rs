//! Serialization strategy used for services.

use crate::core::errors::Result;
use genco::java::{self, Field};
use genco::{Java, Tokens};
use std::fmt;

#[derive(Clone, Copy, Debug)]
pub enum Serialization {
    Jackson,
}

impl Serialization {
    /// Get the field containing the serialization.
    pub fn field(&self) -> Field<'static> {
        use self::Serialization::*;

        match *self {
            Jackson => {
                let ty = java::imported("com.fasterxml.jackson.databind", "ObjectMapper");
                Field::new(ty, "mapper")
            }
        }
    }

    /// Setup the default builder for the serialization strategy.
    pub fn default_builder<'el>(&self) -> Option<Tokens<'el, Java<'el>>> {
        use self::Serialization::*;

        match *self {
            Jackson => {
                let ty = java::imported("io.reproto", "JacksonSupport");
                Some(toks![ty, ".objectMapper()"])
            }
        }
    }

    /// Decode argument for the given type.
    pub fn decode<'el, E>(
        &self,
        m: &Field<'el>,
        ty: &'el Java<'static>,
        i: &'el str,
        o: &'el str,
        exc: E,
    ) -> Result<Tokens<'el, Java<'el>>>
    where
        E: FnOnce(&'el str) -> Result<Tokens<'el, Java<'el>>>,
    {
        use self::Serialization::*;

        match *self {
            Jackson => {
                let arg = if !ty.is_generic() {
                    toks![ty, ".class"]
                } else {
                    let ty = java::imported("com.fasterxml.jackson.core.type", "TypeReference")
                        .with_arguments(vec![ty.clone()]);

                    toks!["new ", ty, "() {}"]
                };

                let mut t = Tokens::new();

                push!(t, "final ", ty, " ", o, ";");

                t.push({
                    let mut t = Tokens::new();

                    push!(t, "try {");
                    nested!(t, o, " = ", m.var(), ".readValue(", i, ", ", arg, ");");
                    push!(t, "} catch(final Exception e) {");
                    t.nested(exc("e")?);
                    push!(t, "}");

                    t
                });

                Ok(t.join_line_spacing())
            }
        }
    }
}

impl fmt::Display for Serialization {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        use self::Serialization::*;

        match *self {
            Jackson => "jackson".fmt(fmt),
        }
    }
}
