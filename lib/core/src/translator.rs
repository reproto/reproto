//! Translates one IR in-place into another.

use Flavor;
use errors::Result;
use linked_hash_map::LinkedHashMap;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use {CoreFlavor, CoreFlavor2, Loc, RpEndpoint, RpField, RpName, RpPackage, RpReg, RpType,
     RpVersionedPackage};

/// Method for translating package.
pub trait PackageTranslator {
    type Source: 'static + Clone + Flavor;
    type Target: 'static + Clone + Flavor;

    /// Translate the given package.
    fn translate_package(
        &self,
        source: <Self::Source as Flavor>::Package,
    ) -> Result<<Self::Target as Flavor>::Package>;
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
        name: RpName<Self::Target>,
        reg: RpReg,
    ) -> Result<<Self::Target as Flavor>::Type>;

    /// Translate the given field.
    fn translate_field<T>(
        &self,
        translator: &T,
        field: <Self::Source as Flavor>::Field,
    ) -> Result<<Self::Target as Flavor>::Field>
    where
        T: Translator<Source = Self::Source, Target = Self::Target>;

    /// Translate the given endpoint.
    fn translate_endpoint<T>(
        &self,
        translator: &T,
        endpoint: <Self::Source as Flavor>::Endpoint,
    ) -> Result<<Self::Target as Flavor>::Endpoint>
    where
        T: Translator<Source = Self::Source, Target = Self::Target>;
}

impl PackageTranslator for () {
    type Source = CoreFlavor;
    type Target = CoreFlavor;

    fn translate_package(
        &self,
        package: <Self::Source as Flavor>::Package,
    ) -> Result<<Self::Target as Flavor>::Package> {
        Ok(package)
    }
}

impl PackageTranslator for HashMap<RpVersionedPackage, RpPackage> {
    type Source = CoreFlavor;
    type Target = CoreFlavor2;

    fn translate_package(
        &self,
        package: <Self::Source as Flavor>::Package,
    ) -> Result<<Self::Target as Flavor>::Package> {
        let package = self.get(&package)
            .ok_or_else(|| format!("no such package: {}", package))?;

        Ok(package.clone())
    }
}

pub struct CoreFlavorTranslator<P> {
    package_translator: P,
}

impl<P> CoreFlavorTranslator<P> {
    pub fn new(package_translator: P) -> Self {
        Self { package_translator }
    }
}

impl<P: 'static, F: 'static> FlavorTranslator for CoreFlavorTranslator<P>
where
    P: PackageTranslator<Source = CoreFlavor, Target = F>,
    F: Flavor<Type = RpType<F>, Field = RpField<F>, Endpoint = RpEndpoint<F>>,
{
    type Source = CoreFlavor;
    type Target = F;

    translator_core_types!(F);
    translator_core_names!(F);

    fn translate_package(
        &self,
        package: <Self::Source as Flavor>::Package,
    ) -> Result<<F as Flavor>::Package> {
        self.package_translator.translate_package(package)
    }

    fn translate_field<T>(&self, translator: &T, field: RpField<Self::Source>) -> Result<RpField<F>>
    where
        T: Translator<Source = Self::Source, Target = F>,
    {
        field.translate(translator)
    }

    fn translate_endpoint<T>(
        &self,
        translator: &T,
        endpoint: <Self::Source as Flavor>::Endpoint,
    ) -> Result<<F as Flavor>::Endpoint>
    where
        T: Translator<Source = Self::Source, Target = F>,
    {
        endpoint.translate(translator)
    }
}

/// Translator trait from one flavor to another.
pub trait Translator {
    type Source: 'static + Flavor;
    type Target: 'static + Flavor;

    /// Indicate that the given name has been visited.
    fn visit(&self, _: &RpName<Self::Source>) -> Result<()> {
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
        <Self::Source as Flavor>::Type,
    ) -> Result<<Self::Target as Flavor>::Type>;

    /// Translate the given field from one flavor to another.
    fn translate_field(
        &self,
        <Self::Source as Flavor>::Field,
    ) -> Result<<Self::Target as Flavor>::Field>;

    /// Translate the given endpoint from one flavor to another.
    fn translate_endpoint(
        &self,
        <Self::Source as Flavor>::Endpoint,
    ) -> Result<<Self::Target as Flavor>::Endpoint>;
}

