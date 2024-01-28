use core::fmt;
use core::fmt::Display;

use std::io;

use serde::ser;

use serde::ser::{SerializeMap, SerializeSeq};
use serde::ser::{SerializeStruct, SerializeStructVariant};
use serde::ser::{SerializeTuple, SerializeTupleStruct, SerializeTupleVariant};

use serde::{Serialize, Serializer};

/// Writes a number of columns to a wtr.
///
/// ## Arguments
/// - wtr: The target writer which implements [`io::Write`].
/// - cnt: The number of columns of a row to be written.
pub fn write_col_cnt<W>(mut wtr: W, cnt: i16) -> Result<(), io::Error>
where
    W: io::Write,
{
    let bc: [u8; 2] = cnt.to_be_bytes();
    wtr.write_all(&bc)?;
    Ok(())
}

/// A number type which can be used for an item of an 1D array.
pub trait PgNum {
    /// Extends a buf by serialized bytes(network byte order)
    fn to_buf(&self, buf: &mut Vec<u8>);

    /// A type number. e.g, i16 -> 0x15, i32 -> 0x17, f32 -> 0x02bc, ...
    fn type_num() -> u32;

    /// The size of a number type. e.g, i16 -> 2, f64 -> 8, ...
    fn size() -> usize;
}

macro_rules! pgnum_create {
    ($ntyp: ty, $typ_id: literal) => {
        impl PgNum for $ntyp {
            fn size() -> usize {
                core::mem::size_of::<$ntyp>()
            }
            fn type_num() -> u32 {
                $typ_id
            }
            fn to_buf(&self, buf: &mut Vec<u8>) {
                const SZ: usize = core::mem::size_of::<$ntyp>();
                let b: [u8; SZ] = self.to_be_bytes();
                buf.extend_from_slice(&b);
            }
        }
    };
}

pgnum_create!(i16, 0x15);
pgnum_create!(i32, 0x17);
pgnum_create!(i64, 0x14);

pgnum_create!(f32, 0x02bc);
pgnum_create!(f64, 0x02bd);

/// An array of postgresql numbers
#[derive(Debug, Clone)]
pub struct PgNumArray<T>(pub Vec<T>)
where
    T: PgNum;

impl<T> From<Vec<T>> for PgNumArray<T>
where
    T: PgNum,
{
    fn from(vt: Vec<T>) -> Self {
        PgNumArray(vt)
    }
}

impl<T> Serialize for PgNumArray<T>
where
    T: PgNum,
{
    fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let esz: usize = T::size();
        let ecnt: usize = self.0.len();
        let sz: usize = match ecnt {
            0 => 12,
            _ => 4 * 5 + (4 + esz) * ecnt,
        };

        let mut buf: Vec<u8> = Vec::with_capacity(sz);

        if 0 == ecnt {
            buf.extend_from_slice(&[0, 0, 0, 0]);
            buf.extend_from_slice(&[0, 0, 0, 0]);
            let typ_num: u32 = T::type_num();
            let b: [u8; 4] = typ_num.to_be_bytes();
            buf.extend_from_slice(&b);
        } else {
            buf.extend_from_slice(&[0, 0, 0, 1]);
            buf.extend_from_slice(&[0, 0, 0, 0]);
            let typ_num: u32 = T::type_num();
            let b: [u8; 4] = typ_num.to_be_bytes();
            buf.extend_from_slice(&b);

            let isz: i32 = ecnt as i32;
            let bsz: [u8; 4] = isz.to_be_bytes();
            buf.extend_from_slice(&bsz);

            buf.extend_from_slice(&[0, 0, 0, 1]);
        }

        let items: &[T] = &self.0;
        let isz: i32 = esz as i32;
        let ib: [u8; 4] = isz.to_be_bytes();

        for item in items {
            buf.extend_from_slice(&ib);
            item.to_buf(&mut buf);
        }

        let s: &[u8] = &buf;
        ser.serialize_bytes(s)
    }
}

