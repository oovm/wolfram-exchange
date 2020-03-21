mod encoding;
mod from_traits;
mod from_traits_extension;
mod systems;

use crate::WolframValue;

use num_bigint::BigInt;
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
}