/// A translated type.
pub trait Translate<T>
where
    T: Translator<Source = Self::Source>,
{
    type Source: 'static + Flavor;
    type Out;

    /// Translate into different flavor.
    fn translate(self, translator: &T) -> Result<Self::Out>;
}

impl<T, I> Translate<T> for Loc<I>
where
    I: Translate<T>,
    T: Translator<Source = I::Source>,
{
    type Source = I::Source;
    type Out = Loc<I::Out>;

    /// Translate into different flavor.
    fn translate(self, translator: &T) -> Result<Loc<I::Out>> {
        Loc::and_then(self, |s| s.translate(translator))
    }
}

impl<T, I> Translate<T> for Vec<I>
where
    I: Translate<T>,
    T: Translator<Source = I::Source>,
{
    type Source = I::Source;
    type Out = Vec<I::Out>;

    /// Translate into different flavor.
    fn translate(self, translator: &T) -> Result<Vec<I::Out>> {
        self.into_iter()
            .map(|v| v.translate(translator))
            .collect::<Result<Vec<_>>>()
    }
}

impl<T, I> Translate<T> for Option<I>
where
    I: Translate<T>,
    T: Translator<Source = I::Source>,
{
    type Source = I::Source;
    type Out = Option<I::Out>;

    /// Translate into different flavor.
    fn translate(self, translator: &T) -> Result<Option<I::Out>> {
        let out = match self {
            Some(inner) => Some(inner.translate(translator)?),
            None => None,
        };

        Ok(out)
    }
}

pub struct Fields<F>(pub Vec<Loc<F>>);

impl<T, F: 'static> Translate<T> for Fields<F::Field>
where
    F: Flavor,
    T: Translator<Source = F>,
{
    type Source = F;
    type Out = Vec<Loc<<T::Target as Flavor>::Field>>;

    /// Translate into different flavor.
    fn translate(self, translator: &T) -> Result<Self::Out> {
        let out = self.0
            .into_iter()
            .map(|f| Loc::and_then(f, |f| translator.translate_field(f)))
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
    pub types: Rc<LinkedHashMap<RpName<T::Source>, RpReg>>,
    /// Cached and translated registered declarations.
    pub decls: Option<RefCell<LinkedHashMap<RpName<T::Source>, RpReg>>>,
}

impl<T> Context<T>
where
    T: FlavorTranslator<Source = CoreFlavor>,
{
    /// Lookup and cause the given name to be registered.
    fn lookup(&self, key: &RpName<T::Source>) -> Result<RpReg> {
        let key = key.clone().without_prefix();

        let decls = self.decls.as_ref().ok_or_else(|| "no declarations")?;
        let mut decls = decls.try_borrow_mut()?;

        match decls.get(&key) {
            Some(reg) => return Ok(reg.clone()),
            None => {}
        }

        let reg = match self.types.get(&key) {
            Some(reg) => reg.clone(),
            None => {
                return Err(format!("no such type: {}", key).into());
            }
        };

        let reg = decls.entry(key).or_insert(reg);
        return Ok(reg.clone());
    }
}

impl<T> Translator for Context<T>
where
    T: FlavorTranslator<Source = CoreFlavor>,
{
    type Source = T::Source;
    type Target = T::Target;

    /// Indicate that the given name has been visited.
    fn visit(&self, name: &RpName<Self::Source>) -> Result<()> {
        self.lookup(name)?;
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
                let inner = self.translate_type(*inner)?;
                self.flavor.translate_array(inner)?
            }
            Name { name } => {
                let reg = self.lookup(&name)?;
                let name = name.translate(self)?;
                self.flavor.translate_name(name, reg)?
            }
            Map { key, value } => {
                let key = self.translate_type(*key)?;
                let value = self.translate_type(*value)?;
                self.flavor.translate_map(key, value)?
            }
            Any => self.flavor.translate_any()?,
            ty => return Err(format!("unsupported type: {}", ty).into()),
        };

        Ok(out)
    }

    fn translate_field(
        &self,
        source: <Self::Source as Flavor>::Field,
    ) -> Result<<Self::Target as Flavor>::Field> {
        self.flavor.translate_field(self, source)
    }

    fn translate_endpoint(
        &self,
        source: <Self::Source as Flavor>::Endpoint,
    ) -> Result<<Self::Target as Flavor>::Endpoint> {
        self.flavor.translate_endpoint(self, source)
    }
}
