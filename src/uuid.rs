//! Uuid related structs

use serde::{Serialize, Serializer};

#[derive(Debug)]
pub struct Uuid(pub u128);

impl Serialize for Uuid {
    fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let b: [u8; 16] = self.0.to_be_bytes();
        let s: &[u8] = &b;
        ser.serialize_bytes(s)
    }
}

impl From<u128> for Uuid {
    fn from(u: u128) -> Self {
        Self(u)
    }
}
