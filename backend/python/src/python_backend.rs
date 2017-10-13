//! Python Backend

use super::{PYTHON_CONTEXT, TYPE, TYPE_SEP};
use backend::{CompilerOptions, Converter, DynamicConverter, DynamicDecode, DynamicEncode,
              Environment, ForContext, FromNaming, Naming, PackageUtils, SnakeCase, ValueBuilder};
use backend::errors::*;
use core::{ForEachLoc, Loc, RpEnumBody, RpField, RpInterfaceBody, RpModifier, RpName, RpTupleBody,
           RpType, RpTypeBody, WithPos};
use genco::{Element, Quoted, Tokens};
use genco::python::{Python, imported_alias, imported_ref};
use listeners::Listeners;
use python_compiler::PythonCompiler;
use python_field::PythonField;
use python_file_spec::PythonFileSpec;
use python_options::PythonOptions;
use std::borrow::Cow;
use std::iter;
use std::rc::Rc;

pub struct PythonBackend {
    pub env: Environment,
    listeners: Box<Listeners>,
    id_converter: Option<Box<Naming>>,
    to_lower_snake: Box<Naming>,
    dict: Element<'static, Python<'static>>,
    enum_enum: Python<'static>,
    type_var: Tokens<'static, Python<'static>>,
}

impl PythonBackend {
    pub fn new(
        env: Environment,
        _: PythonOptions,
        listeners: Box<Listeners>,
        id_converter: Option<Box<Naming>>,
    ) -> PythonBackend {
        PythonBackend {
            env: env,
            listeners: listeners,
            id_converter: id_converter,
            to_lower_snake: SnakeCase::new().to_lower_snake(),
            dict: "dict".into(),
            enum_enum: imported_ref("enum", "Enum"),
            type_var: TYPE.into(),
        }
    }

    pub fn compiler(&self, options: CompilerOptions) -> Result<PythonCompiler> {
        Ok(PythonCompiler {
            out_path: options.out_path,
            backend: self,
        })
    }

    pub fn verify(&self) -> Result<()> {
        Ok(())
    }

