//! Translates one IR in-place into another.

use errors::Result;
use linked_hash_map::LinkedHashMap;
use std::cell::RefCell;
use std::cmp;
use std::collections::HashMap;
use std::hash;
use std::rc::Rc;
use Flavor;
use {
    CoreFlavor, Diagnostics, Loc, RpEndpoint, RpEnumType, RpField, RpName, RpReg, RpType,
    RpVersionedPackage,
};

/// Method for translating package.
pub trait PackageTranslator<K, V> {
    /// Translate the given package.
    fn translate_package(&self, source: K) -> Result<V>;
}

pub trait FlavorTranslator {
    type Source: 'static + Clone + Flavor;
    type Target: 'static + Clone + Flavor;

    fn translate_i32(&self) -> Result<<Self::Target as Flavor>::Type>;

    fn translate_i64(&self) -> Result<<Self::Target as Flavor>::Type>;

    fn translate_u32(&self) -> Result<<Self::Target as Flavor>::Type>;

    fn translate_u64(&self) -> Result<<Self::Target as Flavor>::Type>;

    fn translate_float(&self) -> Result<<Self::Target as Flavor>::Type>;

    fn translate_double(&self) -> Result<<Self::Target as Flavor>::Type>;

    fn translate_boolean(&self) -> Result<<Self::Target as Flavor>::Type>;

    fn translate_string(&self) -> Result<<Self::Target as Flavor>::Type>;

    fn translate_datetime(&self) -> Result<<Self::Target as Flavor>::Type>;

    fn translate_array(
        &self,
        _: <Self::Target as Flavor>::Type,
    ) -> Result<<Self::Target as Flavor>::Type>;

    fn translate_map(
        &self,
        _: <Self::Target as Flavor>::Type,
        _: <Self::Target as Flavor>::Type,
    ) -> Result<<Self::Target as Flavor>::Type>;

    fn translate_any(&self) -> Result<<Self::Target as Flavor>::Type>;

    fn translate_bytes(&self) -> Result<<Self::Target as Flavor>::Type>;

    /// Translate the given package.
    fn translate_package(
        &self,
        package: <Self::Source as Flavor>::Package,
    ) -> Result<<Self::Target as Flavor>::Package>;

    /// Translate the given name.
    fn translate_name(
        &self,
        reg: RpReg,
        name: Loc<RpName<Self::Target>>,
    ) -> Result<<Self::Target as Flavor>::Type>;

    /// Translate the given field.
    fn translate_field<T>(
        &self,
        translator: &T,
        diag: &mut Diagnostics,
        field: <Self::Source as Flavor>::Field,
    ) -> Result<<Self::Target as Flavor>::Field>
    where
        T: Translator<Source = Self::Source, Target = Self::Target>;

    /// Translate the given endpoint.
    fn translate_endpoint<T>(
        &self,
        translator: &T,
        diag: &mut Diagnostics,
        endpoint: <Self::Source as Flavor>::Endpoint,
    ) -> Result<<Self::Target as Flavor>::Endpoint>
    where
        T: Translator<Source = Self::Source, Target = Self::Target>;

    /// Translate a local declaration name.
    fn translate_local_name<T>(
        &self,
        translator: &T,
        diag: &mut Diagnostics,
        reg: RpReg,
        name: <Self::Source as Flavor>::Name,
    ) -> Result<<Self::Target as Flavor>::Name>
    where
        T: Translator<Source = Self::Source, Target = Self::Target>;

    /// Enum type to translate.
    fn translate_enum_type<T>(
        &self,
        translator: &T,
        diag: &mut Diagnostics,
        enum_type: <Self::Source as Flavor>::EnumType,
    ) -> Result<<Self::Target as Flavor>::EnumType>
    where
        T: Translator<Source = Self::Source, Target = Self::Target>;
}

impl PackageTranslator<RpVersionedPackage, RpVersionedPackage> for () {
    fn translate_package(&self, package: RpVersionedPackage) -> Result<RpVersionedPackage> {
        Ok(package)
    }
}

pub struct CoreFlavorTranslator<P, F> {
    package_translator: P,
    flavor: ::std::marker::PhantomData<F>,
}

impl<P, F> CoreFlavorTranslator<P, F> {
    pub fn new(package_translator: P) -> Self {
        Self {
            package_translator,
            flavor: ::std::marker::PhantomData,
        }
    }
}

impl<P: 'static, F: 'static> FlavorTranslator for CoreFlavorTranslator<P, F>
where
    P: PackageTranslator<RpVersionedPackage, F::Package>,
    F: Flavor<
        Type = RpType<F>,
        Field = RpField<F>,
        Endpoint = RpEndpoint<F>,
        Name = Loc<RpName<F>>,
        EnumType = RpEnumType,
    >,
{
    type Source = CoreFlavor;
    type Target = F;

    translator_defaults!(Self, rp_type, local_name, field, endpoint, enum_type);

    fn translate_package(
        &self,
        package: <Self::Source as Flavor>::Package,
    ) -> Result<<F as Flavor>::Package> {
        self.package_translator.translate_package(package)
    }
}

