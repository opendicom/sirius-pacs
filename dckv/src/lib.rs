//! Dicom Contextualized Key Value (DCKV) is a new representation for DICOM objects.
//! This library is a framework for serializing and deserializing DICOM objects into a key-value arrays.

use std::str;

mod error;
mod kvmap;
mod parser;
mod filter;

pub use error::DCKVError;
pub use kvmap::KVMap;
pub use parser::{Deserializer, Value};
pub use filter::Filter;

pub type Result<T> = std::result::Result<T, DCKVError>;

mod vr {
    pub(crate) const AE: u16 = 0x4145;
    pub(crate) const AS: u16 = 0x4153;
    pub(crate) const AT: u16 = 0x4154;
    pub(crate) const CS: u16 = 0x4353;
    pub(crate) const DA: u16 = 0x4441;
    pub(crate) const DS: u16 = 0x4453;
    pub(crate) const DT: u16 = 0x4454;
    pub(crate) const FD: u16 = 0x4644;
    pub(crate) const FL: u16 = 0x464C;
    pub(crate) const IS: u16 = 0x4953;
    pub(crate) const LO: u16 = 0x4c4f;
    pub(crate) const LT: u16 = 0x4c54;
    pub(crate) const PN: u16 = 0x504e;
    pub(crate) const SH: u16 = 0x5348;
    pub(crate) const SL: u16 = 0x534C;
    pub(crate) const SS: u16 = 0x5353;
    pub(crate) const ST: u16 = 0x5354;
    pub(crate) const TM: u16 = 0x544d;
    pub(crate) const UI: u16 = 0x5549;
    pub(crate) const UL: u16 = 0x554C;
    pub(crate) const US: u16 = 0x5553;

    pub(crate) const OB: u16 = 0x4f42;
    pub(crate) const OD: u16 = 0x4f44;
    pub(crate) const OF: u16 = 0x4f46;
    pub(crate) const OL: u16 = 0x4f4c;
    pub(crate) const OV: u16 = 0x4f56;
    pub(crate) const OW: u16 = 0x4f57;
    pub(crate) const SV: u16 = 0x5356;
    pub(crate) const UC: u16 = 0x5543;
    pub(crate) const UR: u16 = 0x5552;
    pub(crate) const UT: u16 = 0x5554;
    pub(crate) const UV: u16 = 0x5556;
    pub(crate) const UN: u16 = 0x554E;

    pub(crate) const SQ: u16 = 0x5351;
}

// Supports max. 12 sequence nested levels.
type KeyBlocks = [u64; 12];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Key {
    bytes: Vec<u8>,
}

impl Key {
    #[inline]
    fn from_key_blocks(key_blocks: &mut [u64; 12]) -> Self {
        let bytes = key_blocks
            .iter()
            .filter(|&&t| t != 0)
            .flat_map(|&t| t.to_be_bytes())
            .collect::<Vec<u8>>();

        Self { bytes }
    }

    #[inline]
    pub fn level(&self) -> usize {
        (self.bytes.len() / 8).saturating_sub(1)
    }

    #[inline]
    pub fn group(&self) -> u16 {
        u16::from_be_bytes([
            self.bytes[self.level() * 8],
            self.bytes[self.level() * 8 + 1],
        ])
    }

    #[inline]
    pub fn element(&self) -> u16 {
        u16::from_be_bytes([
            self.bytes[self.level() * 8 + 2],
            self.bytes[self.level() * 8 + 3],
        ])
    }

    #[inline]
    pub fn vr(&self) -> Result<&str> {
        let vr_text = str::from_utf8(&self.bytes[self.level() * 8 + 4..self.level() * 8 + 6])?;
        Ok(vr_text)
    }

    #[inline]
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    #[inline]
    pub fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }
}