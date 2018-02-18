extern crate inflector;
extern crate linked_hash_map;
extern crate reproto_core as core;
extern crate serde;
extern crate serde_json;
extern crate serde_yaml;

mod sir;
mod json;
mod yaml;
mod format;
mod utils;

pub use self::format::Format;
pub use self::json::Json;
pub use self::yaml::Yaml;
use core::{Loc, Object, Pos, RpDecl, RpField, RpInterfaceBody, RpModifier, RpName, RpPackage,
           RpSubType, RpSubTypeStrategy, RpTupleBody, RpType, RpTypeBody, RpVersionedPackage,
           DEFAULT_TAG};
use core::errors::Result;
use inflector::cases::pascalcase::to_pascal_case;
use inflector::cases::snakecase::to_snake_case;
use linked_hash_map::LinkedHashMap;
use sir::{FieldSir, Sir, SubTypeSir};
use std::cmp;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::fmt;
use std::hash;
use std::ops;
use std::rc::Rc;

#[derive(Debug)]
pub struct Derive {
    root_name: String,
    format: Box<format::Format>,
    package_prefix: Option<RpPackage>,
}

#[derive(Debug, Clone)]
struct Context<'a> {
    path: Vec<String>,
    package_prefix: Option<&'a RpPackage>,
}

impl<'a> Context<'a> {
    /// Extract the 'local name' (last component).
    fn local_name(&self) -> Result<&str> {
        if let Some(local_name) = self.path.iter().last() {
            Ok(local_name.as_str())
        } else {
            Err(format!("No last component in name").into())
        }
    }

    /// Join this context with another path component.
    fn join(&self, name: String) -> Context<'a> {
        let mut path = self.path.iter().cloned().collect::<Vec<_>>();
        path.push(name);

        Context {
            path: path,
            package_prefix: self.package_prefix.clone(),
        }
    }

    /// Constructs an ``RpNAme`.
    fn name(&self) -> RpName {
        let package = self.package_prefix
            .cloned()
            .unwrap_or_else(RpPackage::empty);

        let package = RpVersionedPackage::new(package, None);
        RpName::new(None, package, self.path.clone())
    }
}

impl Derive {
    pub fn new(
        root_name: String,
        format: Box<format::Format>,
        package_prefix: Option<RpPackage>,
    ) -> Derive {
        Derive {
            root_name: root_name,
            format: format,
            package_prefix: package_prefix,
        }
    }
}

type TypesCache = HashMap<Sir, RpName>;

/// An opaque data structure, well all instances are equal but can contain different data.
#[derive(Debug, Clone)]
pub struct Opaque<T> {
    content: T,
}

impl<T> Opaque<T> {
    pub fn new(content: T) -> Self {
        Self { content: content }
    }
}

impl<T> cmp::PartialEq for Opaque<T> {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

impl<T> cmp::Eq for Opaque<T> {}

impl<T> hash::Hash for Opaque<T> {
    fn hash<H: hash::Hasher>(&self, _state: &mut H) {}
}

impl<T> ops::Deref for Opaque<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.content
    }
}

impl<T> ops::DerefMut for Opaque<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.content
    }
}

struct FieldInit<'a> {
    pos: &'a Pos,
    ctx: Context<'a>,
    types: &'a mut TypesCache,
}

