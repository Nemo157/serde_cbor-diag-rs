use std::io;
use std::num::FpCategory;

use dtoa;
use hex;
use itoa;
use serde::ser::{self, Serialize};

use {Error, Result};

enum Context {
    Empty,
    NonEmpty,
}

/// A structure for serializing Rust values into CBOR diagnostic notation.
pub struct Serializer<W> {
    writer: W,
    contexts: Vec<Context>,
}

impl<'a, W> Serializer<W>
where
    W: io::Write,
{
    /// Creates a new CBOR diagnostic notation pretty print serializer.
    #[inline]
    pub fn pretty(writer: W) -> Self {
        Serializer {
            writer,
            contexts: Vec::new(),
        }
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

    fn serialize_bool(self, value: bool) -> Result<Self::Ok> {
        if value {
            self.writer.write_all(b"true")?;
        } else {
            self.writer.write_all(b"false")?;
        }
        Ok(())
    }

    fn serialize_i8(self, value: i8) -> Result<Self::Ok> {
        itoa::write(&mut self.writer, value)?;
        Ok(())
    }

    fn serialize_i16(self, value: i16) -> Result<Self::Ok> {
        itoa::write(&mut self.writer, value)?;
        Ok(())
    }

    fn serialize_i32(self, value: i32) -> Result<Self::Ok> {
        itoa::write(&mut self.writer, value)?;
        Ok(())
    }

    fn serialize_i64(self, value: i64) -> Result<Self::Ok> {
        itoa::write(&mut self.writer, value)?;
        Ok(())
    }

    serde_if_integer128! {
        fn serialize_i128(self, value: i128) -> Result<Self::Ok> {
            self.writer.write_all(value.to_string().as_ref())?;
            Ok(())
        }
    }

    fn serialize_u8(self, value: u8) -> Result<Self::Ok> {
        itoa::write(&mut self.writer, value)?;
        Ok(())
    }

    fn serialize_u16(self, value: u16) -> Result<Self::Ok> {
        itoa::write(&mut self.writer, value)?;
        Ok(())
    }

    fn serialize_u32(self, value: u32) -> Result<Self::Ok> {
        itoa::write(&mut self.writer, value)?;
        Ok(())
    }

    fn serialize_u64(self, value: u64) -> Result<Self::Ok> {
        itoa::write(&mut self.writer, value)?;
        Ok(())
    }

    serde_if_integer128! {
        fn serialize_u128(self, value: u128) -> Result<Self::Ok> {
            self.writer.write_all(value.to_string().as_ref())?;
            Ok(())
        }
    }

    fn serialize_f32(self, value: f32) -> Result<Self::Ok> {
        match value.classify() {
            FpCategory::Infinite => {
                if value.is_sign_positive() {
                    self.writer.write_all(b"Infinity")?;
                } else {
                    self.writer.write_all(b"-Infinity")?;
                }
            }
            FpCategory::Nan => {
                self.writer.write_all(b"NaN")?;
            }
            FpCategory::Zero | FpCategory::Normal | FpCategory::Subnormal => {
                dtoa::write(&mut self.writer, value)?;
            }
        }
        Ok(())
    }

    fn serialize_f64(self, value: f64) -> Result<Self::Ok> {
        match value.classify() {
            FpCategory::Infinite => {
                if value.is_sign_positive() {
                    self.writer.write_all(b"Infinity")?;
                } else {
                    self.writer.write_all(b"-Infinity")?;
                }
            }
            FpCategory::Nan => {
                self.writer.write_all(b"NaN")?;
            }
            FpCategory::Zero | FpCategory::Normal | FpCategory::Subnormal => {
                dtoa::write(&mut self.writer, value)?;
            }
        }
        Ok(())
    }

    fn serialize_char(self, value: char) -> Result<Self::Ok> {
        // A char encoded as UTF-8 takes 4 bytes at most.
        let mut buf = [0; 4];
        self.serialize_str(value.encode_utf8(&mut buf))
    }

    fn serialize_str(self, value: &str) -> Result<Self::Ok> {
        self.writer.write_all(b"\"")?;
        for char in value.chars() {
            let escaped: String = char.escape_default().collect();
            self.writer.write_all(escaped.as_ref())?;
        }
        self.writer.write_all(b"\"")?;
        Ok(())
    }

    fn serialize_bytes(self, value: &[u8]) -> Result<Self::Ok> {
        self.writer.write_all(b"h'")?;
        self.writer.write_all(hex::encode(value).as_ref())?;
        self.writer.write_all(b"'")?;
        Ok(())
    }

    fn serialize_none(self) -> Result<Self::Ok> {
        self.writer.write_all(b"null")?;
        Ok(())
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok>
    where
        T: Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok> {
        self.serialize_none()
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok> {
        self.serialize_unit()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok> {
        self.serialize_str(variant)?;
        Ok(())
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok>
    where
        T: Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok>
    where
        T: Serialize,
    {
        use serde::ser::SerializeMap;
        let mut map = self.serialize_map(Some(1))?;
        map.serialize_key(variant)?;
        map.serialize_value(value)?;
        map.end()?;
        Ok(())
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
        if len.is_some() {
            self.writer.write_all(b"[")?;
            self.contexts.push(Context::Empty);
            Ok(self)
        } else {
            self.writer.write_all(b"[_ ")?;
            self.contexts.push(Context::Empty);
            Ok(self)
        }
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        self.serialize_seq(Some(len))
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

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap> {
        if len.is_some() {
            self.writer.write_all(b"{")?;
            self.contexts.push(Context::Empty);
            Ok(self)
        } else {
            self.writer.write_all(b"{_")?;
            self.contexts.push(Context::Empty);
            Ok(self)
        }
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
    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: ser::Serialize,
    {
        let context = self.contexts.pop().expect("impossible");
        match context {
            Context::Empty => {
                value.serialize(&mut **self)?;
            }
            Context::NonEmpty => {
                self.writer.write_all(b", ")?;
                value.serialize(&mut **self)?;
            }
        }
        self.contexts.push(Context::NonEmpty);
        Ok(())
    }

    #[inline]
    fn end(self) -> Result<()> {
        self.contexts.pop().expect("impossible");
        self.writer.write_all(b"]")?;
        Ok(())
    }
}

impl<'a, W> ser::SerializeTuple for &'a mut Serializer<W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = Error;

    #[inline]
    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: ser::Serialize,
    {
        ser::SerializeSeq::serialize_element(self, value)
    }

    #[inline]
    fn end(self) -> Result<()> {
        ser::SerializeSeq::end(self)
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
    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<()>
    where
        T: ser::Serialize,
    {
        let context = self.contexts.pop().expect("impossible");
        match context {
            Context::Empty => {
                self.writer.write_all(b" ")?;
                key.serialize(&mut **self)?;
            }
            Context::NonEmpty => {
                self.writer.write_all(b", ")?;
                key.serialize(&mut **self)?;
            }
        }
        self.contexts.push(Context::NonEmpty);
        Ok(())
    }

    #[inline]
    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: ser::Serialize,
    {
        self.writer.write_all(b": ")?;
        value.serialize(&mut **self)?;
        Ok(())
    }

    #[inline]
    fn end(self) -> Result<()> {
        self.contexts.pop().expect("impossible");
        self.writer.write_all(b" }")?;
        Ok(())
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
