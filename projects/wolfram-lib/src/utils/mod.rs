mod builders;
mod encoding;

use crate::{ToWolfram, WolframValue};
pub use builders::*;

use num::BigInt;

impl WolframValue {
    pub fn symbol<S>(s: S) -> WolframValue
    where
        S: Into<String>,
    {
        WolframValue::Symbol(s.into())
    }
    pub fn integer<I>(i: I) -> WolframValue
    where
        I: Into<BigInt>,
    {
        WolframValue::BigInteger(i.into())
    }
    pub fn function<T>(head: &str, args: Vec<T>) -> WolframValue
    where
        T: ToWolfram,
    {
        let head = WolframValue::symbol(head);
        let v = args.into_iter().map(|x| x.to_wolfram()).collect();
        WolframValue::Function(Box::new(head), v)
    }
    pub fn list<T>(items: Vec<T>) -> WolframValue
    where
        T: ToWolfram,
    {
        let head = WolframValue::symbol("List");
        let v = items.into_iter().map(|x| x.to_wolfram()).collect();
        WolframValue::Function(Box::new(head), v)
    }
    pub fn numeric_array() {
        unimplemented!()
    }
    pub fn packed_array() {
        unimplemented!()
    }
}
