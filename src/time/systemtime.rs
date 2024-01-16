use std::time::SystemTime;

use serde::ser::Error;
use serde::{Serialize, Serializer};

use postgres_types::{ToSql, Type};

use bytes::BytesMut;

pub struct Timestampz(SystemTime);

impl Serialize for Timestampz {
    fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut buf = BytesMut::with_capacity(8);
        let typ: Type = Type::TIMESTAMPTZ;
        let st: SystemTime = self.0;
        st.to_sql(&typ, &mut buf)
            .map_err(|e| S::Error::custom(format!("unable to convert a system time value: {e}")))?;
        ser.serialize_bytes(&buf)
    }
}

impl From<SystemTime> for Timestampz {
    fn from(st: SystemTime) -> Self {
        Self(st)
    }
}