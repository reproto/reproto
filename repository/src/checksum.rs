//! Represents a calculated checksum.

use errors::*;
use hex::FromHex;
use hex_slice::HexSlice;
use serde::{de, ser};
use std::fmt;
use std::ops::{Index, Range};
use std::result;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub struct Checksum {
    bytes: Vec<u8>,
}

impl Checksum {
    pub fn new(bytes: Vec<u8>) -> Checksum {
        Checksum { bytes: bytes }
    }

    pub fn from_str(input: &str) -> Result<Checksum> {
        let bytes: Vec<u8> = FromHex::from_hex(input)?;

        if bytes.len() != 32usize {
            return Err("expected 32 bytes".into());
        }

        Ok(Checksum { bytes: bytes })
    }
}

impl AsRef<[u8]> for Checksum {
    fn as_ref(&self) -> &[u8] {
        &self.bytes
    }
}

impl Index<Range<usize>> for Checksum {
    type Output = [u8];

    #[inline]
    fn index(&self, index: Range<usize>) -> &[u8] {
        Index::index(&self.bytes[..], index)
    }
}

impl fmt::Display for Checksum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", HexSlice::new(&self.bytes[..]))
    }
}

impl ser::Serialize for Checksum {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        serializer.serialize_str(&format!("{}", HexSlice::new(&self.bytes[..])))
    }
}

struct ChecksumVisitor;

impl<'de> de::Visitor<'de> for ChecksumVisitor {
    type Value = Checksum;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a hex encoded string of bytes")
    }

    fn visit_str<E>(self, value: &str) -> result::Result<Self::Value, E>
    where
        E: de::Error,
    {
        Checksum::from_str(value).map_err(de::Error::custom)
    }

    fn visit_string<E>(self, value: String) -> result::Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_str(value.as_str())
    }
}

impl<'de> de::Deserialize<'de> for Checksum {
    fn deserialize<D>(deserializer: D) -> result::Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_str(ChecksumVisitor)
    }
}