impl<'a> FieldInit<'a> {
    fn new(pos: &'a Pos, ctx: Context<'a>, types: &'a mut TypesCache) -> FieldInit<'a> {
        FieldInit {
            pos: pos,
            ctx: ctx,
            types: types,
        }
    }

    fn init(
        self,
        original_name: String,
        sir: &FieldSir,
        decls: &mut Vec<RpDecl>,
    ) -> Result<RpField> {
        let mut comment = Vec::new();

        let name = to_snake_case(&original_name);

        let ty = match sir.field {
            Sir::Boolean => RpType::Boolean,
            Sir::Float => RpType::Float,
            Sir::Double => RpType::Double,
            Sir::I64(ref examples) => {
                format_comment(&mut comment, examples)?;
                RpType::Signed { size: 64 }
            }
            Sir::U64(ref examples) => {
                format_comment(&mut comment, examples)?;
                RpType::Unsigned { size: 64 }
            }
            Sir::String(ref examples) => {
                format_comment(&mut comment, examples)?;
                RpType::String
            }
            Sir::DateTime(ref examples) => {
                format_comment(&mut comment, examples)?;
                RpType::DateTime
            }
            Sir::Any => RpType::Any,
            Sir::Array(ref inner) => {
                let field = FieldSir {
                    optional: false,
                    field: (**inner).clone(),
                };

                let f = FieldInit::new(&self.pos, self.ctx.clone(), self.types).init(
                    name.clone(),
                    &field,
                    decls,
                )?;

                RpType::Array {
                    inner: Box::new(f.ty),
                }
            }
            ref sir => {
                let ctx = self.ctx.join(to_pascal_case(&name));

                let name = if let Some(name) = self.types.get(sir).cloned() {
                    name
                } else {
                    let name = ctx.name();

                    self.types.insert(sir.clone(), name.clone());

                    decls.push(DeclDeriver {
                        pos: &self.pos,
                        ctx: ctx.clone(),
                        types: self.types,
                    }.derive(sir)?);

                    name
                };

                RpType::Name { name: name }
            }
        };

        let field_as = if name != original_name {
            Some(original_name)
        } else {
            None
        };

        // field referencing inner declaration
        return Ok(RpField {
            modifier: if sir.optional {
                RpModifier::Optional
            } else {
                RpModifier::Required
            },
            name: name.clone(),
            comment: comment,
            ty: ty,
            field_as: field_as,
        });

        /// Format comments and attach examples.
        fn format_comment<T>(out: &mut Vec<String>, examples: &[T]) -> Result<()>
        where
            T: serde::Serialize + fmt::Debug,
        {
            out.push(format!("## Examples"));
            out.push("".to_string());

            out.push(format!("```json"));

            let mut seen = HashSet::new();

            for example in examples.iter() {
                let string = serde_json::to_string_pretty(example)
                    .map_err(|e| format!("Failed to convert to JSON: {}: {:?}", e, example))?;

                if !seen.contains(&string) {
                    seen.insert(string.clone());
                    out.push(string);
                }
            }

            out.push(format!("```"));

            Ok(())
        }
    }
}

struct DeclDeriver<'a> {
    pos: &'a Pos,
    ctx: Context<'a>,
    types: &'a mut TypesCache,
}

impl<'a> DeclDeriver<'a> {
    /// Derive a declaration from the given JSON.
    fn derive(self, sir: &Sir) -> Result<RpDecl> {
        let decl = match *sir {
            Sir::Tuple(ref array) => {
                let mut refiner = TupleRefiner {
                    pos: &self.pos,
                    ctx: self.ctx,
                    types: self.types,
                };

                let tuple = refiner.derive(array)?;
                RpDecl::Tuple(Rc::new(Loc::new(tuple, self.pos.clone())))
            }
            Sir::Object(ref object) => {
                let mut refiner = TypeRefiner {
                    pos: &self.pos,
                    ctx: self.ctx,
                    types: self.types,
                };

                let type_ = refiner.derive(object)?;
                RpDecl::Type(Rc::new(Loc::new(type_, self.pos.clone())))
            }
            Sir::Interface(ref type_field, ref sub_types) => {
                let type_ = InterfaceRefiner {
                    pos: &self.pos,
                    ctx: self.ctx,
                    types: self.types,
                }.derive(type_field, sub_types)?;

                RpDecl::Interface(Rc::new(Loc::new(type_, self.pos.clone())))
            }
            // For arrays, only generate the inner type.
            Sir::Array(ref inner) => self.derive(inner)?,
            ref value => return Err(format!("Unexpected JSON value: {:?}", value).into()),
        };

        Ok(decl)
    }
}

