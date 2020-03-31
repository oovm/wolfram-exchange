use super::*;
use serde::ser::{SerializeSeq, SerializeTuple, SerializeTupleStruct, SerializeTupleVariant};

pub struct SequenceBuffer<'s> {
    ptr: &'s mut WXFSerializer,
    name: Option<&'static str>,
    buffer: Vec<WolframValue>,
}

impl<'s> SequenceBuffer<'s> {
    pub fn new(ptr: &'s mut WXFSerializer, name: Option<&'static str>, size: usize) -> Self {
        Self { name, ptr, buffer: Vec::with_capacity(size) }
    }
    fn push_sequence<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut *self.ptr)?;
        self.buffer.push(self.ptr.this.to_wolfram());
        Ok(())
    }
}

impl<'a> SerializeSeq for SequenceBuffer<'a> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.push_sequence(value)
    }

    fn end(mut self) -> Result<()> {
        Ok(self.ptr.this = self.buffer.to_wolfram())
    }
}

// Same thing but for tuples.
impl<'a> SerializeTuple for SequenceBuffer<'a> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.push_sequence(value)
    }

    fn end(mut self) -> Result<()> {
        Ok(self.ptr.this = self.buffer.to_wolfram())
    }
}

// Same thing but for tuple structs.
impl<'a> SerializeTupleStruct for SequenceBuffer<'a> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.push_sequence(value)
    }

    fn end(mut self) -> Result<()> {
        Ok(self.ptr.this = WolframValue::function(self.name.unwrap(), self.buffer))
    }
}

impl<'a> SerializeTupleVariant for SequenceBuffer<'a> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.push_sequence(value)
    }

    fn end(mut self) -> Result<()> {
        Ok(self.ptr.this = WolframValue::function(self.name.unwrap(), self.buffer))
    }
}
