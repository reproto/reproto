/// Build a declaration body including common fields.
macro_rules! decl_body {
    (pub struct $name:ident<$f:ident> { $($rest:tt)* }) => {
        #[derive(Debug, Clone, Serialize)]
        #[serde(bound = "F: ::serde::Serialize, F::Field: ::serde::Serialize, F::Endpoint: ::serde::Serialize, F::Package: ::serde::Serialize")]
        pub struct $name<$f: 'static> where $f: $crate::flavor::Flavor {
            pub name: $crate::rp_name::RpName<$f>,
            pub ident: String,
            pub comment: Vec<String>,
            pub decls: Vec<$crate::rp_decl::RpDecl<$f>>,
            $($rest)*
        }
    };
}

#[macro_export]
macro_rules! decl_flavor {
    ($flavor:ident, $source:ident) => {
        pub type RpAccept = $source::RpAccept;
        pub type RpCode = $source::RpCode;
        pub type RpContext = $source::RpContext;
        pub type RpDecl = $source::RpDecl<$flavor>;
        pub type RpEndpoint = $source::RpEndpoint<$flavor>;
        pub type RpEndpointArgument = $source::RpEndpointArgument<$flavor>;
        pub type RpEndpointHttp = $source::RpEndpointHttp<$flavor>;
        pub type RpEndpointHttp1 = $source::RpEndpointHttp1<$flavor>;
        pub type RpEnumBody = $source::RpEnumBody<$flavor>;
        pub type RpField = $source::RpField<$flavor>;
        pub type RpFile = $source::RpFile<$flavor>;
        pub type RpHttpMethod = $source::RpHttpMethod;
        pub type RpInterfaceBody = $source::RpInterfaceBody<$flavor>;
        pub type RpPathPart = $source::RpPathPart<$flavor>;
        pub type RpPathSpec = $source::RpPathSpec<$flavor>;
        pub type RpPathStep = $source::RpPathStep<$flavor>;
        pub type RpReg = $source::RpReg;
        pub type RpNamed<'a> = $source::RpNamed<'a, $flavor>;
        pub type RpSubType = $source::RpSubType<$flavor>;
        pub type RpTupleBody = $source::RpTupleBody<$flavor>;
        pub type RpTypeBody = $source::RpTypeBody<$flavor>;
        pub type RpChannel = $source::RpChannel<$flavor>;
        pub type RpEnumOrdinal = $source::RpEnumOrdinal;
        pub type RpEnumType = $source::RpEnumType;
        pub type RpName = $source::RpName<$flavor>;
        pub type RpNumber = $source::RpNumber;
        pub type RpPackage = $source::RpPackage;
        pub type RpRequiredPackage = $source::RpRequiredPackage;
        pub type RpServiceBody = $source::RpServiceBody<$flavor>;
        pub type RpServiceBodyHttp = $source::RpServiceBodyHttp;
        pub type RpSubTypeStrategy = $source::RpSubTypeStrategy;
        pub type RpType = $source::RpType<$flavor>;
        pub type RpValue = $source::RpValue;
        pub type RpVariant = $source::RpVariant<$flavor>;
        pub type RpVersionedPackage = $source::RpVersionedPackage;
    };
}

/// Implement core type translation.
#[macro_export]
macro_rules! translator_core_types {
    ($target:path) => {
        fn translate_i32(&self) -> Result<RpType<$target>> {
            Ok(RpType::Signed { size: 32 })
        }

        fn translate_i64(&self) -> Result<RpType<$target>> {
            Ok(RpType::Signed { size: 64 })
        }

        fn translate_u32(&self) -> Result<RpType<$target>> {
            Ok(RpType::Unsigned { size: 32 })
        }

        fn translate_u64(&self) -> Result<RpType<$target>> {
            Ok(RpType::Unsigned { size: 64 })
        }

        fn translate_float(&self) -> Result<RpType<$target>> {
            Ok(RpType::Float)
        }

        fn translate_double(&self) -> Result<RpType<$target>> {
            Ok(RpType::Double)
        }

        fn translate_boolean(&self) -> Result<RpType<$target>> {
            Ok(RpType::Boolean)
        }

        fn translate_string(&self) -> Result<RpType<$target>> {
            Ok(RpType::String)
        }

        fn translate_datetime(&self) -> Result<RpType<$target>> {
            Ok(RpType::DateTime)
        }

        fn translate_array(&self, inner: RpType<$target>) -> Result<RpType<$target>> {
            Ok(RpType::Array {
                inner: Box::new(inner),
            })
        }

        fn translate_map(
            &self,
            key: RpType<$target>,
            value: RpType<$target>,
        ) -> Result<RpType<$target>> {
            Ok(RpType::Map {
                key: Box::new(key),
                value: Box::new(value),
            })
        }

        fn translate_any(&self) -> Result<RpType<$target>> {
            Ok(RpType::Any)
        }

        fn translate_bytes(&self) -> Result<RpType<$target>> {
            Ok(RpType::Bytes)
        }
    };
}

/// Implement core naming strategy.
#[macro_export]
macro_rules! translator_core_names {
    ($target:path) => {
        fn translate_name(
            &self,
            name: RpName<$target>,
            _reg: RpReg,
        ) -> Result<<$target as Flavor>::Type> {
            Ok(RpType::Name { name })
        }
    };
}
