mod encoding;
mod from_traits;
mod from_traits_extension;
mod systems;

use crate::{ToWolfram, WolframValue};
pub use from_traits_extension::*;

use num::BigInt;
pub use systems::SYSTEM_SYMBOLS;

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
        WolframValue::Function(Box::new(head), args.into())
    }
    pub fn list(v: Vec<WolframValue>) -> WolframValue {
        let head = WolframValue::symbol("List");
        WolframValue::Function(Box::new(head), v)
    }
    pub fn numeric_array() {
        unimplemented!()
    }
    pub fn packed_array() {
        unimplemented!()
    }
}
