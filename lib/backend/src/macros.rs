/// Helper macro for default vector implementation.
#[macro_export]
macro_rules! listeners_vec_default {
    ($fn:ident, $event:ident) => {
        fn $fn(&self, _: &mut $event) -> Result<()> {
            Ok(())
        }
    }
}

/// Helper macro for default option implementation.
#[macro_export]
macro_rules! listeners_opt_default {
    ($fn:ident, $event:ident, $output:ty) => {
        fn $fn(&self, _: &mut $event) -> Result<Option<$output>> {
            Ok(None)
        }
    }
}

/// Helper macro to implement listeners vec loop.
#[macro_export]
macro_rules! listeners_vec {
    ($fn:ident, $event:ident) => {
        fn $fn(&self, e: &mut $event) -> Result<()> {
            for i in self {
                i.$fn(e)?;
            }

            Ok(())
        }
    }
}

/// Helper macro to implement listeners opt loop.
#[macro_export]
macro_rules! listeners_opt {
    ($fn:ident, $event:ident, $output:ty) => {
        fn $fn(&self, e: &mut $event) -> Result<Option<$output>> {
            for i in self {
                if let Some(value) = i.$fn(e)? {
                    return Ok(Some(value));
                }
            }

            Ok(None)
        }
    }
}
