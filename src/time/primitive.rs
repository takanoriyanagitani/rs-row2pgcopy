use serde::ser::Error;
use serde::{Serialize, Serializer};

use postgres_types::{ToSql, Type};

use bytes::BytesMut;

use time::{Date, OffsetDateTime, PrimitiveDateTime, Time};

/// A timestamp object for postgresql TIMESTAMP WITHOUT TIME ZONE
pub struct Timestamp(PrimitiveDateTime);

impl Serialize for Timestamp {
    fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut buf = BytesMut::with_capacity(8);
        let typ: Type = Type::TIMESTAMP;
        let st: PrimitiveDateTime = self.0;
        st.to_sql(&typ, &mut buf).map_err(|e| {
            S::Error::custom(format!(
                "unable to convert a primitive date time value: {e}"
            ))
        })?;
        ser.serialize_bytes(&buf)
    }
}

impl From<PrimitiveDateTime> for Timestamp {
    fn from(st: PrimitiveDateTime) -> Self {
        Self(st)
    }
}

impl From<OffsetDateTime> for Timestamp {
    fn from(o: OffsetDateTime) -> Self {
        let d: Date = o.date();
        let t: Time = o.time();
        let p: PrimitiveDateTime = PrimitiveDateTime::new(d, t);
        p.into()
    }
}

/// Converts from a RFC3339 &str
impl TryFrom<&str> for Timestamp {
    type Error = time::error::Parse;

    fn try_from(rfc3339: &str) -> Result<Self, Self::Error> {
        let fmt = &time::format_description::well_known::Rfc3339;
        let o: OffsetDateTime = OffsetDateTime::parse(rfc3339, fmt)?;
        Ok(o.into())
    }
}
