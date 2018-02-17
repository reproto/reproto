use checksum::Checksum;
use core::errors::*;
use ring::digest;
use std::io::Read;

pub fn to_sha256<R: Read>(mut reader: R) -> Result<Checksum> {
    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 1024];

    loop {
        let len = reader.read(&mut buffer)?;

        if len == 0 {
            break;
        }

        hasher.update(&buffer[0..len]);
    }

    let checksum = hasher.finish()?;
    Ok(checksum)
}

pub struct Sha256 {
    context: digest::Context,
}

impl Sha256 {
    pub fn new() -> Sha256 {
        Sha256 {
            context: digest::Context::new(&digest::SHA256),
        }
    }

    pub fn update(&mut self, bytes: &[u8]) {
        self.context.update(bytes);
    }

    pub fn finish(self) -> Result<Checksum> {
        Ok(Checksum::new(self.context.finish().as_ref().to_vec()))
    }
}
