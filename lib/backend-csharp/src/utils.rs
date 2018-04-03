use genco::csharp::using;
use genco::{Cons, Csharp, IntoTokens, Quoted, Tokens};

/// [DataMember(..)] attribute
#[allow(unused)]
pub struct DataMember<'el> {
    name: Cons<'el>,
    emit_default_value: bool,
}

impl<'el> DataMember<'el> {
    /// Create a new `DataMember` attributes.
    #[allow(unused)]
    pub fn new(name: Cons<'el>) -> DataMember {
        DataMember {
            name: name,
            emit_default_value: false,
        }
    }
}

impl<'el> IntoTokens<'el, Csharp<'el>> for DataMember<'el> {
    fn into_tokens(self) -> Tokens<'el, Csharp<'el>> {
        let data_member = using("System.Runtime.Serialization", "DataMember");

        let mut args: Tokens<'el, Csharp<'el>> = Tokens::new();
        args.append(toks!["Name = ", self.name.quoted()]);
        args.append(toks![
            "EmitDefaultValue = ",
            self.emit_default_value.to_string(),
        ]);

        toks!["[", data_member, "(", args.join(", "), ")]"]
    }
}
