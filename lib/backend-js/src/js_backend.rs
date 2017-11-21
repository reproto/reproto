use super::{JS_CONTEXT, TYPE, TYPE_SEP};
use backend::{Code, CompilerOptions, Converter, DynamicConverter, DynamicDecode, DynamicEncode,
              Environment, FromNaming, Naming, PackageUtils, SnakeCase};
use backend::errors::*;
use core::{ForEachLoc, Loc, RpEnumBody, RpField, RpInterfaceBody, RpModifier, RpName, RpTupleBody,
           RpType, RpTypeBody};
use genco::{Element, JavaScript, Quoted, Tokens};
use genco::js::imported_alias;
use js_compiler::JsCompiler;
use js_field::JsField;
use js_file_spec::JsFileSpec;
use js_options::JsOptions;
use listeners::Listeners;
use std::borrow::Cow;
use std::rc::Rc;
use utils::{is_defined, is_not_defined};

pub struct JsBackend {
    pub env: Environment,
    listeners: Box<Listeners>,
    id_converter: Option<Box<Naming>>,
    to_lower_snake: Box<Naming>,
    type_var: Tokens<'static, JavaScript<'static>>,
    values: Tokens<'static, JavaScript<'static>>,
    enum_name: Tokens<'static, JavaScript<'static>>,
}

impl JsBackend {
    pub fn new(
        env: Environment,
        _: JsOptions,
        listeners: Box<Listeners>,
        id_converter: Option<Box<Naming>>,
    ) -> JsBackend {
        JsBackend {
            env: env,
            listeners: listeners,
            id_converter: id_converter,
            to_lower_snake: SnakeCase::new().to_lower_snake(),
            type_var: TYPE.quoted().into(),
            values: "values".into(),
            enum_name: "name".into(),
        }
    }

    pub fn compiler(&self, options: CompilerOptions) -> Result<JsCompiler> {
        Ok(JsCompiler {
            out_path: options.out_path,
            backend: self,
        })
    }

    /// Build a function that throws an exception if the given value `toks` is None.
    fn throw_if_null<'el, S>(&self, toks: S, field: &JsField) -> Tokens<'el, JavaScript<'el>>
    where
        S: Into<Tokens<'el, JavaScript<'el>>>,
    {
        let required_error = format!("{}: is a required field", field.name).quoted();
        js![if is_not_defined(toks), js![throw required_error]]
    }

    fn encode_method<'el, B>(
        &self,
        fields: &[Loc<JsField<'el>>],
        builder: B,
        extra: Option<Tokens<'el, JavaScript<'el>>>,
    ) -> Result<Tokens<'el, JavaScript<'el>>>
    where
        B: Into<Tokens<'el, JavaScript<'el>>>,
    {
        let mut body = Tokens::new();

        body.push(toks!["const data = ", builder.into(), ";"]);

        if let Some(extra) = extra {
            body.push(extra);
        }

        let mut assign = Tokens::new();

        fields.for_each_loc(|field| {
            let var_string = field.name.quoted();
            let field_toks = toks!["this.", field.ident.clone()];
            let value_toks = self.dynamic_encode(field.ty, field_toks.clone())?;

            match *field.modifier {
                RpModifier::Optional => {
                    let toks = js![if is_defined(field_toks),
                                      toks!["data[", var_string, "] = ", value_toks, ";"]];
                    assign.push(toks);
                }
                _ => {
                    assign.push(self.throw_if_null(field_toks, field));
                    let toks = toks!["data[", var_string, "] = ", value_toks, ";"];
                    assign.push(toks);
                }
            }

            Ok(()) as Result<()>
        })?;

        if !assign.is_empty() {
            body.push(assign.join_line_spacing());
        }

        body.push(js![return "data"]);

        Ok({
            let mut t = Tokens::new();
            t.push("encode() {");
            t.nested(body.join_line_spacing());
            t.push("}");
            t
        })
    }

