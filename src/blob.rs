//! Blob structs.

use serde::{Serialize, Serializer};

/// A Blob for PostgreSQL
pub struct PgBlob {
    dat: Vec<u8>,
}

impl Serialize for PgBlob {
    fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s: &[u8] = &self.dat;
        ser.serialize_bytes(s)
    }
}

impl From<Vec<u8>> for PgBlob {
    fn from(v: Vec<u8>) -> Self {
        Self { dat: v }
    }
}

impl From<&[u8]> for PgBlob {
    fn from(s: &[u8]) -> Self {
        Self::from(Vec::from(s))
    }
}

impl From<&str> for PgBlob {
    fn from(s: &str) -> Self {
        let b: &[u8] = s.as_bytes();
        Self::from(b)
    }
}
