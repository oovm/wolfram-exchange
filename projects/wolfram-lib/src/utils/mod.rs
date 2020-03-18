mod encoding;
mod from_traits;
mod from_traits_extension;
mod systems;

use crate::{ToWolfram, WolframValue};
pub use from_traits_extension::*;

use num::BigInt;
pub use systems::SYSTEM_SYMBOLS;

impl WolframValue {
    pub fn symbol(s: impl Into<String>) -> WolframValue {
        WolframValue::Symbol(s.into())
    }
    pub fn integer(i: impl Into<BigInt>) -> WolframValue {
        WolframValue::BigInteger(i.into())
    }
    pub fn function<T: ToWolfram>(head: &str, args: Vec<T>) -> WolframValue {
        let head = WolframValue::symbol(head);
        let v = args.iter().map(|t| t.to_wolfram()).collect();
        WolframValue::Function(Box::new(head), v)
    }
    pub fn list(v: Vec<WolframValue>) -> WolframValue {
        let head = WolframValue::symbol("List");
        WolframValue::Function(Box::new(head), v)
    }
    pub fn new_numeric_array() {
        unimplemented!()
    }
    pub fn new_packed_array() {
        unimplemented!()
    }
}