/// Translator trait from one flavor to another.
pub trait Translator {
    type Source: 'static + Flavor;
    type Target: 'static + Flavor;

    /// Indicate that the given name has been visited.
    fn visit(&self, _: &mut Diagnostics, _: &<Self::Source as Flavor>::Name) -> Result<()> {
        Ok(())
    }

    /// Translate the given package from one flavor to another.
    fn translate_package(
        &self,
        <Self::Source as Flavor>::Package,
    ) -> Result<<Self::Target as Flavor>::Package>;

    /// Translate the given type from one flavor to another.
    fn translate_type(
        &self,
        diag: &mut Diagnostics,
        <Self::Source as Flavor>::Type,
    ) -> Result<<Self::Target as Flavor>::Type>;

    /// Translate the given field from one flavor to another.
    fn translate_field(
        &self,
        diag: &mut Diagnostics,
        <Self::Source as Flavor>::Field,
    ) -> Result<<Self::Target as Flavor>::Field>;

    /// Translate the given endpoint from one flavor to another.
    fn translate_endpoint(
        &self,
        diag: &mut Diagnostics,
        <Self::Source as Flavor>::Endpoint,
    ) -> Result<<Self::Target as Flavor>::Endpoint>;

    /// Translate a local declaration name.
    fn translate_local_name(
        &self,
        diag: &mut Diagnostics,
        reg: RpReg,
        name: <Self::Source as Flavor>::Name,
    ) -> Result<<Self::Target as Flavor>::Name>;

    /// Enum type to translate.
    fn translate_enum_type(
        &self,
        diag: &mut Diagnostics,
        enum_type: <Self::Source as Flavor>::EnumType,
    ) -> Result<<Self::Target as Flavor>::EnumType>;
}

/// A translated type.
pub trait Translate<T>
where
    T: Translator,
{
    type Out;

    /// Translate into different flavor.
    fn translate(self, diag: &mut Diagnostics, translator: &T) -> Result<Self::Out>;
}

impl<T, V> Translate<T> for Loc<V>
where
    V: Translate<T>,
    T: Translator,
{
    type Out = Loc<V::Out>;

    /// Translate into different flavor.
    fn translate(self, diag: &mut Diagnostics, translator: &T) -> Result<Loc<V::Out>> {
        Loc::and_then(self, |s| s.translate(diag, translator))
    }
}

impl<T, K, V, S> Translate<T> for HashMap<K, V, S>
where
    K: cmp::Eq + hash::Hash,
    V: Translate<T>,
    T: Translator,
    S: hash::BuildHasher,
{
    type Out = HashMap<K, V::Out>;

    /// Translate into different flavor.
    fn translate(self, diag: &mut Diagnostics, translator: &T) -> Result<HashMap<K, V::Out>> {
        let mut out = HashMap::new();

        for (k, v) in self {
            let v = v.translate(diag, translator)?;
            out.insert(k, v);
        }

        Ok(out)
    }
}

impl<T, V> Translate<T> for Vec<V>
where
    V: Translate<T>,
    T: Translator,
{
    type Out = Vec<V::Out>;

    /// Translate into different flavor.
    fn translate(self, diag: &mut Diagnostics, translator: &T) -> Result<Vec<V::Out>> {
        self.into_iter()
            .map(|v| v.translate(diag, translator))
            .collect::<Result<Vec<_>>>()
    }
}

impl<T, V> Translate<T> for Option<V>
where
    V: Translate<T>,
    T: Translator,
{
    type Out = Option<V::Out>;

    /// Translate into different flavor.
    fn translate(self, diag: &mut Diagnostics, translator: &T) -> Result<Option<V::Out>> {
        let out = match self {
            Some(inner) => Some(inner.translate(diag, translator)?),
            None => None,
        };

        Ok(out)
    }
}

impl<T> Translate<T> for String
where
    T: Translator,
{
    type Out = String;

    fn translate(self, _diag: &mut Diagnostics, _translator: &T) -> Result<String> {
        Ok(self)
    }
}

impl<T, A, B> Translate<T> for (A, B)
where
    A: Translate<T>,
    B: Translate<T>,
    T: Translator,
{
    type Out = (A::Out, B::Out);

    /// Translate into different flavor.
    fn translate(self, diag: &mut Diagnostics, translator: &T) -> Result<(A::Out, B::Out)> {
        let (a, b) = self;

        let a = a.translate(diag, translator)?;
        let b = b.translate(diag, translator)?;

        Ok((a, b))
    }
}

pub struct Fields<T>(pub Vec<Loc<T>>);

