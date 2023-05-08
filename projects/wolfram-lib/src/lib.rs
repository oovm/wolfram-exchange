#![deny(rustdoc::missing_crate_level_docs)]
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![doc = include_str!("../Readme.md")]
#![doc(html_logo_url = "https://avatars.githubusercontent.com/u/11549616")]
#![doc(html_favicon_url = "https://avatars.githubusercontent.com/u/11549616")]

use num::BigInt;
use std::collections::BTreeMap;

mod error;
mod extensions;
mod functions;
mod numbers;
mod traits;
mod utils;

pub use crate::{
    error::{Result, WolframError},
    extensions::*,
    functions::{WolframFunction, WolframSymbol},
    numbers::WolframDecimal,
    traits::{object_builder::WolframSerializer, readable_writer::ReadableWriter},
    utils::*,
};

/// The `WolframLib` struct is the main entry point for the library.
pub trait ToWolfram {
    /// Converts the value to a [`WolframValue`] value.
    fn to_wolfram(&self) -> WolframValue;
    /// Converts the value to a [`WolframValue`] in string form.
    fn to_wolfram_string(&self) -> String {
        self.to_wolfram().to_string()
    }
    /// Converts the value to a [`WolframValue`] in bytes form.
    fn to_wolfram_bytes(&self) -> Vec<u8> {
        self.to_wolfram().to_bytes()
    }
    /// Converts the value to a [`WolframValue`] in compressed bytes form.
    fn to_wolfram_solid(&self) -> Vec<u8> {
        self.to_wolfram().to_compressed()
    }
}

/// A [`WolframValue`] is a value that can be converted to a [`WolframValue`]
#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub enum WolframValue {
    /// A Empty value, roughly equivalent to `Nothing` in Wolfram Language
    Skip,
    /// A wolfram function, notice the head of the function can be any [`WolframValue`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use wolfram_wxf::WolframValue;
    /// WolframValue::function("Plus", vec![1, 2, 3]);
    /// ```
    Function(Box<WolframFunction>),
    /// A wolfram boolean, notice that `True` and `False` are symbols in Wolfram Language
    ///
    /// ```
    /// # use wolfram_wxf::WolframValue;
    /// WolframValue::function("Plus", vec![1, 2, 3]);
    /// ```
    Boolean(bool),
    /// A wolfram string
    ///
    /// ```
    /// # use wolfram_wxf::WolframValue;
    /// WolframValue::function("Plus", vec![1, 2, 3]);
    /// ```
    String(String),
    /// A wolfram bytes
    ///
    /// ```
    /// # use wolfram_wxf::WolframValue;
    /// WolframValue::function("Plus", vec![1, 2, 3]);
    /// ```
    Bytes(Vec<u8>),
    /// A wolfram symbol
    ///
    /// ```
    /// # use wolfram_wxf::WolframValue;
    /// WolframValue::function("Plus", vec![1, 2, 3]);
    /// ```
    Symbol(Box<WolframSymbol>),
    /// A wolfram number in range `[-128, 127]`
    Integer8(i8),
    /// A wolfram number in range `[-32768, 32767]`
    Integer16(i16),
    /// A wolfram number in range `[-2147483648, 2147483647]`
    Integer32(i32),
    /// A wolfram number in range `[-9223372036854775808, 9223372036854775807]`
    Integer64(i64),
    /// A wolfram integer in arbitrary precision
    BigInteger(BigInt),
    /// Do not use `f64`, because partial order cannot be defined
    Decimal64([u8; 8]),
    /// A wolfram decimal in arbitrary precision
    BigDecimal(String),
    /// Need to optimize
    PackedArray(Vec<WolframValue>),
    /// Need to optimize
    NumericArray(Vec<WolframValue>),
    /// Record with key, rule, value
    Association(BTreeMap<WolframValue, (WolframValue, WolframValue)>),
    /// Represents the [->](https://reference.wolfram.com/language/ref/Rule.html) symbol.
    Rule,
    /// Represents the [:>](https://reference.wolfram.com/language/ref/RuleDelayed.html) symbol.
    RuleDelayed,
}
