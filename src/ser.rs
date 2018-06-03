use std::io;

use dtoa;
use itoa;
use serde::ser::{self, Serialize};

use {Error, Result};

/// A structure for serializing Rust values into CBOR diagnostic notation.
pub struct Serializer<W> {
    writer: W,
}

impl<'a, W> Serializer<W>
where
    W: io::Write,
{
    /// Creates a new CBOR diagnostic notation pretty print serializer.
    #[inline]
    pub fn pretty(writer: W) -> Self {
        Serializer { writer }
    }

    /// Unwrap the inner [`io::Write`](::std::io::Write) from the
    /// [`Serializer`].
    #[inline]
    pub fn into_inner(self) -> W {
        self.writer
    }
}

impl<'a, W> ser::Serializer for &'a mut Serializer<W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = Error;

    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, _value: bool) -> Result<Self::Ok> {
        unimplemented!()
    }

    fn serialize_i8(self, _value: i8) -> Result<Self::Ok> {
        unimplemented!()
    }

    fn serialize_i16(self, _value: i16) -> Result<Self::Ok> {
        unimplemented!()
    }

    fn serialize_i32(self, _value: i32) -> Result<Self::Ok> {
        unimplemented!()
    }

    fn serialize_i64(self, value: i64) -> Result<Self::Ok> {
        itoa::write(&mut self.writer, value)?;
        Ok(())
    }

    serde_if_integer128! {
        fn serialize_i128(self, _value: i128) -> Result<Self::Ok> {
            unimplemented!()
        }
    }

    fn serialize_u8(self, _value: u8) -> Result<Self::Ok> {
        unimplemented!()
    }

    fn serialize_u16(self, _value: u16) -> Result<Self::Ok> {
        unimplemented!()
    }

    fn serialize_u32(self, _value: u32) -> Result<Self::Ok> {
        unimplemented!()
    }

    fn serialize_u64(self, value: u64) -> Result<Self::Ok> {
        itoa::write(&mut self.writer, value)?;
        Ok(())
    }

    serde_if_integer128! {
        fn serialize_u128(self, _value: u128) -> Result<Self::Ok> {
            unimplemented!()
        }
    }

    fn serialize_f32(self, _value: f32) -> Result<Self::Ok> {
        unimplemented!()
    }

    fn serialize_f64(self, value: f64) -> Result<Self::Ok> {
        dtoa::write(&mut self.writer, value)?;
        Ok(())
    }

    fn serialize_char(self, _value: char) -> Result<Self::Ok> {
        unimplemented!()
    }

    fn serialize_str(self, _value: &str) -> Result<Self::Ok> {
        unimplemented!()
    }

    fn serialize_bytes(self, _value: &[u8]) -> Result<Self::Ok> {
        unimplemented!()
    }

    fn serialize_none(self) -> Result<Self::Ok> {
        unimplemented!()
    }

    fn serialize_some<T: ?Sized>(self, _value: &T) -> Result<Self::Ok>
    where
        T: Serialize,
    {
        unimplemented!()
    }

    fn serialize_unit(self) -> Result<Self::Ok> {
        unimplemented!()
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok> {
        unimplemented!()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok> {
        unimplemented!()
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _name: &'static str,
        _value: &T,
    ) -> Result<Self::Ok>
    where
        T: Serialize,
    {
        unimplemented!()
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok>
    where
        T: Serialize,
    {
        unimplemented!()
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        unimplemented!()
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        unimplemented!()
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        unimplemented!()
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        unimplemented!()
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        unimplemented!()
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct> {
        unimplemented!()
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        unimplemented!()
    }
}

impl<'a, W> ser::SerializeSeq for &'a mut Serializer<W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = Error;

    #[inline]
    fn serialize_element<T: ?Sized>(&mut self, _value: &T) -> Result<()>
    where
        T: ser::Serialize,
    {
        unimplemented!()
    }

    #[inline]
    fn end(self) -> Result<()> {
        unimplemented!()
    }
}

impl<'a, W> ser::SerializeTuple for &'a mut Serializer<W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = Error;

    #[inline]
    fn serialize_element<T: ?Sized>(&mut self, _value: &T) -> Result<()>
    where
        T: ser::Serialize,
    {
        unimplemented!()
    }

    #[inline]
    fn end(self) -> Result<()> {
        unimplemented!()
    }
}

impl<'a, W> ser::SerializeTupleStruct for &'a mut Serializer<W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = Error;

    #[inline]
    fn serialize_field<T: ?Sized>(&mut self, _value: &T) -> Result<()>
    where
        T: ser::Serialize,
    {
        unimplemented!()
    }

    #[inline]
    fn end(self) -> Result<()> {
        unimplemented!()
    }
}