impl<T, F: 'static> Translate<T> for Fields<F::Field>
where
    F: Flavor,
    T: Translator<Source = F>,
{
    type Out = Vec<Loc<<T::Target as Flavor>::Field>>;

    /// Translate into different flavor.
    fn translate(self, diag: &mut Diagnostics, translator: &T) -> Result<Self::Out> {
        let out = self
            .0
            .into_iter()
            .map(|f| Loc::and_then(f, |f| translator.translate_field(diag, f)))
            .collect::<Result<Vec<_>>>()?;

        Ok(out)
    }
}

/// Context used when translating.
pub struct Context<T>
where
    T: FlavorTranslator<Source = CoreFlavor>,
{
    /// Type used to translate types.
    pub flavor: T,
    /// Registered declarations of the source type.
    pub types: Rc<LinkedHashMap<RpName<T::Source>, Loc<RpReg>>>,
    /// Cached and translated registered declarations.
    pub decls: Option<RefCell<LinkedHashMap<RpName<T::Source>, RpReg>>>,
}

impl<T> Context<T>
where
    T: FlavorTranslator<Source = CoreFlavor>,
{
    /// Lookup and cause the given name to be registered.
    fn lookup(&self, diag: &mut Diagnostics, key: &Loc<RpName<T::Source>>) -> Result<RpReg> {
        let (key, span) = Loc::borrow_pair(key);
        let key = key.clone().without_prefix();

        let decls = self.decls.as_ref().ok_or_else(|| "no declarations")?;
        let mut decls = decls.try_borrow_mut()?;

        if let Some(reg) = decls.get(&key) {
            return Ok(reg.clone());
        }

        let reg = match self.types.get(&key) {
            Some(reg) => Loc::borrow(reg).clone(),
            None => {
                diag.err(span, format!("`{}` does not exist", key));
                return Err(format!("no such type: {}", key).into());
            }
        };

        let reg = decls.entry(key).or_insert(reg);
        Ok(reg.clone())
    }
}

impl<T> Translator for Context<T>
where
    T: FlavorTranslator<Source = CoreFlavor>,
{
    type Source = T::Source;
    type Target = T::Target;

    /// Indicate that the given name has been visited.
    fn visit(&self, diag: &mut Diagnostics, name: &Loc<RpName<Self::Source>>) -> Result<()> {
        self.lookup(diag, name)?;
        Ok(())
    }

    fn translate_package(
        &self,
        source: <Self::Source as Flavor>::Package,
    ) -> Result<<Self::Target as Flavor>::Package> {
        self.flavor.translate_package(source)
    }

    fn translate_type(
        &self,
        diag: &mut Diagnostics,
        source: <Self::Source as Flavor>::Type,
    ) -> Result<<Self::Target as Flavor>::Type> {
        use self::RpType::*;

        let out = match source {
            String => self.flavor.translate_string()?,
            DateTime => self.flavor.translate_datetime()?,
            Bytes => self.flavor.translate_bytes()?,
            Signed { size: 32 } => self.flavor.translate_i32()?,
            Signed { size: 64 } => self.flavor.translate_i64()?,
            Unsigned { size: 32 } => self.flavor.translate_u32()?,
            Unsigned { size: 64 } => self.flavor.translate_u64()?,
            Float => self.flavor.translate_float()?,
            Double => self.flavor.translate_double()?,
            Boolean => self.flavor.translate_boolean()?,
            Array { inner } => {
                let inner = self.translate_type(diag, *inner)?;
                self.flavor.translate_array(inner)?
            }
            Name { name } => {
                let reg = self.lookup(diag, &name)?;
                let name = name.translate(diag, self)?;
                self.flavor.translate_name(reg, name)?
            }
            Map { key, value } => {
                let key = self.translate_type(diag, *key)?;
                let value = self.translate_type(diag, *value)?;
                self.flavor.translate_map(key, value)?
            }
            Any => self.flavor.translate_any()?,
            ty => return Err(format!("unsupported type: {}", ty).into()),
        };

        Ok(out)
    }

    fn translate_field(
        &self,
        diag: &mut Diagnostics,
        source: <Self::Source as Flavor>::Field,
    ) -> Result<<Self::Target as Flavor>::Field> {
        self.flavor.translate_field(self, diag, source)
    }

    fn translate_endpoint(
        &self,
        diag: &mut Diagnostics,
        source: <Self::Source as Flavor>::Endpoint,
    ) -> Result<<Self::Target as Flavor>::Endpoint> {
        self.flavor.translate_endpoint(self, diag, source)
    }

    /// Translate a local declaration name.
    fn translate_local_name(
        &self,
        diag: &mut Diagnostics,
        reg: RpReg,
        name: <Self::Source as Flavor>::Name,
    ) -> Result<<Self::Target as Flavor>::Name> {
        self.flavor.translate_local_name(self, diag, reg, name)
    }

    /// Translate enum type.
    fn translate_enum_type(
        &self,
        diag: &mut Diagnostics,
        enum_type: <Self::Source as Flavor>::EnumType,
    ) -> Result<<Self::Target as Flavor>::EnumType> {
        self.flavor.translate_enum_type(self, diag, enum_type)
    }
}
