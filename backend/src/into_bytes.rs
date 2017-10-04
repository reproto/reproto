use errors::*;

pub trait IntoBytes<Processor> {
    fn into_bytes(self, processor: &Processor) -> Result<Vec<u8>>;
}
