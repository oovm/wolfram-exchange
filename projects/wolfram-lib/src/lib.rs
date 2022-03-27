#![deny(rustdoc::missing_crate_level_docs)]
// #![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![doc = include_str!("../../../Readme.md")]
#![doc(html_logo_url = "https://avatars.githubusercontent.com/u/11549616")]
#![doc(html_favicon_url = "https://avatars.githubusercontent.com/u/11549616")]

use num::BigInt;
use std::collections::BTreeMap;

mod extensions;
mod traits;
mod utils;

pub use self::{extensions::*, utils::*};

pub trait ToWolfram {
    fn to_wolfram(&self) -> WolframValue;
    fn to_wolfram_string(&self) -> String {
        self.to_wolfram().to_string()
    }
    fn to_wolfram_bytes(&self) -> Vec<u8> {
        self.to_wolfram().to_bytes()
    }
    fn to_wolfram_solid(&self) -> Vec<u8> {
        self.to_wolfram().to_compressed()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub enum WolframValue {
    Skip,
    /// Function with name, args
    Function(Box<WolframValue>, Vec<WolframValue>),
    String(String),
    Bytes(Vec<u8>),
    Symbol(String),
    Integer8(i8),
    Integer16(i16),
    Integer32(i32),
    Integer64(i64),
    BigInteger(BigInt),
    /// Do not use `f64`, because partial order cannot be defined
    Decimal64([u8; 8]),
    BigDecimal(String),
    /// Need to optimize
    PackedArray(Vec<WolframValue>),
    /// Need to optimize
    NumericArray(Vec<WolframValue>),
    /// Record with key, rule, value
    Association(BTreeMap<WolframValue, (WolframValue, WolframValue)>),
    Rule,
    RuleDelayed,
}
