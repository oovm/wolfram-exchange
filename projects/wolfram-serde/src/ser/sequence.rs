use super::*;
use serde::ser::SerializeSeq;

pub struct SequenceBuffer<'s> {
    target: &'s mut WXFSerializer,
    buffer: Vec<WolframValue>,
}

impl<'s> SequenceBuffer<'s> {
    pub fn new(ptr: &'s mut WXFSerializer, size:usize) -> Self {
        Self {
            target: ptr,
            buffer: Vec::with_capacity(size)
        }
    }
}

impl<'a> SerializeSeq for SequenceBuffer<'a> {
    type Ok = ();
    type Error = Error;

    // Serialize a single element of the sequence.
    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
        where
            T: ?Sized + Serialize,
    {
        value.serialize(&mut *self.target)?;
        self.buffer.push(self.target.this.to_wolfram());
        Ok(())
    }

    // Close the sequence.
    fn end(mut self) -> Result<()> {
        self.target.this = self.buffer.to_wolfram();
        self.buffer.clear();
        Ok(())
    }
}

// Same thing but for tuples.
impl<'a> ser::SerializeTuple for SequenceBuffer<'a> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
        where
            T: ?Sized + Serialize,
    {
        value.serialize(&mut *self.target)?;
        self.buffer.push(self.target.this.to_wolfram());
        Ok(())
    }

    fn end(mut self) -> Result<()> {
        self.target.this = self.buffer.to_wolfram();
        self.buffer.clear();
        Ok(())
    }
}

// Same thing but for tuple structs.
impl<'a> ser::SerializeTupleStruct for SequenceBuffer<'a> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
        where
            T: ?Sized + Serialize,
    {
        value.serialize(&mut *self.target)?;
        self.buffer.push(self.target.this.to_wolfram());
        Ok(())
    }

    fn end(mut self) -> Result<()> {
        self.target.this = self.buffer.to_wolfram();
        self.buffer.clear();
        Ok(())
    }
}

// Tuple variants are a little different. Refer back to the
// `serialize_tuple_variant` method above:
//
//    self.output += "{";
//    variant.serialize(&mut *self)?;
//    self.output += ":[";
//
// So the `end` method in this impl is responsible for closing both the `]` and
// the `}`.
impl<'a> ser::SerializeTupleVariant for SequenceBuffer<'a> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
        where
            T: ?Sized + Serialize,
    {
        value.serialize(&mut *self.target)?;
        self.buffer.push(self.target.this.to_wolfram());
        Ok(())
    }

    fn end(mut self) -> Result<()> {
        self.target.this = self.buffer.to_wolfram();
        self.buffer.clear();
        Ok(())
    }
}