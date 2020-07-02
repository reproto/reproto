use crate::flavored::*;
use crate::Compiler;
use core::Spanned;
use genco::prelude::*;
use std::rc::Rc;

macro_rules! decl_codegen {
    (
        $(
        $(#[$ty_m:meta])*
        $name:ident<$lt:lifetime> {
            $($(#[$m:meta])* $vis:vis $field:ident: $ty:ty,)*
        }
        )*
    ) => {
        $(
        $(#[$ty_m])*
        pub(crate) mod $name {
            use super::*;

            pub(crate) trait Codegen {
                fn generate(&self, e: Args<'_>);
            }

            pub(crate) struct Args<$lt> {
                $(
                    $(#[$m])*
                    $vis $field: $ty,
                )*
            }
        }
        )*

        #[derive(Default)]
        pub struct Generators {
            $(pub(crate) $name: Vec<Rc<dyn $name::Codegen>>,)*
        }

        impl Generators {
            $(
            $(#[$ty_m])*
            pub(crate) fn $name<$lt>(&self, $($field: $ty,)*) {
                for gen in &self.$name {
                    gen.generate($name::Args {
                        $($field,)*
                    });
                }
            }
            )*
        }
    }
}

decl_codegen! {
    /// Event emitted when a struct has been added.
    type_added<'a> {
        pub container: &'a mut Vec<swift::Tokens>,
        pub name: &'a Name,
        pub fields: &'a [Spanned<Field>],
    }

    /// Event emitted when a struct has been added.
    tuple_added<'a> {
        pub container: &'a mut Vec<swift::Tokens>,
        pub name: &'a Name,
        pub fields: &'a [Spanned<Field>],
    }

    /// Event emitted when a struct has been added.
    struct_model_added<'a> {
        pub container: &'a mut Vec<swift::Tokens>,
        pub fields: &'a [Spanned<Field>],
    }

    /// Event emitted when an enum has been added.
    enum_added<'a> {
        pub container: &'a mut Vec<swift::Tokens>,
        pub name: &'a Name,
        pub body: &'a RpEnumBody,
    }

    /// Event emitted when an interface has been added.
    interface_added<'a> {
        pub container: &'a mut Vec<swift::Tokens>,
        pub compiler: &'a Compiler<'a>,
        pub name: &'a Name,
        pub body: &'a RpInterfaceBody,
    }

    /// Event emitted when an interface model has been added.
    interface_model_added<'a> {
        pub container: &'a mut Vec<swift::Tokens>,
        pub body: &'a RpInterfaceBody,
    }

    /// Event emitted when an interface model has been added.
    package_added<'a> {
        pub files: &'a mut Vec<(RpPackage, swift::Tokens)>,
    }
}
