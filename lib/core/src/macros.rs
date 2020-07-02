/// Build a declaration body including common fields.
macro_rules! decl_body {
    (pub struct $name:ident<$f:ident> { $($rest:tt)* }) => {
        #[derive(Debug, Clone, serde::Serialize)]
        #[serde(bound = "F: serde::Serialize, F::Field: serde::Serialize, F::Endpoint: serde::Serialize, F::Package: serde::Serialize, F::Name: serde::Serialize, F::EnumType: serde::Serialize")]
        pub struct $name<$f: 'static> where $f: $crate::flavor::Flavor {
            pub name: $f::Name,
            pub ident: String,
            pub comment: Vec<String>,
            pub decls: Vec<$crate::rp_decl::RpDecl<$f>>,
            pub decl_idents: ::linked_hash_map::LinkedHashMap<String, usize>,
            $($rest)*
        }
    };
}

#[macro_export]
macro_rules! decl_flavor {
    ($vis:vis $flavor:ident, $source:ident) => {
        $vis type RpAccept = $source::RpAccept;
        $vis type RpCode = $source::RpCode;
        $vis type RpContext = $source::RpContext;
        $vis type RpDecl = $source::RpDecl<$flavor>;
        $vis type RpEndpoint = $source::RpEndpoint<$flavor>;
        $vis type RpEndpointArgument = $source::RpEndpointArgument<$flavor>;
        $vis type RpEndpointHttp = $source::RpEndpointHttp<$flavor>;
        $vis type RpEndpointHttp1 = $source::RpEndpointHttp1<$flavor>;
        $vis type RpEnumBody = $source::RpEnumBody<$flavor>;
        $vis type RpField = $source::RpField<$flavor>;
        $vis type RpFile = $source::RpFile<$flavor>;
        $vis type RpHttpMethod = $source::RpHttpMethod;
        $vis type RpInterfaceBody = $source::RpInterfaceBody<$flavor>;
        $vis type RpPathPart = $source::RpPathPart<$flavor>;
        $vis type RpPathSpec = $source::RpPathSpec<$flavor>;
        $vis type RpPathStep = $source::RpPathStep<$flavor>;
        $vis type RpReg = $source::RpReg;
        $vis type RpNamed<'a> = $source::RpNamed<'a, $flavor>;
        $vis type RpSubType = $source::RpSubType<$flavor>;
        $vis type RpTupleBody = $source::RpTupleBody<$flavor>;
        $vis type RpTypeBody = $source::RpTypeBody<$flavor>;
        $vis type RpChannel = $source::RpChannel<$flavor>;
        $vis type RpEnumType = $source::RpEnumType;
        $vis type RpName = $source::RpName<$flavor>;
        $vis type RpNumber = $source::RpNumber;
        $vis type RpPackage = $source::RpPackage;
        $vis type RpRequiredPackage = $source::RpRequiredPackage;
        $vis type RpServiceBody = $source::RpServiceBody<$flavor>;
        $vis type RpServiceBodyHttp = $source::RpServiceBodyHttp;
        $vis type RpSubTypeStrategy = $source::RpSubTypeStrategy;
        $vis type RpType = $source::RpType<$flavor>;
        $vis type RpValue = $source::RpValue<$flavor>;
        $vis type RpVariants = $source::RpVariants<$flavor>;
        $vis type RpVariant<V> = $source::RpVariant<$flavor, V>;
        $vis type RpVariantRef<'a> = $source::RpVariantRef<'a, $flavor>;
        $vis type RpVariantValue<'a> = $source::RpVariantValue<'a>;
        $vis type RpVersionedPackage = $source::RpVersionedPackage;
        $vis type Attributes = $source::Attributes<$flavor>;
        $vis type Selection = $source::Selection<$flavor>;
    };
}

