use crate::flavored::{Field, RpSubType, RpSubTypeStrategy, RpVariantRef, RpVariants};
use core::Loc;
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
    /// Generator used for classes.
    class<'a> {
        /// Annotations to add to the class.
        pub(crate) annotations: &'a mut Vec<csharp::Tokens>,
    }

    /// A class field was added.
    class_field<'a> {
        pub(crate) field: &'a Loc<Field>,
        /// Annotations to add to the field.
        pub(crate) annotations: &'a mut Vec<csharp::Tokens>,
    }

    /// A class constructor was added.
    class_constructor<'a> {
        /// Annotations to add to the field.
        pub(crate) annotations: &'a mut Vec<csharp::Tokens>,
    }

    /// A class constructor argument was added.
    class_constructor_arg<'a> {
        /// The fielda dded.
        pub(crate) field: &'a Loc<Field>,
        /// Annotations to add to the constructor argument.
        pub(crate) annotations: &'a mut Vec<csharp::Tokens>,
    }

    /// An enum was added.
    enum_type<'a> {
        /// Variants associated with the enum.
        pub(crate) variants: &'a RpVariants,
        /// Annotations to add to the enum typpe.
        pub(crate) annotations: &'a mut Vec<csharp::Tokens>,
    }

    /// An enum variant was added.
    enum_variant<'a> {
        /// The variant added.
        pub(crate) variant: RpVariantRef<'a>,
        /// The value being assigned to the variant.
        pub(crate) value: &'a mut Option<csharp::Tokens>,
        /// An enum variant was added.
        pub(crate) annotations: &'a mut Vec<csharp::Tokens>,
    }

    tuple<'a> {
        /// The identifier of the tuple.
        pub(crate) ident: &'a str,
        /// Fields in the tuple.
        pub(crate) fields: &'a [Loc<Field>],
        /// Annotations generated.
        pub(crate) annotations: &'a mut Vec<csharp::Tokens>,
        /// Inner code generated.
        pub(crate) inner: &'a mut Vec<csharp::Tokens>,
    }

    /// Generate code for an interface.
    interface<'a> {
        /// The identifier of the interface.
        pub(crate) ident: &'a str,
        /// The current sub type strategy.
        pub(crate) sub_type_strategy: &'a RpSubTypeStrategy,
        /// All known sub types.
        pub(crate) sub_types: &'a [Loc<RpSubType>],
        /// Annotations generated.
        pub(crate) annotations: &'a mut Vec<csharp::Tokens>,
        /// Annotations to use for the tag.
        pub(crate) tag_annotations: &'a mut Vec<csharp::Tokens>,
        /// Inner code generated.
        pub(crate) inner: &'a mut Vec<csharp::Tokens>,
    }

    /// Generate annotations for a tagged constructor.
    interface_tag_constructor_arg<'a> {
        /// The tag.
        pub(crate) tag: &'a str,
        /// Annotations generated.
        pub(crate) annotations: &'a mut Vec<csharp::Tokens>,
    }
}