    fn encode_tuple_method<'el>(
        &self,
        fields: &[Loc<JsField<'el>>],
    ) -> Result<Tokens<'el, JavaScript<'el>>> {
        let mut values = Tokens::new();

        let mut body = Tokens::new();

        fields.for_each_loc(|field| {
            let toks = toks!["this.", field.ident.clone()];
            body.push(self.throw_if_null(toks.clone(), field));
            values.push(self.dynamic_encode(field.ty, toks)?);
            Ok(()) as Result<()>
        })?;

        body.push(js![@return [ values ]]);

        let mut encode = Tokens::new();
        encode.push("encode() {");
        encode.nested(body.join_line_spacing());
        encode.push("}");
        Ok(encode)
    }

    fn decode_enum_method<'el>(
        &self,
        type_name: Rc<String>,
        ident: Rc<String>,
    ) -> Result<Tokens<'el, JavaScript<'el>>> {
        let members = toks![type_name, ".", self.values.clone()];
        let loop_init = toks!["let i = 0, l = ", members.clone(), ".length"];
        let match_member = toks!["member.", ident, " === data"];

        let mut loop_body = Tokens::new();
        loop_body.push(toks!["const member = ", members, "[i]"]);
        loop_body.push(js![if match_member, toks!["return member;"]]);

        let mut member_loop = Tokens::new();
        member_loop.push(
            js![for loop_init; "i < l"; "i++", loop_body.join_line_spacing()],
        );

        let mut body = Tokens::new();
        body.push(member_loop);
        body.push(js![throw "no matching value: ".quoted(), " + data"]);

        let mut decode = Tokens::new();
        decode.push("static decode(data) {");
        decode.nested(body.join_line_spacing());
        decode.push("}");
        Ok(decode)
    }

    fn decode_method<'el, F>(
        &self,
        fields: &[Loc<JsField<'el>>],
        type_name: Rc<String>,
        variable_fn: F,
    ) -> Result<Tokens<'el, JavaScript<'el>>>
    where
        F: Fn(usize, &JsField<'el>) -> Element<'el, JavaScript<'el>>,
    {
        let mut arguments = Tokens::new();
        let mut assign = Tokens::new();

        for (i, field) in fields.iter().enumerate() {
            let var_name = Rc::new(format!("v_{}", field.ident.clone()));
            let var = variable_fn(i, field);

            let toks = field.as_ref().and_then(|field| match *field.modifier {
                RpModifier::Optional => {
                    let var_name = toks![var_name.clone()];
                    let var_toks = self.dynamic_decode(field.ty, var_name.clone())?;

                    let mut check = Tokens::new();

                    check.push(toks!["let ", var_name.clone(), " = data[", var, "];"]);
                    check.push(js![if is_defined(var_name.clone()),
                                      toks![var_name.clone(), " = ", var_toks, ";"],
                                      toks![var_name, " = null", ";"]]);

                    Ok(check.join_line_spacing().into()) as Result<Tokens<'el, JavaScript<'el>>>
                }
                _ => {
                    let var_toks = toks!["data[", var.clone(), "]"];
                    let var_toks = self.dynamic_decode(field.ty, var_toks.into())?;

                    let mut check = Tokens::new();

                    let var_name = toks![var_name.clone()];

                    check.push(toks!["const ", var_name.clone(), " = ", var_toks, ";"]);
                    check.push(js![if is_not_defined(var_name),
                                   js![throw var, " + ", ": required field".quoted()]]);

                    Ok(check.join_line_spacing().into()) as Result<Tokens<'el, JavaScript<'el>>>
                }
            })?;

            assign.push(toks);
            arguments.append(var_name);
        }

        let mut body = Tokens::new();

        if !assign.is_empty() {
            body.push(assign.join_line_spacing());
        }

        body.push(js![@return new type_name, arguments]);

        let mut decode = Tokens::new();
        decode.push("static decode(data) {");
        decode.nested(body.join_line_spacing());
        decode.push("}");
        Ok(decode)
    }

    fn field_by_name<'el>(_i: usize, field: &JsField<'el>) -> Element<'el, JavaScript<'el>> {
        field.name.quoted()
    }

    fn field_by_index<'el>(i: usize, _field: &JsField<'el>) -> Element<'el, JavaScript<'el>> {
        i.to_string().into()
    }

    fn field_ident(&self, field: &RpField) -> String {
        if let Some(ref id_converter) = self.id_converter {
            id_converter.convert(field.ident())
        } else {
            field.ident().to_owned()
        }
    }

    fn build_constructor<'el>(&self, fields: &[Loc<JsField<'el>>]) -> Tokens<'el, JavaScript<'el>> {
        let mut arguments = Tokens::new();
        let mut assignments = Tokens::new();

        for field in fields {
            arguments.append(field.ident.clone());
            assignments.push(toks![
                "this.",
                field.ident.clone(),
                " = ",
                field.ident.clone(),
                ";",
            ]);
        }

        let mut ctor = Tokens::new();
        ctor.push(toks!["constructor(", arguments.join(", "), ") {"]);
        ctor.nested(assignments);
        ctor.push("}");
        ctor
    }

    fn build_enum_constructor<'a, 'el>(&self, field: &JsField<'a>) -> Tokens<'el, JavaScript<'el>> {
        let mut arguments = Tokens::new();
        let mut assignments = Tokens::new();

        arguments.append(self.enum_name.clone());
        assignments.push(toks![
            "this.",
            self.enum_name.clone(),
            " = ",
            self.enum_name.clone(),
            ";",
        ]);

        arguments.append(field.ident.clone());
        assignments.push(toks![
            "this.",
            field.ident.clone(),
            " = ",
            field.ident.clone(),
            ";",
        ]);

        let mut ctor = Tokens::new();
        ctor.push(toks!["constructor(", arguments.join(", "), ") {"]);
        ctor.nested(assignments);
        ctor.push("}");
        ctor
    }

    fn enum_encode_decode<'a, 'el>(
        &self,
        field: &JsField<'a>,
        type_name: Rc<String>,
    ) -> Result<Tokens<'el, JavaScript<'el>>> {
        let mut elements = Tokens::new();

        elements.push({
            let mut encode = Tokens::new();
            encode.push("encode() {");
            encode.nested(js![return "this.", field.ident.clone()]);
            encode.push("}");
            encode
        });

        let decode = self.decode_enum_method(type_name, field.ident.clone())?;
        elements.push(decode);
        return Ok(elements.into());
    }

    fn build_getters<'el>(
        &self,
        fields: &[Loc<JsField<'el>>],
    ) -> Result<Vec<Tokens<'el, JavaScript<'el>>>> {
        let mut result = Vec::new();

        for field in fields {
            let name = Rc::new(self.to_lower_snake.convert(&field.ident));

            result.push({
                let mut tokens = Tokens::new();
                tokens.push(toks!["function get_", name.clone(), "() {"]);
                tokens.push(js![return "this.", name]);
                tokens.push("}");
                tokens
            });
        }

        Ok(result)
    }

    fn any_ident(field: JsField) -> JsField {
        field
    }

    fn enum_ident(field: JsField) -> JsField {
        match field.ident.as_str() {
            "name" => field.with_ident("_name".to_owned()),
            _ => field,
        }
    }

    fn into_js_field_with<'el, F>(&self, field: &'el RpField, js_field_f: F) -> JsField<'el>
    where
        F: Fn(JsField) -> JsField,
    {
        let ident = self.field_ident(&field);

        js_field_f(JsField {
            modifier: &field.modifier,
            ty: &field.ty,
            name: field.name(),
            ident: Rc::new(ident),
        })
    }

    fn into_js_field<'el>(&self, field: &'el RpField) -> JsField<'el> {
        self.into_js_field_with(field, Self::any_ident)
    }

    pub fn process_tuple<'el>(
        &self,
        out: &mut JsFileSpec<'el>,
        body: &'el RpTupleBody,
    ) -> Result<()> {
        let tuple_name = Rc::new(body.name.join(TYPE_SEP));
        let mut class_body = Tokens::new();

        let fields: Vec<Loc<JsField>> = body.fields
            .iter()
            .map(|f| f.as_ref().map(|f| self.into_js_field(f)))
            .collect();

        class_body.push(self.build_constructor(&fields));

        // TODO: make configurable
        if false {
            for getter in self.build_getters(&fields)? {
                class_body.push(getter);
            }
        }

        class_body.push(self.decode_method(
            &fields,
            tuple_name.clone(),
            Self::field_by_index,
        )?);

        class_body.push(self.encode_tuple_method(&fields)?);
        class_body.push_unless_empty(Code(&body.codes, JS_CONTEXT));

        let mut class = Tokens::new();

        class.push(toks!["export class ", tuple_name, " {"]);
        class.nested(class_body.join_line_spacing());
        class.push("}");

        out.0.push(class);
        Ok(())
    }

    /// Convert enum to JavaScript.
    pub fn process_enum<'el>(
        &self,
        out: &mut JsFileSpec<'el>,
        body: &'el Loc<RpEnumBody>,
    ) -> Result<()> {
        let type_name = Rc::new(body.name.join(TYPE_SEP));

        let mut class_body = Tokens::new();

        let variant_field = body.variant_type.as_field();

        let field = Loc::new(
            self.into_js_field_with(&variant_field, Self::enum_ident),
            body.pos().clone(),
        );

        let mut members = Tokens::new();

        class_body.push(self.build_enum_constructor(&field));
        class_body.push(self.enum_encode_decode(&field, type_name.clone())?);

        let mut values = Tokens::new();

        body.variants.iter().for_each_loc(|variant| {
            let mut arguments = Tokens::new();

            arguments.append(variant.local_name.as_str().quoted());
            arguments.append(self.ordinal(variant)?);

            let arguments = js![new body.local_name.as_str(), arguments];
            let member = toks![type_name.clone(), ".", variant.local_name.as_str()];

            values.push(js![= member.clone(), arguments]);
            members.append(member);

            Ok(()) as Result<()>
        })?;

        class_body.push_unless_empty(Code(&body.codes, JS_CONTEXT));

        let mut elements = Tokens::new();

        let mut class = Tokens::new();

        class.push(toks!["export class ", type_name.clone(), " {"]);
        class.nested(class_body.join_line_spacing());
        class.push("}");

        // class declaration
        elements.push(class);

        // enum literal values
        elements.push(values);

        // push members field
        let members_key = toks![type_name.clone(), ".", self.values.clone()];
        elements.push(js![= members_key, js!([members])]);

        out.0.push(elements.join_line_spacing());
        Ok(())
    }


    pub fn process_type<'el>(
        &self,
        out: &mut JsFileSpec<'el>,
        body: &'el RpTypeBody,
    ) -> Result<()> {
        let fields: Vec<Loc<JsField>> = body.fields
            .iter()
            .map(|f| f.as_ref().map(|f| self.into_js_field(f)))
            .collect();

        let type_name = Rc::new(body.name.join(TYPE_SEP));

        let mut class_body = Tokens::new();

        class_body.push(self.build_constructor(&fields));

        // TODO: make configurable
        if false {
            for getter in self.build_getters(&fields)? {
                class_body.push(getter);
            }
        }

        class_body.push(self.decode_method(
            &fields,
            type_name.clone(),
            Self::field_by_name,
        )?);

        class_body.push(self.encode_method(&fields, "{}", None)?);
        class_body.push_unless_empty(Code(&body.codes, JS_CONTEXT));

        let mut class = Tokens::new();

        class.push(toks!["export class ", type_name, " {"]);
        class.nested(class_body.join_line_spacing());
        class.push("}");

        out.0.push(class);
        Ok(())
    }

    pub fn process_interface<'el>(
        &self,
        out: &mut JsFileSpec<'el>,
        body: &'el RpInterfaceBody,
    ) -> Result<()> {
        let mut classes = Tokens::new();
        let interface_type_name = Rc::new(body.name.join(TYPE_SEP));

        let mut interface_body = Tokens::new();

        interface_body.push(self.interface_decode_method(&body)?);
        interface_body.push_unless_empty(Code(&body.codes, JS_CONTEXT));

        let interface_fields: Vec<Loc<JsField>> = body.fields
            .iter()
            .map(|f| f.as_ref().map(|f| self.into_js_field(f)))
            .collect();

        classes.push({
            let mut tokens = Tokens::new();

            tokens.push(toks!["export class ", interface_type_name.clone(), " {"]);
            tokens.nested(interface_body);
            tokens.push("}");

            tokens
        });

        let sub_types = body.sub_types.values().map(|l| l.loc_ref());

        sub_types.for_each_loc(|sub_type| {
            let type_name = Rc::new(sub_type.name.join(TYPE_SEP));

            let mut class_body = Tokens::new();

            let fields: Vec<Loc<JsField>> = interface_fields
                .iter()
                .cloned()
                .chain(sub_type.fields.iter().map(|f| {
                    f.as_ref().map(|f| self.into_js_field(f))
                }))
                .collect();

            class_body.push(self.build_constructor(&fields));

            // TODO: make configurable
            if false {
                for getter in self.build_getters(&fields)? {
                    class_body.push(getter);
                }
            }

            class_body.push(self.decode_method(
                &fields,
                type_name.clone(),
                Self::field_by_name,
            )?);

            let type_toks =
                toks![
                "data[",
                self.type_var.clone(),
                "] = ",
                interface_type_name.clone(),
                ".TYPE;",
            ];

            class_body.push(self.encode_method(&fields, "{}", Some(type_toks))?);
            class_body.push_unless_empty(Code(&sub_type.codes, JS_CONTEXT));

            classes.push({
                let mut tokens = Tokens::new();

                tokens.push(toks!["export class ", type_name.clone(), " {"]);
                tokens.nested(class_body);
                tokens.push("}");

                tokens
            });

            classes.push(toks![
                interface_type_name.clone(),
                ".TYPE",
                " = ",
                type_name.quoted(),
                ";",
            ]);

            Ok(()) as Result<()>
        })?;

        out.0.push(classes.join_line_spacing());
        Ok(())
    }
}

