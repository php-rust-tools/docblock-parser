use std::{
    fmt::{Debug, Display, Formatter},
    ops::{Deref, DerefMut},
};

pub struct ByteString {
    bytes: Vec<u8>,
}

impl ByteString {
    pub fn new(bytes: Vec<u8>) -> Self {
        Self { bytes }
    }

    pub fn empty() -> Self {
        Self { bytes: Vec::new() }
    }
}

impl From<Vec<u8>> for ByteString {
    fn from(bytes: Vec<u8>) -> Self {
        Self::new(bytes)
    }
}

impl From<&[u8]> for ByteString {
    fn from(bytes: &[u8]) -> Self {
        Self::new(bytes.to_vec())
    }
}

impl Default for ByteString {
    fn default() -> Self {
        Self::empty()
    }
}

impl Display for ByteString {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for &b in &self.bytes {
            match b {
                0 => write!(f, "\\0")?,
                b'\n' | b'\r' | b'\t' => write!(f, "{}", b.escape_ascii())?,
                0x01..=0x19 | 0x7f..=0xff => write!(f, "\\x{:02x}", b)?,
                _ => write!(f, "{}", b as char)?,
            }
        }

        Ok(())
    }
}

impl Debug for ByteString {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"{}\"", self)
    }
}

impl Deref for ByteString {
    type Target = Vec<u8>;

    fn deref(&self) -> &Vec<u8> {
        &self.bytes
    }
}

impl DerefMut for ByteString {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.bytes
    }
}
