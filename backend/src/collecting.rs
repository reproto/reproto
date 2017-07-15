use errors::*;

pub trait Collecting<'a> {
    type Processor: 'a;

    fn new() -> Self;

    fn into_bytes(self, processor: &Self::Processor) -> Result<Vec<u8>>;
}