    /// Build a function that raises an exception if the given value `toks` is None.
    fn raise_if_none<'el>(
        &self,
        toks: Tokens<'el, Python<'el>>,
        field: &PythonField,
    ) -> Tokens<'el, Python<'el>> {
        let mut raise_if_none = Tokens::new();
        let required_error = format!("{}: is a required field", field.name).quoted();

        raise_if_none.push(toks!["if ", toks, " is None:"]);
        raise_if_none.nested(toks!["raise Exception(", required_error, ")"]);

        raise_if_none
    }

    fn encode_method<'el>(
        &self,
        fields: &[Loc<PythonField<'el>>],
        builder: Tokens<'el, Python<'el>>,
        extra: Option<Tokens<'el, Python<'el>>>,
    ) -> Result<Tokens<'el, Python<'el>>> {
        let mut encode_body = Tokens::new();

        encode_body.push(toks!["data = ", builder.clone(), "()"]);

        if let Some(extra) = extra {
            encode_body.push(extra);
        }

        fields.for_each_loc(|field| {
            let var_string = field.name.quoted();
            let field_toks = toks!["self.", field.ident.clone()];

            let value_toks = self.dynamic_encode(field.ty, field_toks.clone())?;

            match *field.modifier {
                RpModifier::Optional => {
                    let mut check_if_none = Tokens::new();

                    check_if_none.push(toks!["if ", field_toks, " is not None:"]);

                    let toks = toks!["data[", var_string, "] = ", value_toks];

                    check_if_none.nested(toks);

                    encode_body.push(check_if_none);
                }
                _ => {
                    encode_body.push(self.raise_if_none(field_toks, field));

                    let toks = toks!["data[", var_string, "] = ", value_toks];

                    encode_body.push(toks);
                }
            }

            Ok(()) as Result<()>
        })?;

        encode_body.push(toks!["return data"]);

        let mut encode = Tokens::new();
        encode.push("def encode(self):");
        encode.nested(encode_body.join_line_spacing());
        Ok(encode)
    }

    fn encode_tuple_method<'el>(
        &self,
        fields: &[Loc<PythonField<'el>>],
    ) -> Result<Tokens<'el, Python<'el>>> {
        let mut values = Tokens::new();
        let mut encode_body = Tokens::new();

        for field in fields {
            let toks = toks!["self.", field.ident.clone()];
            encode_body.push(self.raise_if_none(toks.clone(), field));
            values.append(self.dynamic_encode(field.ty, toks).with_pos(field.pos())?);
        }

        encode_body.push(toks!["return (", values.join(", "), ")"]);

        let mut encode = Tokens::new();
        encode.push("def encode(self):");
        encode.nested(encode_body.join_line_spacing());
        Ok(encode)
    }

    fn encode_enum_method<'el>(&self, field: &PythonField) -> Result<Tokens<'el, Python<'el>>> {
        let mut encode = Tokens::new();
        encode.push("def encode(self):");
        encode.nested(toks!["return self.", field.ident.clone()]);
        Ok(encode)
    }

    fn decode_enum_method<'el>(&self, field: &PythonField) -> Result<Tokens<'el, Python<'el>>> {
        let mut decode_body = Tokens::new();

        let mut check = Tokens::new();
        check.push(toks!["if value.", field.ident.clone(), " == data:"]);
        check.nested(toks!["return value"]);

        let mut member_loop = Tokens::new();

        member_loop.push("for value in cls.__members__.values():");
        member_loop.nested(check);

        decode_body.push(member_loop);
        decode_body.push(toks![
            "raise Exception(", "data does not match enum".quoted(), ")",
        ]);

        let mut decode = Tokens::new();
        decode.push("@classmethod");
        decode.push("def decode(cls, data):");
        decode.nested(decode_body.join_line_spacing());
        Ok(decode)
    }

    fn repr_method<'a, 'el, I>(&self, name: Rc<String>, fields: I) -> Tokens<'el, Python<'el>>
    where
        I: IntoIterator<Item = &'a Loc<PythonField<'a>>>,
    {
        let mut args = Vec::new();
        let mut vars = Tokens::new();

        for field in fields {
            args.push(format!("{}: {{!r}}", field.ident.as_str()));
            vars.append(toks!["self.", field.ident.clone()]);
        }

        let format = format!("<{} {}>", name, args.join(", "));

        let mut repr = Tokens::new();
        repr.push("def __repr__(self):");
        repr.nested(toks![
            "return ",
            format.quoted(),
            ".format(",
            vars.join(", "),
            ")",
        ]);
        repr
    }

    fn optional_check<'el>(
        &self,
        var: Tokens<'el, Python<'el>>,
        index: Tokens<'el, Python<'el>>,
        toks: Tokens<'el, Python<'el>>,
    ) -> Tokens<'el, Python<'el>> {
        let mut check = Tokens::new();

        let mut none_check = Tokens::new();
        none_check.push(toks![var.clone(), " = data[", index.clone(), "]"]);

        let mut none_check_if = Tokens::new();

        let assign_var = toks![var.clone(), " = ", toks];

        none_check_if.push(toks!["if ", var.clone(), " is not None:"]);
        none_check_if.nested(assign_var);

        none_check.push(none_check_if);

        check.push(toks!["if ", index.clone(), " in data:"]);
        check.nested(none_check.join_line_spacing());

        check.push(toks!["else:"]);
        check.nested(toks![var.clone(), " = None"]);

        check.into()
    }

    fn decode_method<'el, F>(
        &self,
        name: &'el RpName,
        fields: &[Loc<PythonField<'el>>],
        variable_fn: F,
    ) -> Result<Tokens<'el, Python<'el>>>
    where
        F: Fn(usize, &PythonField<'el>) -> Tokens<'el, Python<'el>>,
    {
        let mut body = Tokens::new();
        let mut args = Tokens::new();

        for (i, field) in fields.into_iter().enumerate() {
            let var_name = Rc::new(format!("f_{}", field.ident));
            let var = variable_fn(i, field);

            let toks = match *field.modifier {
                RpModifier::Optional => {
                    let var_name = toks!(var_name.clone());
                    let var_toks = self.dynamic_decode(field.ty, var_name.clone()).with_pos(
                        field.pos(),
                    )?;
                    self.optional_check(var_name.clone(), var, var_toks)
                }
                _ => {
                    let data = toks!["data[", var.clone(), "]"];
                    let var_toks = self.dynamic_decode(field.ty, data).with_pos(field.pos())?;
                    toks![var_name.clone(), " = ", var_toks]
                }
            };

            body.push(toks);
            args.append(toks!(var_name));
        }

        let args = args.join(", ");
        let name = self.convert_type(name)?;
        body.push(toks!["return ", name, "(", args, ")"]);

        let mut decode = Tokens::new();
        decode.push("@staticmethod");
        decode.push("def decode(data):");
        decode.nested(body.join_line_spacing());

        Ok(decode)
    }

    fn ident(&self, name: &str) -> String {
        if let Some(ref id_converter) = self.id_converter {
            id_converter.convert(name)
        } else {
            name.to_owned()
        }
    }

    fn field_ident(&self, field: &RpField) -> String {
        self.ident(field.ident())
    }

    fn build_constructor<'a, 'el, I>(&self, fields: I) -> Tokens<'el, Python<'el>>
    where
        I: IntoIterator<Item = &'a Loc<PythonField<'a>>>,
    {
        let mut args = Tokens::new();
        let mut assign = Tokens::new();

        args.append("self");

        for field in fields {
            args.append(field.ident.clone());

            assign.push(toks![
                "self.",
                field.ident.clone(),
                " = ",
                field.ident.clone(),
            ]);
        }

        let mut constructor = Tokens::new();
        constructor.push(toks!["def __init__(", args.join(", "), "):"]);
        constructor.nested(assign);
        constructor
    }

    fn build_getters<'a, 'el, I>(&self, fields: I) -> Result<Vec<Tokens<'el, Python<'el>>>>
    where
        I: IntoIterator<Item = &'a Loc<PythonField<'a>>>,
    {
        let mut result = Vec::new();

        for field in fields {
            let name = Rc::new(self.to_lower_snake.convert(field.ident.as_str()));
            let mut body = Tokens::new();
            body.push(toks!("def get_", name, "(self):"));
            body.nested(toks!["return self.", field.ident.clone()]);
            result.push(body);
        }

        Ok(result)
    }

    fn convert_type_id<'el, F>(
        &self,
        name: &'el RpName,
        path_syntax: F,
    ) -> Result<Tokens<'el, Python<'el>>>
    where
        F: Fn(Vec<&str>) -> String,
    {
        let registered = self.env.lookup(name)?;

        let local_name = registered.local_name(name, |p| p.join(TYPE_SEP), path_syntax);

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

    pub fn enum_variants<'el>(&self, body: &'el RpEnumBody) -> Result<Tokens<'el, Python<'el>>> {
        let mut args = Tokens::new();

        let variants = body.variants.iter().map(|l| l.loc_ref());

        variants.for_each_loc(|variant| {
            let var_name = variant.local_name.as_str().quoted();

            let mut enum_arguments = Tokens::new();

            enum_arguments.append(var_name);
            enum_arguments.append(self.ordinal(variant)?);

            args.append(toks!["(", enum_arguments.join(", "), ")"]);

            Ok(()) as Result<()>
        })?;

        let class_name = body.local_name.as_str().quoted();

        Ok(toks![
            body.local_name.as_str(),
            " = ",
            self.enum_enum.clone(),
            "(",
            class_name,
            ", [",
            args.join(", "),
            "], type=",
            body.local_name.as_str(),
            ")",
        ])
    }

    fn enum_ident(field: PythonField) -> PythonField {
        match field.ident.as_str() {
            "name" => field.with_ident("_name".to_owned()),
            "value" => field.with_ident("_value".to_owned()),
            "ordinal" => field.with_ident("_ordinal".to_owned()),
            _ => field,
        }
    }

    fn into_python_field_with<'el, F>(
        &self,
        field: &'el RpField,
        python_field_f: F,
    ) -> PythonField<'el>
    where
        F: Fn(PythonField) -> PythonField,
    {
        let ident = self.field_ident(field);

        python_field_f(PythonField {
            modifier: &field.modifier,
            ty: &field.ty,
            name: field.name(),
            ident: Rc::new(ident),
        })
    }

    fn into_python_field<'el>(&self, field: &'el RpField) -> PythonField<'el> {
        self.into_python_field_with(field, |ident| ident)
    }

    fn as_class<'el>(
        &self,
        name: Rc<String>,
        body: Tokens<'el, Python<'el>>,
    ) -> Tokens<'el, Python<'el>> {
        let mut class = Tokens::new();
        class.push(toks!("class ", name, ":"));
        class.nested(body.join_line_spacing());
        class
    }

    pub fn process_tuple<'el>(
        &self,
        out: &mut PythonFileSpec<'el>,
        body: &'el RpTupleBody,
    ) -> Result<()> {
        let mut tuple_body = Tokens::new();
        let type_name = Rc::new(body.name.join(TYPE_SEP));

        let fields: Vec<Loc<PythonField>> = body.fields
            .iter()
            .map(|f| f.as_ref().map(|f| self.into_python_field(f)))
            .collect();

        tuple_body.push(self.build_constructor(&fields));

        // TODO: make configurable
        if false {
            for getter in self.build_getters(&fields)? {
                tuple_body.push(getter);
            }
        }

        for code in body.codes.for_context(PYTHON_CONTEXT) {
            for line in &code.lines {
                tuple_body.push(line.as_str());
            }
        }

        let decode = self.decode_method(
            &body.name,
            &fields,
            |i, _| i.to_string().into(),
        )?;
        tuple_body.push(decode);

        let encode = self.encode_tuple_method(&fields)?;
        tuple_body.push(encode);

        let repr_method = self.repr_method(type_name.clone(), &fields);
        tuple_body.push(repr_method);

        let class = self.as_class(type_name, tuple_body);

        out.0.push(class);
        Ok(())
    }

    /// Process an enum for Python.
    pub fn process_enum<'el>(
        &self,
        out: &mut PythonFileSpec<'el>,
        body: &'el Loc<RpEnumBody>,
    ) -> Result<()> {
        let type_name = Rc::new(body.name.join(TYPE_SEP));
        let mut class_body = Tokens::new();
        let variant_field = body.variant_type.as_field();

        let field = Loc::new(
            self.into_python_field_with(&variant_field, Self::enum_ident),
            body.pos().clone(),
        );

        class_body.push(self.build_constructor(iter::once(&field)));

        // TODO: make configurable
        if false {
            for getter in self.build_getters(iter::once(&field))? {
                class_body.push(getter);
            }
        }

        for code in body.codes.for_context(PYTHON_CONTEXT) {
            for line in &code.lines {
                class_body.push(line.as_str());
            }
        }

        class_body.push(self.encode_enum_method(&field)?);
        class_body.push(self.decode_enum_method(&field)?);

        let repr_method = self.repr_method(type_name.clone(), iter::once(&field));
        class_body.push(repr_method);

        let class = self.as_class(type_name, class_body);
        out.0.push(class);
        Ok(())
    }

    pub fn process_type<'el>(
        &self,
        out: &mut PythonFileSpec<'el>,
        body: &'el RpTypeBody,
    ) -> Result<()> {
        let type_name = Rc::new(body.name.join(TYPE_SEP));
        let mut class_body = Tokens::new();

        let fields: Vec<Loc<PythonField>> = body.fields
            .iter()
            .map(|f| f.as_ref().map(|f| self.into_python_field(f)))
            .collect();

        let constructor = self.build_constructor(&fields);
        class_body.push(constructor);

        // TODO: make configurable
        if false {
            for getter in self.build_getters(&fields)? {
                class_body.push(getter);
            }
        }

        let decode = self.decode_method(
            &body.name,
            &fields,
            |_, field| toks!(field.name.quoted()),
        )?;

        class_body.push(decode);

        let encode = self.encode_method(&fields, self.dict.clone().into(), None)?;

        class_body.push(encode);

        let repr_method = self.repr_method(type_name.clone(), &fields);
        class_body.push(repr_method);

        for code in body.codes.for_context(PYTHON_CONTEXT) {
            for line in &code.lines {
                class_body.push(line.as_str());
            }
        }

        out.0.push(self.as_class(type_name, class_body));
        Ok(())
    }

    pub fn process_interface<'el>(
        &self,
        out: &mut PythonFileSpec<'el>,
        body: &'el RpInterfaceBody,
    ) -> Result<()> {
        let type_name = Rc::new(body.name.join(TYPE_SEP));
        let mut type_body = Tokens::new();

        type_body.push(self.interface_decode_method(&body)?);

        for code in body.codes.for_context(PYTHON_CONTEXT) {
            for line in &code.lines {
                type_body.push(line.as_str());
            }
        }

        out.0.push(self.as_class(type_name, type_body));

        let values = body.sub_types.values().map(|l| l.loc_ref());

        values.for_each_loc(|sub_type| {
            let sub_type_name = Rc::new(sub_type.name.join(TYPE_SEP));

            let mut sub_type_body = Tokens::new();

            sub_type_body.push(toks!["TYPE = ", sub_type.name().quoted()]);

            let fields: Vec<Loc<PythonField>> = body.fields
                .iter()
                .chain(sub_type.fields.iter())
                .map(|f| f.as_ref().map(|f| self.into_python_field(f)))
                .collect();

            let constructor = self.build_constructor(&fields);
            sub_type_body.push(constructor);

            // TODO: make configurable
            if false {
                for getter in self.build_getters(&fields)? {
                    sub_type_body.push(getter);
                }
            }

            let decode = self.decode_method(&body.name, &fields, |_, field| {
                toks!(field.ident.clone().quoted())
            })?;

            sub_type_body.push(decode);

            let type_var = self.type_var.clone();

            let encode = self.encode_method(
                &fields,
                self.dict.clone().into(),
                Some(toks![
                    "data[",
                    type_var,
                    "] = ",
                    sub_type.name().quoted(),
                ]),
            )?;

            sub_type_body.push(encode);

            let repr_method = self.repr_method(sub_type_name.clone(), &fields);
            sub_type_body.push(repr_method);

            for code in sub_type.codes.for_context(PYTHON_CONTEXT) {
                for line in &code.lines {
                    sub_type_body.push(line.as_str());
                }
            }

            out.0.push(self.as_class(sub_type_name, sub_type_body));
            Ok(()) as Result<()>
        })?;

        Ok(())
    }
}