/// Writes a val to a wtr
pub fn to_writer<W, T>(wtr: W, val: &T) -> Result<(), Error>
where
    W: io::Write,
    T: Serialize,
{
    let mut ser = Ser { wtr };
    val.serialize(&mut ser)
}

struct Ser<W> {
    wtr: W,
}

#[derive(Debug)]
pub enum Error {
    Message(String),

    WriteError(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Self::Message(msg) => write!(f, "{msg}"),
            Self::WriteError(msg) => write!(f, "Write Error: {msg}"),
        }
    }
}

impl ser::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Self::Message(msg.to_string())
    }
}

impl std::error::Error for Error {}

impl<'a, W> SerializeSeq for &'a mut Ser<W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, val: &T) -> Result<(), Self::Error>
    where
        T: Serialize + ?Sized,
    {
        val.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a, W> SerializeTuple for &'a mut Ser<W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, _val: &T) -> Result<(), Self::Error>
    where
        T: Serialize + ?Sized,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl<'a, W> SerializeTupleStruct for &'a mut Ser<W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, _val: &T) -> Result<(), Self::Error>
    where
        T: Serialize + ?Sized,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl<'a, W> SerializeTupleVariant for &'a mut Ser<W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, _val: &T) -> Result<(), Self::Error>
    where
        T: Serialize + ?Sized,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl<'a, W> SerializeMap for &'a mut Ser<W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_key<T>(&mut self, _key: &T) -> Result<(), Self::Error>
    where
        T: Serialize + ?Sized,
    {
        todo!()
    }

    fn serialize_value<T>(&mut self, _val: &T) -> Result<(), Self::Error>
    where
        T: Serialize + ?Sized,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl<'a, W> SerializeStruct for &'a mut Ser<W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, _key: &'static str, val: &T) -> Result<(), Self::Error>
    where
        T: Serialize + ?Sized,
    {
        val.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a, W> SerializeStructVariant for &'a mut Ser<W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, _key: &'static str, _val: &T) -> Result<(), Self::Error>
    where
        T: Serialize + ?Sized,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl<W> Ser<W>
where
    W: io::Write,
{
    fn serialize_col_size(&mut self, sz: i32) -> Result<(), Error> {
        let ib: [u8; 4] = sz.to_be_bytes();
        self.wtr
            .write_all(&ib)
            .map_err(|e| Error::WriteError(format!("unable to write a size of a column: {e}")))?;
        Ok(())
    }
}

macro_rules! serialize_signed {
    ($ity: ty, $name: ident) => {
        fn $name(self, i: $ity) -> Result<Self::Ok, Self::Error> {
            const SZ: i32 = core::mem::size_of::<$ity>() as i32;
            self.serialize_col_size(SZ)?;
            let b: [u8; SZ as usize] = i.to_be_bytes();
            self.wtr
                .write_all(&b)
                .map_err(|e| Error::WriteError(format!("unable to write an integer: {e}")))?;
            Ok(())
        }
    };
}

macro_rules! serialize_float {
    ($fty: ty, $name: ident) => {
        fn $name(self, f: $fty) -> Result<Self::Ok, Self::Error> {
            const SZ: i32 = core::mem::size_of::<$fty>() as i32;
            self.serialize_col_size(SZ)?;
            let b: [u8; SZ as usize] = f.to_be_bytes();
            self.wtr
                .write_all(&b)
                .map_err(|e| Error::WriteError(format!("unable to write a float value: {e}")))?;
            Ok(())
        }
    };
}

