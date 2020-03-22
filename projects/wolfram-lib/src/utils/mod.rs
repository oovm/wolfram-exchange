mod encoding;
mod from_traits;
#[allow(unused_imports)]
mod from_traits_extension;
mod systems;

use crate::{ToWolfram, WolframValue};

use num::BigInt;
pub use systems::SYSTEM_SYMBOLS;

impl WolframValue {
    pub fn new_symbol(s: &str) -> WolframValue {
        WolframValue::Symbol(Box::from(s))
    }
    pub fn new_integer<T>(i: T) -> WolframValue
    where
        BigInt: From<T>,
    {
        WolframValue::BigInteger(BigInt::from(i))
    }
    pub fn new_list(v: Vec<WolframValue>) -> WolframValue {
        WolframValue::Function(Box::from(WolframValue::new_symbol("List")), v)
    }
    pub fn new_function<T>(name: &str, args: Vec<T>) -> WolframValue
    where
        T: ToWolfram,
    {
        let f = Box::new(Self::new_symbol(name));
        let v = args.iter().map(|t| t.to_wolfram()).collect();
        WolframValue::Function(f, v)
    }
}