struct TypeRefiner<'a> {
    pos: &'a Pos,
    ctx: Context<'a>,
    types: &'a mut TypesCache,
}

impl<'a> TypeRefiner<'a> {
    /// Derive an struct body from the given input array.
    fn derive(&mut self, object: &LinkedHashMap<String, FieldSir>) -> Result<RpTypeBody> {
        let mut body = RpTypeBody {
            local_name: self.ctx.local_name()?.to_string(),
            name: self.ctx.name(),
            comment: Vec::new(),
            decls: Vec::new(),
            fields: Vec::new(),
            codes: Vec::new(),
            reserved: HashSet::new(),
        };

        self.init(&mut body, object)?;
        Ok(body)
    }

    fn init(
        &mut self,
        base: &mut RpTypeBody,
        object: &LinkedHashMap<String, FieldSir>,
    ) -> Result<()> {
        for (name, added) in object.iter() {
            let field = FieldInit::new(&self.pos, self.ctx.clone(), self.types).init(
                name.to_string(),
                added,
                &mut base.decls,
            )?;

            base.fields.push(Loc::new(field, self.pos.clone()));
        }

        Ok(())
    }
}

struct SubTypeRefiner<'a> {
    pos: &'a Pos,
    ctx: Context<'a>,
    types: &'a mut TypesCache,
}

impl<'a> SubTypeRefiner<'a> {
    /// Derive an struct body from the given input array.
    fn derive(&mut self, sub_type: &SubTypeSir) -> Result<RpSubType> {
        let mut body = RpSubType {
            local_name: self.ctx.local_name()?.to_string(),
            name: self.ctx.name(),
            comment: vec![],
            decls: vec![],
            fields: vec![],
            codes: vec![],
            sub_type_name: None,
        };

        self.init(&mut body, sub_type)?;
        Ok(body)
    }

    fn init(&mut self, base: &mut RpSubType, sub_type: &SubTypeSir) -> Result<()> {
        if sub_type.name != base.local_name {
            base.sub_type_name = Some(Loc::new(sub_type.name.to_string(), self.pos.clone()));
        }

        for (field_name, field_value) in &sub_type.structure {
            let field = FieldInit::new(&self.pos, self.ctx.clone(), self.types).init(
                field_name.to_string(),
                field_value,
                &mut base.decls,
            )?;

            base.fields.push(Loc::new(field, self.pos.clone()));
        }

        Ok(())
    }
}

struct InterfaceRefiner<'a> {
    pos: &'a Pos,
    ctx: Context<'a>,
    types: &'a mut TypesCache,
}

impl<'a> InterfaceRefiner<'a> {
    /// Derive an struct body from the given input array.
    fn derive(&mut self, tag: &str, sub_types: &[SubTypeSir]) -> Result<RpInterfaceBody> {
        let sub_type_strategy = if tag != DEFAULT_TAG {
            RpSubTypeStrategy::Tagged {
                tag: tag.to_string(),
            }
        } else {
            RpSubTypeStrategy::default()
        };

        let mut body = RpInterfaceBody {
            local_name: self.ctx.local_name()?.to_string(),
            name: self.ctx.name(),
            comment: Vec::new(),
            decls: Vec::new(),
            fields: Vec::new(),
            codes: Vec::new(),
            sub_types: BTreeMap::new(),
            sub_type_strategy: sub_type_strategy,
        };

        self.init(&mut body, sub_types)?;
        Ok(body)
    }

    fn init(&mut self, base: &mut RpInterfaceBody, sub_types: &[SubTypeSir]) -> Result<()> {
        for st in sub_types {
            let local_name = to_pascal_case(&st.name);
            let ctx = self.ctx.join(local_name.clone());

            let sub_type = SubTypeRefiner {
                pos: self.pos,
                ctx: ctx,
                types: self.types,
            }.derive(st)?;

            base.sub_types
                .insert(local_name, Rc::new(Loc::new(sub_type, self.pos.clone())));
        }

        Ok(())
    }
}