impl PackageUtils for JsBackend {}

impl<'el> Converter<'el> for JsBackend {
    type Custom = JavaScript<'el>;

    fn convert_type(&self, name: &'el RpName) -> Result<Tokens<'el, JavaScript<'el>>> {
        let registered = self.env.lookup(name)?;

        let local_name = registered.local_name(name, |p| p.join(TYPE_SEP), |c| c.join(TYPE_SEP));

        if let Some(ref used) = name.prefix {
            let package = self.package(&name.package).parts.join(".");
            return Ok(
                imported_alias(
                    Cow::Owned(package),
                    Cow::Owned(local_name),
                    Cow::Borrowed(used),
                ).into(),
            );
        }

        Ok(local_name.into())
    }
}

impl<'el> DynamicConverter<'el> for JsBackend {
    fn is_native(&self, ty: &RpType) -> bool {
        use self::RpType::*;

        match *ty {
            Signed { size: _ } |
            Unsigned { size: _ } => true,
            Float | RpType::Double => true,
            String => true,
            Any => true,
            Boolean => true,
            Array { ref inner } => self.is_native(inner),
            Map { ref key, ref value } => self.is_native(key) && self.is_native(value),
            _ => false,
        }
    }

    fn map_key_var(&self) -> Tokens<'el, JavaScript<'el>> {
        toks!["k"]
    }

