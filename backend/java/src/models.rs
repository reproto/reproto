pub use super::*;

/// A single field.
#[derive(Debug, Clone)]
pub struct JavaField<'a> {
    pub modifier: &'a RpModifier,
    pub ty: &'a RpType,
    pub camel_name: String,
    pub name: &'a str,
    pub ident: String,
    pub java_type: Type,
    pub java_spec: FieldSpec,
}

impl<'a> JavaField<'a> {
    pub fn setter(&self) -> Result<Option<MethodSpec>> {
        if self.java_spec.modifiers.contains(&Modifier::Final) {
            return Ok(None);
        }

        let name = format!("set{}", self.camel_name);
        let mut setter = MethodSpec::new(mods![Modifier::Public], &name);

        let argument = ArgumentSpec::new(mods![Modifier::Final], &self.java_type, &self.ident);

        setter.push_argument(&argument);
        setter.returns(VOID);

        let mut method_body = Elements::new();

        method_body.push(stmt!["this.", &self.ident, " = ", &argument, ";"]);
        setter.push(method_body);

        Ok(Some(setter))
    }

    pub fn getter_without_body(&self) -> Result<MethodSpec> {
        let name = format!("get{}", self.camel_name);
        let mut getter = MethodSpec::new(mods![Modifier::Public], &name);

        if *self.modifier == RpModifier::Optional {
            let optional = Type::class("java.util", "Optional");
            getter.returns(optional.with_arguments(vec![&self.java_type]));
        } else {
            getter.returns(&self.java_type);
        }

        Ok(getter)
    }

    pub fn getter(&self) -> Result<MethodSpec> {
        let mut getter = self.getter_without_body()?;
        getter.push(stmt!["return this.", &self.ident, ";"]);
        Ok(getter)
    }
}
