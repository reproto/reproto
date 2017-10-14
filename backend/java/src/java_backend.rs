//! Java Backend for ReProto

use super::JAVA_CONTEXT;
use backend::{Converter, Environment, ForContext, FromNaming, Naming, SnakeCase};
use backend::errors::*;
use core::{ForEachLoc, Loc, RpDecl, RpEnumBody, RpEnumType, RpField, RpInterfaceBody, RpName,
           RpPackage, RpServiceBody, RpTupleBody, RpType, RpTypeBody, RpVersionedPackage};
use genco::{Cons, Element, IoFmt, Java, Quoted, Tokens, WriteTokens};
use genco::java::{Argument, BOOLEAN, Class, Constructor, DOUBLE, Enum, Extra, FLOAT, Field,
                  INTEGER, Interface, LONG, Method, Modifier, imported, local, optional};
use java_field::JavaField;
use java_options::JavaOptions;
use listeners::{ClassAdded, EnumAdded, InterfaceAdded, Listeners, TupleAdded};
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use std::rc::Rc;

pub struct JavaBackend {
    env: Environment,
    options: JavaOptions,
    listeners: Box<Listeners>,
    snake_to_upper_camel: Box<Naming>,
    snake_to_lower_camel: Box<Naming>,
    null_string: Element<'static, Java<'static>>,
    suppress_warnings: Java<'static>,
    string_builder: Java<'static>,
    void: Java<'static>,
    override_: Java<'static>,
    objects: Java<'static>,
    object: Java<'static>,
    list: Java<'static>,
    map: Java<'static>,
    string: Java<'static>,
    optional: Java<'static>,
    illegal_argument: Java<'static>,
    async_container: Java<'static>,
    byte_buffer: Java<'static>,
}

impl JavaBackend {
    pub fn new(env: Environment, options: JavaOptions, listeners: Box<Listeners>) -> JavaBackend {
        let async_container =
            options
                .async_container
                .as_ref()
                .map(Clone::clone)
                .unwrap_or_else(|| imported("java.util.concurrent", "CompletableFuture"));

        JavaBackend {
            env: env,
            options: options,
            snake_to_upper_camel: SnakeCase::new().to_upper_camel(),
            snake_to_lower_camel: SnakeCase::new().to_lower_camel(),
            null_string: "null".quoted(),
            listeners: listeners,
            void: imported("java.lang", "Void"),
            override_: imported("java.lang", "Override"),
            objects: imported("java.util", "Objects"),
            suppress_warnings: imported("java.lang", "SuppressWarnings"),
            string_builder: imported("java.lang", "StringBuilder"),
            object: imported("java.lang", "Object"),
            list: imported("java.util", "List"),
            map: imported("java.util", "Map"),
            string: imported("java.lang", "String"),
            optional: imported("java.util", "Optional"),
            illegal_argument: imported("java.lang", "IllegalArgumentException"),
            async_container: async_container,
            byte_buffer: imported("java.nio", "ByteBuffer"),
        }
    }

    pub fn compile(&self, out_path: &Path) -> Result<()> {
        self.env.for_each_toplevel_decl(|decl| {
            let package = self.java_package(&decl.name().package);
            let package_name = package.parts.join(".");

            let out_dir = package.parts.iter().fold(
                out_path.to_owned(),
                |current, next| current.join(next),
            );

            let full_path = out_dir.join(format!("{}.java", decl.local_name()));

            debug!("+class: {}", full_path.display());

            if let Some(out_dir) = full_path.parent() {
                if !out_dir.is_dir() {
                    debug!("+dir: {}", out_dir.display());
                    fs::create_dir_all(&out_dir)?;
                }
            }

            let mut file: Tokens<Java> = Tokens::new();
            let mut extra = Extra::default();
            extra.package(package_name);
            self.process_decl(decl.as_ref(), 0usize, &mut file)?;

            let mut f = File::create(full_path)?;
            IoFmt(&mut f).write_file(file, &mut extra)?;
            f.flush()?;

            Ok(())
        })
    }

    pub fn verify(&self) -> Result<()> {
        Ok(())
    }

    fn field_mods(&self) -> Vec<Modifier> {
        use self::Modifier::*;

        if self.options.immutable {
            vec![Private, Final]
        } else {
            vec![Private]
        }
    }

