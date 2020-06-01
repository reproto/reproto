use crate::core::errors::*;
use crate::core::RpPackage;

pub trait IntoBytes<Processor> {
    fn into_bytes(self, processor: &Processor, package: &RpPackage) -> Result<Vec<u8>>;
}