#[macro_export]
macro_rules! translator_defaults {
    ($slf:ident $($rest:tt)*) => {
        $crate::translator_defaults!(@internal $slf $($rest)*);
    };

    (@internal $slf:ident, local_name $($rest:tt)*) => {
        fn translate_local_name<T>(
            &self,
            translator: &T,
            diag: &mut $crate::Diagnostics,
            _reg: $crate::RpReg,
            name: $crate::Spanned<$crate::RpName<$slf::Source>>,
        ) -> Result<$crate::Spanned<$crate::RpName<$slf::Target>>>
        where
            T: Translator<Source = $slf::Source, Target = $slf::Target>,
        {
            name.translate(diag, translator)
        }

        $crate::translator_defaults!(@internal $slf $($rest)*);
    };

    (@internal $slf:ident, field $($rest:tt)*) => {
        fn translate_field<T>(
            &self,
            translator: &T,
            diag: &mut $crate::Diagnostics,
            field: $crate::RpField<$slf::Source>,
        ) -> Result<$crate::RpField<$slf::Target>>
        where
            T: Translator<Source = $slf::Source, Target = $slf::Target>,
        {
            field.translate(diag, translator)
        }

        $crate::translator_defaults!(@internal $slf $($rest)*);
    };

    (@internal $slf:ident, endpoint $($rest:tt)*) => {
        fn translate_endpoint<T>(
            &self,
            translator: &T,
            diag: &mut $crate::Diagnostics,
            endpoint: $crate::RpEndpoint<$slf::Source>,
        ) -> Result<$crate::RpEndpoint<$slf::Target>>
        where
            T: Translator<Source = $slf::Source, Target = $slf::Target>,
        {
            endpoint.translate(diag, translator)
        }

        $crate::translator_defaults!(@internal $slf $($rest)*);
    };

    (@internal $slf:ident, rp_type $($rest:tt)*) => {
        fn translate_number(&self, number: RpNumberType) -> Result<RpType<$slf::Target>> {
            Ok(RpType::Number(number))
        }

        fn translate_float(&self) -> Result<RpType<$slf::Target>> {
            Ok(RpType::Float)
        }

        fn translate_double(&self) -> Result<RpType<$slf::Target>> {
            Ok(RpType::Double)
        }

        fn translate_boolean(&self) -> Result<RpType<$slf::Target>> {
            Ok(RpType::Boolean)
        }

        fn translate_string(&self, string: RpStringType) -> Result<RpType<$slf::Target>> {
            Ok(RpType::String(string))
        }

        fn translate_datetime(&self) -> Result<RpType<$slf::Target>> {
            Ok(RpType::DateTime)
        }

        fn translate_array(&self, inner: RpType<$slf::Target>) -> Result<RpType<$slf::Target>> {
            Ok(RpType::Array {
                inner: Box::new(inner),
            })
        }

        fn translate_map(
            &self,
            key: RpType<$slf::Target>,
            value: RpType<$slf::Target>,
        ) -> Result<RpType<$slf::Target>> {
            Ok(RpType::Map {
                key: Box::new(key),
                value: Box::new(value),
            })
        }

        fn translate_any(&self) -> Result<RpType<$slf::Target>> {
            Ok(RpType::Any)
        }

        fn translate_bytes(&self) -> Result<RpType<$slf::Target>> {
            Ok(RpType::Bytes)
        }

        fn translate_name(
            &self,
            _from: &<$slf::Target as Flavor>::Package,
            _reg: RpReg,
            name: Spanned<RpName<$slf::Target>>,
        ) -> Result<<$slf::Target as Flavor>::Type> {
            Ok(RpType::Name { name })
        }

        $crate::translator_defaults!(@internal $slf $($rest)*);
    };

    (@internal $slf:ident, enum_type $($rest:tt)*) => {
        fn translate_enum_type<T>(
            &self,
            _: &T,
            _: &mut $crate::Diagnostics,
            enum_type: $crate::RpEnumType,
        ) -> Result<<$slf::Target as Flavor>::EnumType>
        where
            T: Translator<Source = $slf::Source, Target = $slf::Target>,
        {
            Ok(enum_type)
        }

        $crate::translator_defaults!(@internal $slf $($rest)*);
    };

    (@internal $slf:ident) => {
    };
}