    fn new_field_spec<'el>(&self, ty: &Java<'el>, name: &'el str) -> Field<'el> {
        let mut field = Field::new(ty.clone(), name);
        field.modifiers = self.field_mods();
        field
    }

    /// Build the java package of a given package.
    ///
    /// This includes the prefixed configured in `self.options`, if specified.
    pub fn java_package(&self, pkg: &RpVersionedPackage) -> RpPackage {
        pkg.into_package(|version| {
            format!("_{}", version).replace(".", "_").replace("-", "_")
        })
    }

    pub fn java_package_name(&self, pkg: &RpVersionedPackage) -> Rc<String> {
        Rc::new(self.java_package(pkg).parts.join("."))
    }

    fn convert_type_id<'a, 'el>(&self, name: &'a RpName) -> Result<Java<'el>> {
        let registered = self.env.lookup(name)?;

        let package_name = self.java_package_name(&name.package);

        let name = Rc::new(registered.local_name(
            name,
            |p| p.join("."),
            |c| c.join("."),
        ));

        Ok(imported(package_name, name))
    }

    /// Convert the given type to a java type.
    pub fn into_java_type<'a, 'el>(&self, ty: &'a RpType) -> Result<Java<'el>> {
        use self::RpType::*;

        let out = match *ty {
            String => self.string.clone().into(),
            Signed { ref size } |
            Unsigned { ref size } => {
                // default to integer if unspecified.
                // TODO: should we care about signedness?
                // TODO: > 64 bits, use BitInteger?
                if size.map(|s| s <= 32usize).unwrap_or(true) {
                    INTEGER.into()
                } else {
                    LONG.into()
                }
            }
            Float => FLOAT.into(),
            Double => DOUBLE.into(),
            Boolean => BOOLEAN.into(),
            Array { ref inner } => {
                let argument = self.into_java_type(inner)?;
                self.list.with_arguments(vec![argument]).into()
            }
            Name { ref name } => self.convert_type_id(name)?,
            Map { ref key, ref value } => {
                let key = self.into_java_type(key)?;
                let value = self.into_java_type(value)?;
                self.map.with_arguments(vec![key, value]).into()
            }
            Any => self.object.clone().into(),
            Bytes => self.byte_buffer.clone().into(),
        };

        Ok(out)
    }

    fn build_constructor<'el>(&self, fields: &[JavaField<'el>]) -> Constructor<'el> {
        let mut c = Constructor::new();

        for field in fields {
            let field = &field.spec;

            let argument = Argument::new(field.ty(), field.var());

            if !self.options.nullable {
                if let Some(non_null) = self.require_non_null(field, &argument) {
                    c.body.push(non_null);
                }
            }

            c.arguments.push(argument.clone());

            c.body.push(toks![
                "this.",
                field.var(),
                " = ",
                argument.var(),
                ";",
            ]);
        }

