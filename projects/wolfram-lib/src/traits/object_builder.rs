use crate::{ToWolfram, WolframError, WolframFunction, WolframValue};
use serde::{
    ser::{
        Error, SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant, SerializeTuple, SerializeTupleStruct, SerializeTupleVariant,
    },
    Serialize, Serializer,
};
use std::fmt::Display;

/// A serializer for the Wolfram Language.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct WolframSerializer {}

impl Default for WolframSerializer {
    fn default() -> Self {
        Self {}
    }
}

impl Error for WolframError {
    fn custom<T: Display>(_msg: T) -> Self {
        todo!()
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct SerializerToAny {}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SerializeAsFunction<'i> {
    name_space: &'static str,
    name: &'static str,
    body: Vec<WolframValue>,
    config: &'i WolframSerializer,
}

impl<'i> Serializer for &'i WolframSerializer {
    type Ok = WolframValue;
    type Error = WolframError;
    type SerializeSeq = SerializerToAny;
    type SerializeTuple = SerializerToAny;
    type SerializeTupleStruct = SerializeAsFunction<'i>;
    type SerializeTupleVariant = SerializeAsFunction<'i>;
    type SerializeMap = SerializerToAny;
    type SerializeStruct = SerializeAsFunction<'i>;
    type SerializeStructVariant = SerializeAsFunction<'i>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        Ok(WolframValue::Boolean(v))
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        Ok(WolframValue::Integer8(v))
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        Ok(WolframValue::Integer16(v))
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        Ok(WolframValue::Integer32(v))
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        Ok(WolframValue::Integer64(v))
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        if v > 127 { Ok(WolframValue::Integer16(v as i16)) } else { Ok(WolframValue::Integer8(v as i8)) }
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        if v > 32767 { Ok(WolframValue::Integer32(v as i32)) } else { Ok(WolframValue::Integer16(v as i16)) }
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        if v > 2147483647 { Ok(WolframValue::Integer64(v as i64)) } else { Ok(WolframValue::Integer32(v as i32)) }
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        if v > 9223372036854775807 { Ok(WolframValue::Integer64(v as i64)) } else { Ok(WolframValue::integer(v)) }
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        Ok(WolframValue::String(v.to_string()))
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        Ok(WolframValue::String(v.to_string()))
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        todo!()
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        Ok(WolframFunction::global(name, vec![]).to_wolfram())
    }

    fn serialize_unit_variant(self, name: &'static str, _: u32, variant: &'static str) -> Result<Self::Ok, Self::Error> {
        Ok(WolframFunction::namespace(name, variant, vec![]).to_wolfram())
    }

    fn serialize_newtype_struct<T: ?Sized>(self, name: &'static str, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        todo!()
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        todo!()
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        todo!()
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        todo!()
    }

    fn serialize_tuple_struct(self, name: &'static str, len: usize) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Ok(SerializeAsFunction { name_space: "", name, body: Vec::with_capacity(len), config: self })
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        _: u32,
        variant: &'static str,
        n: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Ok(SerializeAsFunction { name_space: name, name: variant, body: Vec::with_capacity(n), config: self })
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        todo!()
    }

    fn serialize_struct(self, name: &'static str, len: usize) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(SerializeAsFunction { name_space: "", name, body: Vec::with_capacity(len), config: self })
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        _: u32,
        variant: &'static str,
        n: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Ok(SerializeAsFunction { name_space: name, name: variant, body: Vec::with_capacity(n), config: self })
    }
    fn is_human_readable(&self) -> bool {
        false
    }
    fn serialize_i128(self, v: i128) -> Result<Self::Ok, Self::Error> {
        Ok(WolframValue::integer(v))
    }
    fn serialize_u128(self, v: u128) -> Result<Self::Ok, Self::Error> {
        Ok(WolframValue::integer(v))
    }
}

impl SerializeSeq for SerializerToAny {
    type Ok = WolframValue;
    type Error = WolframError;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl SerializeTuple for SerializerToAny {
    type Ok = WolframValue;
    type Error = WolframError;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}
impl SerializeMap for SerializerToAny {
    type Ok = WolframValue;
    type Error = WolframError;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        todo!()
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl<'i> SerializeTupleStruct for SerializeAsFunction<'i> {
    type Ok = WolframValue;
    type Error = WolframError;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        let item = value.serialize(self.config)?;
        self.body.push(item);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(WolframFunction::global(self.name, self.body).to_wolfram())
    }
}

impl<'i> SerializeTupleVariant for SerializeAsFunction<'i> {
    type Ok = WolframValue;
    type Error = WolframError;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        SerializeTupleStruct::serialize_field(self, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(WolframFunction::namespace(self.name_space, self.name, self.body).to_wolfram())
    }
}

impl<'i> SerializeStruct for SerializeAsFunction<'i> {
    type Ok = WolframValue;
    type Error = WolframError;

    fn serialize_field<T: ?Sized>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        let key = WolframValue::String(key.to_string());
        let value = value.serialize(self.config)?;
        let pair = WolframValue::pair(key, value, false);
        self.body.push(pair);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(WolframFunction::global(self.name, self.body).to_wolfram())
    }
}

impl<'i> SerializeStructVariant for SerializeAsFunction<'i> {
    type Ok = WolframValue;
    type Error = WolframError;

    fn serialize_field<T: ?Sized>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        SerializeStruct::serialize_field(self, key, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(WolframFunction::namespace(self.name_space, self.name, self.body).to_wolfram())
    }
}
