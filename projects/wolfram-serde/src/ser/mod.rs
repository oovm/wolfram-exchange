mod association;
mod sequence;

use serde::{ser, Serialize, Serializer};
use wolfram_wxf::{ToWolfram, WolframValue};

pub use self::{association::AssociationBuffer, sequence::SequenceBuffer};
use crate::{Result, WXFError as Error};
use std::collections::BTreeMap;

impl ToWolfram for WXFSerializer {
    fn to_wolfram(&self) -> WolframValue {
        self.this.to_owned()
    }
}

pub struct WXFSerializer {
    this: WolframValue,
    dict_buffer: BTreeMap<WolframValue, WolframValue>,
}

impl Default for WXFSerializer {
    fn default() -> Self {
        Self { this: WolframValue::Skip, dict_buffer: Default::default() }
    }
}

impl<'a> Serializer for &'a mut WXFSerializer {
    // The output type produced by this `Serializer` during successful
    // serialization. Most serializers that produce text or binary output should
    // set `Ok = ()` and serialize into an `io::Write` or buffer contained
    // within the `Serializer` instance, as happens here. Serializers that build
    // in-memory data structures may be simplified by using `Ok` to propagate
    // the data structure around.
    type Ok = ();

    // The error type when some error occurs during serialization.
    type Error = Error;

    // Associated types for keeping track of additional state while serializing
    // compound data structures like sequences and maps. In this case no
    // additional state is required beyond what is already stored in the
    // Serializer struct.
    type SerializeSeq = SequenceBuffer<'a>;
    type SerializeTuple = SequenceBuffer<'a>;
    type SerializeTupleStruct = SequenceBuffer<'a>;
    type SerializeTupleVariant = SequenceBuffer<'a>;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, v: bool) -> Result<()> {
        Ok(self.this = v.to_wolfram())
    }

    fn serialize_i8(self, v: i8) -> Result<()> {
        Ok(self.this = v.to_wolfram())
    }

    fn serialize_i16(self, v: i16) -> Result<()> {
        Ok(self.this = v.to_wolfram())
    }

    fn serialize_i32(self, v: i32) -> Result<()> {
        Ok(self.this = v.to_wolfram())
    }

    fn serialize_i64(self, v: i64) -> Result<()> {
        Ok(self.this = v.to_wolfram())
    }

    fn serialize_u8(self, v: u8) -> Result<()> {
        Ok(self.this = v.to_wolfram())
    }

    fn serialize_u16(self, v: u16) -> Result<()> {
        Ok(self.this = v.to_wolfram())
    }

    fn serialize_u32(self, v: u32) -> Result<()> {
        Ok(self.this = v.to_wolfram())
    }

    fn serialize_u64(self, v: u64) -> Result<()> {
        Ok(self.this = v.to_wolfram())
    }

    fn serialize_f32(self, v: f32) -> Result<()> {
        Ok(self.this = v.to_wolfram())
    }

    fn serialize_f64(self, v: f64) -> Result<()> {
        Ok(self.this = v.to_wolfram())
    }

    fn serialize_char(self, v: char) -> Result<()> {
        Ok(self.this = v.to_wolfram())
    }

    fn serialize_str(self, v: &str) -> Result<()> {
        Ok(self.this = v.to_wolfram())
    }

    // Serialize a byte array as an array of bytes. Could also use a base64
    // string here. Binary formats will typically represent byte arrays more
    // compactly.
    fn serialize_bytes(self, _v: &[u8]) -> Result<()> {
        // WolframValue::PackedArray()
        unimplemented!()
    }

    /// None
    fn serialize_none(self) -> Result<()> {
        Ok(self.this = WolframValue::symbol("None"))
    }

    fn serialize_some<T>(self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    /// In Serde, unit means an anonymous value containing no data.
    /// Nothing to output, aka Null in wolfram language
    fn serialize_unit(self) -> Result<()> {
        Ok(self.this = WolframValue::symbol("Null"))
    }

    // Unit struct means a named value containing no data. Again, since there is
    // no data, map this to JSON as `null`. There is no need to serialize the
    // name in most formats.
    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        unimplemented!()
    }

    // When serializing a unit variant (or any other kind of variant), formats
    // can choose whether to keep track of it by index or by name. Binary
    // formats typically use the index of the variant and human-readable formats
    // typically use the name.
    fn serialize_unit_variant(self, _name: &'static str, _variant_index: u32, _variant: &'static str) -> Result<()> {
        unimplemented!()
    }

    // As is done here, serializers are encouraged to treat newtype structs as
    // insignificant wrappers around the data they contain.
    fn serialize_newtype_struct<T>(self, _name: &'static str, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!()
    }

    // Note that newtype variant (and all of the other variant serialization
    // methods) refer exclusively to the "externally tagged" enum
    // representation.
    //
    // Serialize this to JSON in externally tagged form as `{ NAME: VALUE }`.
    fn serialize_newtype_variant<T>(self, _name: &'static str, _variant_index: u32, _variant: &'static str, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!()
    }

    // Now we get to the serialization of compound types.
    //
    // The start of the sequence, each value, and the end are three separate
    // method calls. This one is responsible only for serializing the start,
    // which in JSON is `[`.
    //
    // The length of the sequence may or may not be known ahead of time. This
    // doesn't make a difference in JSON because the length is not represented
    // explicitly in the serialized form. Some serializers may only be able to
    // support sequences for which the length is known up front.
    fn serialize_seq(self, length: Option<usize>) -> Result<Self::SerializeSeq> {
        Ok(SequenceBuffer::new(self, None, length.unwrap_or_default()))
    }

    // Tuples look just like sequences in JSON. Some formats may be able to
    // represent tuples more efficiently by omitting the length, since tuple
    // means that the corresponding `Deserialize implementation will know the
    // length without needing to look at the serialized data.
    fn serialize_tuple(self, length: usize) -> Result<Self::SerializeTuple> {
        Ok(SequenceBuffer::new(self, None, length))
    }

    // Tuple structs look just like sequences in JSON.
    fn serialize_tuple_struct(self, name: &'static str, length: usize) -> Result<Self::SerializeTupleStruct> {
        Ok(SequenceBuffer::new(self, Some(name), length))
    }

    // Tuple variants are represented in JSON as `{ NAME: [DATA...] }`. Again
    // this method is only responsible for the externally tagged representation.
    fn serialize_tuple_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        length: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        Ok(SequenceBuffer::new(self, Some(name), length))
    }

    // Maps are represented in JSON as `{ K: V, K: V, ... }`.
    fn serialize_map(self, _: Option<usize>) -> Result<Self::SerializeMap> {
        Ok(self)
    }

    // Structs look just like maps in JSON. In particular, JSON requires that we
    // serialize the field names of the struct. Other formats may be able to
    // omit the field names when serializing structs because the corresponding
    // Deserialize implementation is required to know what the keys are without
    // looking at the serialized data.
    fn serialize_struct(self, _name: &'static str, len: usize) -> Result<Self::SerializeStruct> {
        self.serialize_map(Some(len))
    }

    // Struct variants are represented in JSON as `{ NAME: { K: V, ... } }`.
    // This is the externally tagged representation.
    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        // self.inner += "{";
        // variant.serialize(&mut *self)?;
        // self.inner += ":{";
        // Ok(self)
        unimplemented!()
    }
}