impl PackageUtils for PythonBackend {}

impl<'el> Converter<'el> for PythonBackend {
    type Custom = Python<'el>;

    fn convert_type(&self, name: &'el RpName) -> Result<Tokens<'el, Self::Custom>> {
        self.convert_type_id(name, |v| v.join(TYPE_SEP))
    }

    fn convert_constant(&self, name: &'el RpName) -> Result<Tokens<'el, Self::Custom>> {
        // TODO: only last part in a constant lookup should be separated with dots.
        self.convert_type_id(name, |mut v| {
            let at = v.len().saturating_sub(2);
            let last = v.split_off(at).join(".");

            let mut parts = v.clone();
            parts.push(last.as_str());
            parts.join(TYPE_SEP)
        })
    }
}

/// Build values in python.
impl<'el> ValueBuilder<'el> for PythonBackend {}

impl<'el> DynamicConverter<'el> for PythonBackend {
    fn is_native(&self, ty: &RpType) -> bool {
        match *ty {
            RpType::Signed { size: _ } |
            RpType::Unsigned { size: _ } => true,
            RpType::Float | RpType::Double => true,
            RpType::String => true,
            RpType::Any => true,
            RpType::Boolean => true,
            RpType::Array { ref inner } => self.is_native(inner),
            RpType::Map { ref key, ref value } => self.is_native(key) && self.is_native(value),
            _ => false,
        }
    }

    fn map_key_var(&self) -> Tokens<'el, Self::Custom> {
        toks!["k"]
    }

    fn map_value_var(&self) -> Tokens<'el, Self::Custom> {
        toks!["v"]
    }