    fn map_value_var(&self) -> Tokens<'el, JavaScript<'el>> {
        toks!["data[k]"]
    }

    fn array_inner_var(&self) -> Tokens<'el, JavaScript<'el>> {
        toks!["v"]
    }
}

impl<'el> DynamicDecode<'el> for JsBackend {
    fn name_decode(
        &self,
        input: Tokens<'el, JavaScript<'el>>,
        name: Tokens<'el, JavaScript<'el>>,
    ) -> Tokens<'el, JavaScript<'el>> {
        toks![name, ".decode(", input, ")"]
    }

    /// Decoding an Array in JavaScript.
    ///
    /// Maps over each decoded value using `Array.map(...)`, decoding each variable.
    fn array_decode(
        &self,
        input: Tokens<'el, JavaScript<'el>>,
        inner: Tokens<'el, JavaScript<'el>>,
    ) -> Tokens<'el, JavaScript<'el>> {
        toks![input, ".map(function(v) { return ", inner, "; })"]
    }

    /// Decoding a map in JavaScript.
    fn map_decode(
        &self,
        input: Tokens<'el, JavaScript<'el>>,
        _key: Tokens<'el, JavaScript<'el>>,
        value: Tokens<'el, JavaScript<'el>>,
    ) -> Tokens<'el, JavaScript<'el>> {
        let mut t = Tokens::new();

        t.append("(function(data) {");
        t.append(" let o = {};");
        t.append(" for (let k in data) {");
        t.append(toks![" o[k] = ", value, ";"]);
        t.append(" };");
        t.append(" return o;");
        t.append(toks![" })(", input, ")"]);

        t
    }

