mod encoding;
mod from_traits;
mod from_traits_extension;
mod systems;

use crate::{ToWolfram, WolframValue};

use num::BigInt;
pub use systems::SYSTEM_SYMBOLS;

impl WolframValue {
    pub fn new_symbol(s: &str) -> WolframValue {
        WolframValue::Symbol(Box::from(s))
    }
    pub fn new_integer<T: Into<BigInt>>(i: T) -> WolframValue {
        WolframValue::BigInteger(i.into())
    }
    pub fn new_function<T: ToWolfram>(name: &str, args: Vec<T>) -> WolframValue {
        let f = Box::from(name);
        let v = args.iter().map(|t| t.to_wolfram()).collect();
        WolframValue::Function(f, v)
    }
    pub fn new_list(v: Vec<WolframValue>) -> WolframValue {
        WolframValue::Function(Box::from("List"), v)
    }
    pub fn new_numeric_array() {
        unimplemented!()
    }
    pub fn new_packed_array() {
        unimplemented!()
    }
}
