use crate::flavored::{Field, RpSubType, RpSubTypeStrategy, Type};
use core::Spanned;
use genco::lang::java;
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
    /// Generator used for classes.
    class<'a> {
        /// The name of the type being generated for.
        pub(crate) ident: &'a str,
        /// Fields associated with class.
        pub(crate) fields: &'a [Spanned<Field>],
        /// Additional declarations.
        pub(crate) inner: &'a mut Vec<java::Tokens>,
        /// Annotations to add to the class.
        pub(crate) annotations: &'a mut Vec<java::Tokens>,
    }

    /// Generator used for interfaces.
    interface<'a> {
        /// The identifier for the interface.
        pub(crate) ident: &'a str,
        /// Sub types associated with the interface.
        pub(crate) sub_types: &'a [Spanned<RpSubType>],
        /// The sub type strategy associated with the interface.
        pub(crate) sub_type_strategy: &'a RpSubTypeStrategy,
        /// Annotations to add to the interface.
        pub(crate) annotations: &'a mut Vec<java::Tokens>,
        /// Innter content to add to the interface class.
        pub(crate) inner: &'a mut Vec<java::Tokens>,
    }

    /// Generator used for interface sub-types.
    interface_sub_type<'a> {
        /// The sub type strategy associated with the interface.
        pub(crate) sub_type_strategy: &'a RpSubTypeStrategy,
        /// Annotations to add to the interface.
        pub(crate) annotations: &'a mut Vec<java::Tokens>,
    }

    /// Generator used for class constructors.
    class_constructor<'a> {
        /// Fields associated with constructor.
        pub(crate) fields: &'a [Spanned<Field>],
        /// Annotations to add to the class.
        pub(crate) annotations: &'a mut Vec<java::Tokens>,
    }

    tuple<'a> {
        /// The name of the type being generated for.
        pub(crate) ident: &'a str,
        /// Fields associated with tuple.
        pub(crate) fields: &'a [Spanned<Field>],
        /// Additional inner declarations.
        pub(crate) inner: &'a mut Vec<java::Tokens>,
        /// Annotations to add to the class.
        pub(crate) annotations: &'a mut Vec<java::Tokens>,
    }

    enum_ty<'a> {
        /// The name of the type being generated for.
        pub(crate) ident: &'a str,
        /// The type of the enum.
        pub(crate) enum_type: &'a Type,
        /// Additional declarations generated.
        pub(crate) inner: &'a mut Vec<java::Tokens>,
    }

    class_constructor_arg<'a> {
        /// Field associated with the constructor argument.
        pub(crate) field: &'a Spanned<Field>,
        /// Annotations to add to the class.
        pub(crate) annotations: &'a mut Vec<java::Tokens>,
    }

    class_field<'a> {
        /// Field declaration being generated for.
        pub(crate) field: &'a Spanned<Field>,
        /// Annotations to add to the class field.
        pub(crate) annotations: &'a mut Vec<java::Tokens>,
    }

    class_getter<'a> {
        /// Field declaration being generated for.
        pub(crate) field: &'a Spanned<Field>,
        /// Annotations to add to the class field.
        pub(crate) annotations: &'a mut Vec<java::Tokens>,
    }
}