impl<'a, W> ser::SerializeTupleVariant for &'a mut Serializer<W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = Error;

    #[inline]
    fn serialize_field<T: ?Sized>(&mut self, _value: &T) -> Result<()>
    where
        T: ser::Serialize,
    {
        unimplemented!()
    }

    #[inline]
    fn end(self) -> Result<()> {
        unimplemented!()
    }
}

impl<'a, W> ser::SerializeMap for &'a mut Serializer<W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = Error;

    #[inline]
    fn serialize_key<T: ?Sized>(&mut self, _key: &T) -> Result<()>
    where
        T: ser::Serialize,
    {
        unimplemented!()
    }

    #[inline]
    fn serialize_value<T: ?Sized>(&mut self, _value: &T) -> Result<()>
    where
        T: ser::Serialize,
    {
        unimplemented!()
    }

    #[inline]
    fn end(self) -> Result<()> {
        unimplemented!()
    }
}

impl<'a, W> ser::SerializeStruct for &'a mut Serializer<W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = Error;

    #[inline]
    fn serialize_field<T: ?Sized>(
        &mut self,
        _key: &'static str,
        _value: &T,
    ) -> Result<()>
    where
        T: ser::Serialize,
    {
        unimplemented!()
    }

    #[inline]
    fn end(self) -> Result<()> {
        unimplemented!()
    }
}

impl<'a, W> ser::SerializeStructVariant for &'a mut Serializer<W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = Error;

    #[inline]
    fn serialize_field<T: ?Sized>(
        &mut self,
        _key: &'static str,
        _value: &T,
    ) -> Result<()>
    where
        T: ser::Serialize,
    {
        unimplemented!()
    }

    #[inline]
    fn end(self) -> Result<()> {
        unimplemented!()
    }
}

/// Serialize the given data structure as pretty-printed CBOR diagnostic
/// notation into the provided [`io::Write`](::std::io::Write).
///
/// # Errors
///
/// Serialization can fail if `T`'s implementation of
/// [`Serialize`](ser::Serialize) decides to fail.
#[inline]
pub fn to_writer_pretty<W, T: ?Sized>(writer: W, value: &T) -> Result<()>
where
    W: io::Write,
    T: ser::Serialize,
{
    let mut ser = Serializer::pretty(writer);
    value.serialize(&mut ser)?;
    Ok(())
}

/// Serialize the given data structure as a pretty-printed CBOR diagnostic
/// notation into a [`Vec<u8>`](Vec).
///
/// # Errors
///
/// Serialization can fail if `T`'s implementation of
/// [`Serialize`](ser::Serialize) decides to fail.
#[inline]
pub fn to_vec_pretty<T: ?Sized>(value: &T) -> Result<Vec<u8>>
where
    T: ser::Serialize,
{
    let mut writer = Vec::with_capacity(128);
    to_writer_pretty(&mut writer, value)?;
    Ok(writer)
}

/// Serialize the given data structure as a pretty-printed [`String`] of CBOR
/// diagnostic notation.
///
/// # Errors
///
/// Serialization can fail if `T`'s implementation of
/// [`Serialize`](ser::Serialize) decides to fail.
#[inline]
pub fn to_string_pretty<T: ?Sized>(value: &T) -> Result<String>
where
    T: ser::Serialize,
{
    let vec = to_vec_pretty(value)?;
    let string = unsafe {
        // We do not emit invalid UTF-8.
        String::from_utf8_unchecked(vec)
    };
    Ok(string)
}