impl<'a, W> Serializer for &'a mut Ser<W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = Error;

    type SerializeSeq = Self;
    type SerializeMap = Self;

    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        let u: u8 = match v {
            true => 1,
            false => 0,
        };
        let sz: i32 = 1;
        self.serialize_col_size(sz)?;
        self.wtr
            .write_all(&[u])
            .map_err(|e| Error::WriteError(format!("unable to write a bool value: {e}")))?;
        Ok(())
    }

    fn serialize_i8(self, i: i8) -> Result<Self::Ok, Self::Error> {
        self.serialize_i16(i.into())
    }

    serialize_signed!(i16, serialize_i16);
    serialize_signed!(i32, serialize_i32);
    serialize_signed!(i64, serialize_i64);

    serialize_float!(f32, serialize_f32);
    serialize_float!(f64, serialize_f64);

    /// postgresql does not support i128
    fn serialize_i128(self, _i: i128) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    /// serialize as an i16 value
    fn serialize_u8(self, i: u8) -> Result<Self::Ok, Self::Error> {
        self.serialize_i16(i.into())
    }

    /// serialize as an i32 value
    fn serialize_u16(self, i: u16) -> Result<Self::Ok, Self::Error> {
        self.serialize_i32(i.into())
    }

    /// serialize as an i64 value
    fn serialize_u32(self, i: u32) -> Result<Self::Ok, Self::Error> {
        self.serialize_i64(i.into())
    }

    /// postgresql does not support u64
    fn serialize_u64(self, _i: u64) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    /// postgresql does not support u128
    fn serialize_u128(self, _i: u128) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_char(self, c: char) -> Result<Self::Ok, Self::Error> {
        let mut buf: [u8; 4] = [0; 4];
        let encoded: &mut str = c.encode_utf8(&mut buf);
        let b: &[u8] = encoded.as_bytes();
        self.serialize_bytes(b)
    }
    fn serialize_str(self, s: &str) -> Result<Self::Ok, Self::Error> {
        self.serialize_bytes(s.as_bytes())
    }

    fn serialize_bytes(self, b: &[u8]) -> Result<Self::Ok, Self::Error> {
        let sz: i32 = b.len() as i32;
        self.serialize_col_size(sz)?;
        self.wtr
            .write_all(b)
            .map_err(|e| Error::WriteError(format!("unable to write bytes: {e}")))?;
        Ok(())
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        let sz: i32 = -1;
        self.serialize_col_size(sz)
    }
    fn serialize_some<T>(self, t: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize + ?Sized,
    {
        t.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        self.serialize_none()
    }
    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.serialize_unit()
    }
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _vix: u32,
        var: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(var)
    }

    fn serialize_newtype_struct<T>(
        self,
        _name: &'static str,
        val: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize + ?Sized,
    {
        val.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _vix: u32,
        _var: &'static str,
        val: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize + ?Sized,
    {
        val.serialize(self)
    }

    /// the use of native Vec is unsupported.
    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        todo!()
    }

    /// the use of native Tuple is unsupported.
    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        todo!()
    }

    /// the use of native Tuple is unsupported.
    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        todo!()
    }

    /// the use of native Tuple is unsupported.
    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _vix: u32,
        _var: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        todo!()
    }

    /// map is unsupported
    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        todo!()
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(self)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _vix: u32,
        _var: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        unimplemented!()
    }
}

#[cfg(test)]
mod test_item {
    mod pg_num_array {
        mod from {
            use crate::item::PgNumArray;

            #[test]
            fn empty4i() {
                let pna3: PgNumArray<i16> = PgNumArray::from(vec![]);
                assert_eq!(0, pna3.0.len());
            }

            #[test]
            fn single5i() {
                let pna3: PgNumArray<i32> = PgNumArray::from(vec![299792458]);
                assert_eq!(1, pna3.0.len());
                let raw: i32 = pna3.0[0];
                assert_eq!(299792458, raw);
            }

            #[test]
            fn multi6f() {
                let pna3: PgNumArray<f64> = PgNumArray::from(vec![3.776]);
                assert_eq!(1, pna3.0.len());
                let raw: f64 = pna3.0[0];
                assert_eq!(3.776, raw);
            }
        }
    }
}