        c
    }

    /// Build a require-non-null check.
    fn require_non_null<'el>(
        &self,
        field: &Field<'el>,
        argument: &Argument<'el>,
    ) -> Option<Tokens<'el, Java<'el>>> {
        use self::Java::*;

        match field.ty() {
            Primitive { .. } => None,
            _ => {
                let req = toks![self.objects.clone(), ".requireNonNull"];

                Some(toks![
                    req, "(", argument.var(), ", ", field.var().quoted(), ");",
                ])
            }
        }
    }

    fn build_hash_code<'el>(&self, fields: &[JavaField<'el>]) -> Method<'el> {
        let mut hash_code = Method::new("hashCode");

        hash_code.annotation(toks!["@", self.override_.clone()]);
        hash_code.returns = INTEGER;

        hash_code.body.push("int result = 1;");

        for field in fields {
            let field = &field.spec;

            let field_toks = toks!["this.", field.var()];

            let value = match field.ty() {
                ref primitive @ Java::Primitive { .. } => {
                    if *primitive == INTEGER {
                        field_toks.clone()
                    } else {
                        toks![primitive.as_boxed(), ".hashCode(", field_toks.clone(), ")"]
                    }
                }
                _ => toks![field_toks.clone(), ".hashCode()"],
            };

            let value = if self.options.nullable {
                match field.ty() {
                    Java::Primitive { .. } => value,
                    _ => toks!["(", field_toks.clone(), " != null ? 0 : ", value, ")"],
                }
            } else {
                value
            };

            hash_code.body.push(
                toks!["result = result * 31 + ", value, ";"],
            );
        }

        hash_code.body.push("return result;");
        hash_code
    }

    fn build_equals<'el>(&self, name: Cons<'el>, fields: &[JavaField<'el>]) -> Method<'el> {
        let argument = Argument::new(self.object.clone(), "other");

        let mut equals = Method::new("equals");

        equals.annotation(toks!["@", self.override_.clone()]);
        equals.returns = BOOLEAN;
        equals.arguments.push(argument.clone());

        // check if argument is null.
        {
            let mut null_check = Tokens::new();

            null_check.push(toks!["if (", argument.var(), " == null) {"]);
            null_check.nested("return false;");
            null_check.push("}");

            equals.body.push(null_check);
        }

        // check that argument is expected type.
        equals.body.push({
            let mut t = Tokens::new();

            t.push(toks![
                "if (!(", argument.var(), " instanceof ", name.clone(), ")) {",
            ]);
            t.nested("return false;");
            t.push("}");
            t
        });

        // cast argument.
        equals.body.push({
            let mut t = Tokens::new();

            t.push(toks![
                "@", self.suppress_warnings.clone(), "(", "unchecked".quoted(), ")",
            ]);

            t.push(toks![
                "final ", name.clone(), " o = (", name.clone(), ") ", argument.var(), ";",
            ]);

            t
        });

        for field in fields {
            let field = &field.spec;
            let field_toks = toks!["this.", field.var()];
            let o = toks!["o.", field.var()];

            let equals_condition = match field.ty() {
                Java::Primitive { .. } => toks![field_toks.clone(), " != ", o.clone()],
                _ => toks!["!", field_toks.clone(), ".equals(", o.clone(), ")"],
            };

            let mut equals_check = Tokens::new();

            equals_check.push(toks!["if (", equals_condition, ") {"]);
            equals_check.nested("return false;");
            equals_check.push("}");

            if self.options.nullable {
                let mut null_check = Tokens::new();

                null_check.push(toks!["if (", o, " != null) {"]);
                null_check.nested("return false;");
                null_check.push("}");

                let mut field_check = Tokens::new();

                field_check.push(toks!["if (", field_toks, " == null) {"]);
                field_check.nested(null_check);
                field_check.push("} else {");
                field_check.nested(equals_check);
                field_check.push("}");

                equals.body.push(field_check);
            } else {
                equals.body.push(equals_check);
            }
        }

        equals.body.push("return true;");
        equals.body = equals.body.join_line_spacing();

        equals
    }

    fn build_to_string<'el>(&self, name: Cons<'el>, fields: &[JavaField<'el>]) -> Method<'el> {
        let mut to_string = Method::new("toString");

        to_string.annotation(toks!["@", self.override_.clone()]);
        to_string.returns = self.string.clone();

        to_string.body.push(toks![
            "final ",
            self.string_builder.clone(),
            " b = new ",
            self.string_builder.clone(),
            "();",
        ]);

        let mut body = Tokens::new();

        for field in fields {
            let field = &field.spec;

            let field_toks = toks!["this.", field.var()];

            let format = match field.ty() {
                java @ Java::Primitive { .. } => {
                    toks![java.as_boxed(), ".toString(", field_toks.clone(), ")"]
                }
                _ => {
                    let format = toks![field_toks.clone(), ".toString()"];

                    if self.options.nullable {
                        toks![
                            field_toks.clone(),
                            " == null ? ",
                            self.null_string.clone(),
                            " : ",
                            format,
                        ]
                    } else {
                        format
                    }
                }
            };

            let field_key = Rc::new(format!("{}=", field.var().as_ref())).quoted();

            body.push({
                let mut t = Tokens::new();
                t.push(toks!["b.append(", field_key, ");"]);
                t.push(toks!["b.append(", format, ");"]);
                t
            });
        }

        // join each field with ", "
        let mut class_appends = Tokens::new();

        class_appends.push(toks!["b.append(", name.quoted(), ");"]);
        class_appends.push(toks![
            "b.append(",
            "(".quoted(),
            ");",
        ]);

        let sep = toks![Element::PushSpacing, "b.append(", ", ".quoted(), ");"];
        class_appends.push(body.join(sep));
        class_appends.push(toks![
            "b.append(",
            ")".quoted(),
            ");",
        ]);

        to_string.body.push(class_appends);
        to_string.body.push(toks!["return b.toString();"]);
        to_string.body = to_string.body.join_line_spacing();

        to_string
    }

    fn add_class<'el>(
        &self,
        name: Cons<'el>,
        fields: &[JavaField<'el>],
        methods: &mut Vec<Method<'el>>,
        constructors: &mut Vec<Constructor<'el>>,
    ) -> Result<()> {
        if self.options.build_constructor {
            constructors.push(self.build_constructor(fields));
        }

        if self.options.build_hash_code {
            methods.push(self.build_hash_code(fields));
        }

        if self.options.build_equals {
            methods.push(self.build_equals(name.clone(), fields));
        }

        if self.options.build_to_string {
            methods.push(self.build_to_string(name.clone(), fields));
        }

        Ok(())
    }

    fn build_enum_constructor<'el>(&self, fields: &[Field<'el>]) -> Constructor<'el> {
        use self::Modifier::*;

        let mut c = Constructor::new();
        c.modifiers = vec![Modifier::Private];

        for field in fields {
            let mut argument = Argument::new(field.ty(), field.var());
            argument.modifiers = vec![Final];

            if !self.options.nullable {
                if let Some(non_null) = self.require_non_null(&field, &argument) {
                    c.body.push(non_null);
                }
            }

            c.body.push(toks![
                "this.",
                field.var(),
                " = ",
                argument.var(),
                ";",
            ]);

            c.arguments.push(argument);
        }

        c
    }

    fn enum_from_value_method<'el>(&self, name: Cons<'el>, field: &Field<'el>) -> Method<'el> {
        use self::Modifier::*;

        let argument = Argument::new(field.ty(), field.var());

        // use naming convention that won't be generated
        let cond = match field.ty() {
            Java::Primitive { .. } => toks!["v_value.", field.var(), " == ", argument.var()],
            _ => toks!["v_value.", field.var(), ".equals(", argument.var(), ")"],
        };

        let mut return_matched = Tokens::new();

        return_matched.push(toks!["if (", cond, ") {"]);
        return_matched.nested(toks!["return v_value;"]);
        return_matched.push("}");

        let mut value_loop = Tokens::new();

        value_loop.push(
            toks!["for (final ", name.clone(), " v_value : ", "values()) {", ],
        );

        value_loop.nested(return_matched);
        value_loop.push("}");

        let mut from_value = Method::new("fromValue");

        from_value.modifiers = vec![Public, Static];
        from_value.returns = local(name.clone());

        let throw =
            toks![
            "throw new ",
            self.illegal_argument.clone(),
            "(",
            argument.var().quoted(),
            ");",
        ];

        from_value.body.push(value_loop);
        from_value.body.push(throw);
        from_value.body = from_value.body.join_line_spacing();

        from_value.arguments.push(argument);

        from_value
    }

    fn enum_to_value_method<'el>(&self, field: &Field<'el>) -> Method<'el> {
        let mut to_value = Method::new("toValue");
        to_value.returns = field.ty();
        to_value.body.push(toks!["return this.", field.var(), ";"]);
        to_value
    }

    fn enum_type_to_java<'a, 'el>(&self, ty: &'a RpEnumType) -> Result<Java<'el>> {
        use self::RpEnumType::*;

        match *ty {
            String => Ok(self.string.clone().into()),
            Generated => Ok(self.string.clone().into()),
        }
    }

    fn process_enum<'el>(&self, body: &'el RpEnumBody) -> Result<Enum<'el>> {
        let mut spec = Enum::new(body.local_name.clone());

        let enum_type = self.enum_type_to_java(&body.variant_type)?;
        spec.fields.push(self.new_field_spec(&enum_type, "value"));

        for variant in &body.variants {
            let mut enum_value = Tokens::new();
            let mut enum_toks = toks![variant.local_name.as_str()];

            let value = self.ordinal(variant)?;
            enum_toks.append(toks!["(", value, ")"]);

            enum_value.push(enum_toks);
            spec.variants.append(enum_value);
        }

        spec.constructors.push(
            self.build_enum_constructor(&spec.fields),
        );

        let variant_field = body.variant_type.as_field();
        let variant_java_field = self.convert_field(&variant_field)?;

        let mut from_value = self.enum_from_value_method(spec.name(), &variant_java_field.spec);
        let mut to_value = self.enum_to_value_method(&variant_java_field.spec);

        self.listeners.enum_added(&mut EnumAdded {
            body: body,
            spec: &mut spec,
            from_value: &mut from_value,
            to_value: &mut to_value,
        })?;

        spec.methods.push(from_value);
        spec.methods.push(to_value);

        for code in body.codes.for_context(JAVA_CONTEXT) {
            for line in &code.lines {
                spec.body.push(line.as_str());
            }
        }

        Ok(spec)
    }

    fn process_tuple<'el>(&self, body: &'el RpTupleBody) -> Result<Class<'el>> {
        let mut spec = Class::new(body.local_name.clone());

        let fields = self.convert_fields(&body.fields)?;

        self.add_class(
            spec.name(),
            &fields,
            &mut spec.methods,
            &mut spec.constructors,
        )?;

        for field in fields {
            if self.options.build_getters {
                spec.methods.push(field.getter());
            }

            if self.options.build_setters {
                if let Some(setter) = field.setter() {
                    spec.methods.push(setter);
                }
            }

            spec.fields.push(field.spec);
        }

        for code in body.codes.for_context(JAVA_CONTEXT) {
            for line in &code.lines {
                spec.body.push(line.as_str());
            }
        }

        self.listeners.tuple_added(
            &mut TupleAdded { spec: &mut spec },
        )?;

        Ok(spec)
    }

    fn process_type<'el>(&self, body: &'el RpTypeBody) -> Result<Class<'el>> {
        let mut spec = Class::new(body.local_name.clone());
        let fields = self.convert_fields(&body.fields)?;
        let names: Vec<_> = fields.iter().map(|f| f.name.clone()).collect();

        for field in &fields {
            spec.fields.push(field.spec.clone());

            if self.options.build_getters {
                spec.methods.push(field.getter());
            }

            if self.options.build_setters {
                if let Some(setter) = field.setter() {
                    spec.methods.push(setter);
                }
            }
        }

        for code in body.codes.for_context(JAVA_CONTEXT) {
            for line in &code.lines {
                spec.body.push(line.as_str());
            }
        }

        self.add_class(
            spec.name(),
            &fields,
            &mut spec.methods,
            &mut spec.constructors,
        )?;

        self.listeners.class_added(&mut ClassAdded {
            names: &names,
            spec: &mut spec,
        })?;

        Ok(spec)
    }

    fn process_interface<'el>(&self, body: &'el RpInterfaceBody) -> Result<Interface<'el>> {
        use self::Modifier::*;
        let mut spec = Interface::new(body.local_name.clone());
        let interface_fields = self.convert_fields(&body.fields)?;

        body.sub_types.values().for_each_loc(|sub_type| {
            let mut class = Class::new(sub_type.local_name.clone());
            class.modifiers = vec![Public, Static];

            let sub_type_fields = self.convert_fields(&sub_type.fields)?;

            for code in sub_type.codes.for_context(JAVA_CONTEXT) {
                for line in &code.lines {
                    class.body.push(line.as_str());
                }
            }

            class.implements = vec![local(spec.name())];

            // override methods for interface fields.
            for field in &interface_fields {
                if self.options.build_getters {
                    let mut getter = field.getter();
                    getter.annotation(toks!["@", self.override_.clone()]);
                    class.methods.push(getter);
                }

                if self.options.build_setters {
                    if let Some(mut setter) = field.setter() {
                        setter.annotation(toks!["@", self.override_.clone()]);
                        class.methods.push(setter);
                    }
                }
            }

            for field in &sub_type_fields {
                if self.options.build_getters {
                    class.methods.push(field.getter());
                }

                if self.options.build_setters {
                    if let Some(setter) = field.setter() {
                        class.methods.push(setter);
                    }
                }
            }

            let mut fields = interface_fields.to_vec();
            fields.extend(sub_type_fields);
            let names: Vec<_> = fields.iter().map(|f| f.name.clone()).collect();

            class.fields.extend(fields.iter().map(|f| f.spec.clone()));

            self.add_class(
                class.name(),
                &fields,
                &mut class.methods,
                &mut class.constructors,
            )?;

            self.listeners.class_added(&mut ClassAdded {
                names: &names,
                spec: &mut class,
            })?;

            spec.body.push(class);
            Ok(()) as Result<()>
        })?;

        self.listeners.interface_added(&mut InterfaceAdded {
            body: body,
            spec: &mut spec,
        })?;

        if self.options.build_getters {
            for field in &interface_fields {
                spec.methods.push(field.getter_without_body());
            }
        }

        for code in body.codes.for_context(JAVA_CONTEXT) {
            for line in &code.lines {
                spec.body.push(line.as_str());
            }
        }

        Ok(spec)
    }

    fn process_service<'el>(&self, body: &'el RpServiceBody) -> Result<Interface<'el>> {
        let mut spec = Interface::new(body.local_name.as_str());

        for endpoint in &body.endpoints {
            let mut ret: Option<Java> = None;

            for r in &endpoint.returns {
                let check = match r.status {
                    Some(200) => true,
                    Some(_) => false,
                    None => true,
                };

                if check {
                    ret = match r.ty {
                        Some(ref ret) => Some(self.into_java_type(ret)?),
                        None => None,
                    };
                }
            }

            for accept in &endpoint.accepts {
                if let Some(ref alias) = accept.alias {
                    let alias = Rc::new(self.snake_to_lower_camel.convert(alias));
                    let mut method = Method::new(alias);
                    method.modifiers = vec![];

                    if let Some(ref ret) = ret {
                        method.returns = self.async_container.with_arguments(vec![ret.clone()]);
                    } else {
                        method.returns =
                            self.async_container.with_arguments(vec![self.void.clone()]);
                    }

                    if let Some(ref ty) = accept.ty {
                        let arg = self.into_java_type(ty)?;
                        let arg = Argument::new(arg, "request");
                        method.arguments.push(arg);
                    }

                    spec.methods.push(method);
                }
            }
        }

        Ok(spec)
    }

    fn convert_field<'el>(&self, field: &RpField) -> Result<JavaField<'el>> {
        let java_value_type = self.into_java_type(&field.ty)?;

        let java_type = if field.is_optional() {
            optional(
                java_value_type.clone(),
                self.optional.with_arguments(vec![java_value_type.clone()]),
            )
        } else {
            java_value_type
        };

        let camel_name = Rc::new(self.snake_to_upper_camel.convert(field.ident()));
        let ident = Rc::new(self.snake_to_lower_camel.convert(field.ident()));

        let spec = Field::new(java_type, ident);

        Ok(JavaField {
            name: Rc::new(field.name().to_string()).into(),
            camel_name: camel_name,
            spec: spec,
        })
    }

    fn convert_fields<'el>(&self, fields: &'el [Loc<RpField>]) -> Result<Vec<JavaField<'el>>> {
        let mut out = Vec::new();

        fields.for_each_loc(|field| {
            out.push(self.convert_field(field)?);
            Ok(()) as Result<()>
        })?;

        Ok(out)
    }

    pub fn process_decl<'el>(
        &self,
        decl: &'el RpDecl,
        depth: usize,
        container: &mut Tokens<'el, Java<'el>>,
    ) -> Result<()> {
        match *decl {
            RpDecl::Interface(ref interface) => {
                let mut spec = self.process_interface(interface)?;

                for d in &interface.decls {
                    self.process_decl(d, depth + 1, &mut spec.body)?;
                }

                container.push(spec);
            }
            RpDecl::Type(ref ty) => {
                let mut spec = self.process_type(ty)?;

                // Inner classes should be static.
                if depth > 0 {
                    spec.modifiers.push(Modifier::Static);
                }

                for d in &ty.decls {
                    self.process_decl(d, depth + 1, &mut spec.body)?;
                }

                container.push(spec);
            }
            RpDecl::Tuple(ref ty) => {
                let mut spec = self.process_tuple(ty)?;

                // Inner classes should be static.
                if depth > 0 {
                    spec.modifiers.push(Modifier::Static);
                }

                for d in &ty.decls {
                    self.process_decl(d, depth + 1, &mut spec.body)?;
                }

                container.push(spec);
            }
            RpDecl::Enum(ref ty) => {
                let mut spec = self.process_enum(ty)?;

                for d in &ty.decls {
                    self.process_decl(d, depth + 1, &mut spec.body)?;
                }

                container.push(spec);
            }
            RpDecl::Service(ref ty) => {
                let spec = self.process_service(ty)?;
                container.push(spec);
            }
        }

        Ok(())
    }
}

impl<'el> Converter<'el> for JavaBackend {
    type Custom = Java<'el>;

    fn convert_type(&self, name: &'el RpName) -> Result<Tokens<'el, Self::Custom>> {
        Ok(toks![self.convert_type_id(name)?])
    }
}