struct TupleRefiner<'a> {
    pos: &'a Pos,
    ctx: Context<'a>,
    types: &'a mut TypesCache,
}

impl<'a> TupleRefiner<'a> {
    /// Derive an tuple body from the given input array.
    fn derive(&mut self, array: &[FieldSir]) -> Result<RpTupleBody> {
        let mut body = RpTupleBody {
            local_name: self.ctx.local_name()?.to_string(),
            name: self.ctx.name(),
            comment: Vec::new(),
            decls: Vec::new(),
            fields: Vec::new(),
            codes: Vec::new(),
        };

        self.init(&mut body, array)?;
        Ok(body)
    }

    fn init(&mut self, base: &mut RpTupleBody, array: &[FieldSir]) -> Result<()> {
        for (index, added) in array.iter().enumerate() {
            let field = FieldInit::new(&self.pos, self.ctx.clone(), self.types).init(
                format!("field_{}", index),
                added,
                &mut base.decls,
            )?;

            base.fields.push(Loc::new(field, self.pos.clone()));
        }

        Ok(())
    }
}

/// Derive a declaration from the given input.
pub fn derive(derive: Derive, object: &Object) -> Result<RpDecl> {
    let Derive {
        root_name,
        format,
        package_prefix,
    } = derive;

    let sir = format.decode(object)?;

    let pos: Pos = (Rc::new(object.clone_object()), 0, 0).into();

    let mut types = HashMap::new();

    let ctx = Context {
        path: vec![root_name],
        package_prefix: package_prefix.as_ref(),
    };

    let decl = DeclDeriver {
        pos: &pos,
        ctx: ctx,
        types: &mut types,
    }.derive(&sir)?;

    Ok(decl)
}

#[cfg(test)]
mod tests {
    use super::{derive, Derive, Json};
    use core::{BytesObject, Loc, RpDecl, RpSubTypeStrategy, RpType};
    use std::sync::Arc;

    fn input(input: &str) -> RpDecl {
        let object = BytesObject::new(
            "test".to_string(),
            Arc::new(input.as_bytes().iter().cloned().collect()),
        );

        let derive_config = Derive {
            root_name: "Generator".to_string(),
            format: Box::new(Json),
            package_prefix: None,
        };

        derive(derive_config, &object).expect("bad derive")
    }

    #[test]
    fn simple_declaration() {
        let decl = input(r#"{"id": 42, "name": "Oscar"}"#);

        let ty = match decl {
            RpDecl::Type(ty) => ty,
            other => panic!("expected type, got: {:?}", other),
        };

        assert_eq!(2, ty.fields.len());
        assert_eq!("id", ty.fields[0].name.as_str());
        assert_eq!(RpType::Unsigned { size: 64 }, ty.fields[0].ty);
        assert_eq!("name", ty.fields[1].name.as_str());
        assert_eq!(RpType::String, ty.fields[1].ty);
    }

    #[test]
    fn test_interface() {
        let decl = input(
            r#"[
    {"kind": "dragon", "name": "Stephen", "age": 4812, "fire": "blue"},
    {"kind": "knight", "name": "Olivia", "armor": "Unobtanium"}
]"#,
        );

        let intf = match decl {
            RpDecl::Interface(intf) => intf,
            other => panic!("expected interface, got: {:?}", other),
        };

        assert_eq!(
            RpSubTypeStrategy::Tagged {
                tag: "kind".to_string(),
            },
            intf.sub_type_strategy
        );

        assert_eq!(2, intf.sub_types.len());
        assert_eq!(
            Some("dragon"),
            intf.sub_types["Dragon"]
                .sub_type_name
                .as_ref()
                .map(Loc::value)
                .map(String::as_str)
        );
        assert_eq!(
            Some("knight"),
            intf.sub_types["Knight"]
                .sub_type_name
                .as_ref()
                .map(Loc::value)
                .map(String::as_str)
        );
    }
}