    fn array_inner_var(&self) -> Tokens<'el, Self::Custom> {
        toks!["v"]
    }
}

impl<'el> DynamicDecode<'el> for PythonBackend {
    fn name_decode(
        &self,
        input: Tokens<'el, Self::Custom>,
        name: Tokens<'el, Self::Custom>,
    ) -> Tokens<'el, Self::Custom> {
        toks![name, ".decode(", input, ")"]
    }

    fn array_decode(
        &self,
        input: Tokens<'el, Self::Custom>,
        inner: Tokens<'el, Self::Custom>,
    ) -> Tokens<'el, Self::Custom> {
        toks![
            "[",
            inner,
            " for ",
            self.array_inner_var(),
            " in ",
            input,
            "]",
        ]
    }

    fn map_decode(
        &self,
        input: Tokens<'el, Self::Custom>,
        key: Tokens<'el, Self::Custom>,
        value: Tokens<'el, Self::Custom>,
    ) -> Tokens<'el, Self::Custom> {
        toks![
            self.dict.clone(),
            "((",
            key,
            ", ",
            value,
            ") for (",
            self.map_key_var(),
            ", ",
            self.map_value_var(),
            ") in ",
            input,
            ".items())",
        ]
    }

    fn assign_type_var(&self, data: &'el str, type_var: &'el str) -> Tokens<'el, Self::Custom> {
        toks![type_var, " = ", data, "[", self.type_var.clone(), "]"]
    }

    fn check_type_var(
        &self,
        _data: &'el str,
        type_var: &'el str,
        name: &'el Loc<String>,
        type_name: Tokens<'el, Self::Custom>,
    ) -> Tokens<'el, Self::Custom> {
        let mut check = Tokens::new();

        check.push(toks![
            "if ",
            type_var,
            " == ",
            name.value().as_str().quoted(),
            ":",
        ]);

        check.nested(toks!["return ", type_name, ".decode(data)"]);
        check
    }

    fn raise_bad_type(&self, type_var: &'el str) -> Tokens<'el, Self::Custom> {
        toks![
            "raise Exception(",
            "bad type".quoted(),
            " + ",
            type_var,
            ")",
        ]
    }

    fn new_decode_method(
        &self,
        data: &'el str,
        body: Tokens<'el, Self::Custom>,
    ) -> Tokens<'el, Self::Custom> {
        let mut decode = Tokens::new();
        decode.push("@staticmethod");
        decode.push(toks!("def decode(", data, "):"));
        decode.nested(body);
        decode
    }
}

impl<'el> DynamicEncode<'el> for PythonBackend {
    fn name_encode(
        &self,
        input: Tokens<'el, Self::Custom>,
        _: Tokens<'el, Self::Custom>,
    ) -> Tokens<'el, Self::Custom> {
        toks![input, ".encode()"]
    }

    fn array_encode(
        &self,
        input: Tokens<'el, Self::Custom>,
        inner: Tokens<'el, Self::Custom>,
    ) -> Tokens<'el, Self::Custom> {
        toks![
            "[",
            inner,
            " for ",
            self.array_inner_var(),
            " in ",
            input,
            "]",
        ]
    }

    fn map_encode(
        &self,
        input: Tokens<'el, Self::Custom>,
        k: Tokens<'el, Self::Custom>,
        v: Tokens<'el, Self::Custom>,
    ) -> Tokens<'el, Self::Custom> {
        toks![
            self.dict.clone(), "((", k.clone(), ", ", v, ") for (", k, ", ", self.map_value_var(), ") in ", input, ".items())",
        ]
    }
}
