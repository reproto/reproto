/// Build a declaration body including common fields.
macro_rules! decl_body {
    (pub struct $name:ident { $($rest:tt)* }) => {
        #[derive(Debug, Clone, Serialize)]
        pub struct $name {
            pub name: $crate::rp_name::RpName,
            pub local_name: String,
            pub comment: Vec<String>,
            pub attributes: $crate::attributes::Attributes,
            pub decls: Vec<$crate::rp_decl::RpDecl>,
            $($rest)*
        }
    };
}
