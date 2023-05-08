mod builders;
mod encoding;

use crate::{ToWolfram, WolframFunction, WolframValue};
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
        let v = args.into_iter().map(|x| x.to_wolfram());
        WolframFunction::system(head, v).to_wolfram()
    }
    /// Creates a [List] from sequence of elements
    pub fn list<T>(items: Vec<T>) -> WolframValue
    where
        T: ToWolfram,
    {
        let v = items.into_iter().map(|x| x.to_wolfram());
        WolframFunction::system("List", v).to_wolfram()
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