    fn assign_type_var(&self, data: &'el str, type_var: &'el str) -> Tokens<'el, JavaScript<'el>> {
        toks![
            "const ",
            type_var,
            " = ",
            data,
            "[",
            self.type_var.clone(),
            "]",
        ]
    }

    fn check_type_var(
        &self,
        data: &'el str,
        type_var: &'el str,
        name: &'el Loc<String>,
        type_name: Tokens<'el, Self::Custom>,
    ) -> Tokens<'el, JavaScript<'el>> {
        let mut body = Tokens::new();
        let cond = toks![type_var, " === ", name.as_str().quoted()];
        body.push(js![if cond, js![return type_name, ".decode(", data, ")"]]);
        body
    }

    fn raise_bad_type(&self, type_var: &'el str) -> Tokens<'el, JavaScript<'el>> {
        js![throw "bad type: ".quoted(), " + ", type_var]
    }

    fn new_decode_method(
        &self,
        data: &'el str,
        body: Tokens<'el, Self::Custom>,
    ) -> Tokens<'el, JavaScript<'el>> {
        let mut decode = Tokens::new();
        decode.push(toks!["static decode(", data, ") {"]);
        decode.nested(body);
        decode.push("}");
        decode
    }
}

impl<'el> DynamicEncode<'el> for JsBackend {
    fn name_encode(
        &self,
        input: Tokens<'el, JavaScript<'el>>,
        _: Tokens<'el, JavaScript<'el>>,
    ) -> Tokens<'el, JavaScript<'el>> {
        toks![input, ".encode()"]
    }

    fn array_encode(
        &self,
        input: Tokens<'el, JavaScript<'el>>,
        inner: Tokens<'el, JavaScript<'el>>,
    ) -> Tokens<'el, JavaScript<'el>> {
        toks![input, ".map(function(v) { return ", inner, "; })"]
    }

    fn map_encode(
        &self,
        input: Tokens<'el, JavaScript<'el>>,
        _: Tokens<'el, JavaScript<'el>>,
        value: Tokens<'el, JavaScript<'el>>,
    ) -> Tokens<'el, JavaScript<'el>> {
        let mut t = Tokens::new();

        t.append("(function(data) {");
        t.append(" let o = {};");
        t.append(" for (let k in data) {");
        t.append(toks![" o[k] = ", value, ";"]);
        t.append(" };");
        t.append(" return o;");
        t.append(toks![" })(", input, ")"]);

        t
    }
}
