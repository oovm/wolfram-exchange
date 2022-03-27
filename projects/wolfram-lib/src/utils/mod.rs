mod builders;
mod encoding;

use crate::{ToWolfram, WolframValue};
pub use builders::*;

use num::BigInt;

impl WolframValue {
    /// Creates a [Symbol] value
    pub fn symbol<S>(s: S) -> WolframValue
    where
        S: Into<String>,
    {
        WolframValue::Symbol(s.into())
    }
    /// Creates a [Integer] value
    pub fn integer<I>(i: I) -> WolframValue
    where
        I: Into<BigInt>,
    {
        WolframValue::BigInteger(i.into())
    }
    /// Creates a [Function] with sequence of arguments
    pub fn function<T>(head: &str, args: Vec<T>) -> WolframValue
    where
        T: ToWolfram,
    {
        let head = WolframValue::symbol(head);
        let v = args.into_iter().map(|x| x.to_wolfram()).collect();
        WolframValue::Function(Box::new(head), v)
    }
    /// Creates a [List] from sequence of elements
    pub fn list<T>(items: Vec<T>) -> WolframValue
    where
        T: ToWolfram,
    {
        let head = WolframValue::symbol("List");
        let v = items.into_iter().map(|x| x.to_wolfram()).collect();
        WolframValue::Function(Box::new(head), v)
    }
    /// Creates a [NumericArray]
    pub fn numeric_array() {
        unimplemented!()
    }
    /// Creates a [PackedArray]
    pub fn packed_array() {
        unimplemented!()
    }
}
