use num::BigInt;
use std::{
    collections::BTreeMap,
    fmt::{self, Display},
};

pub mod objects;
pub mod utils;

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

impl Display for WolframValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            WolframValue::Skip => write!(f, ""),
            WolframValue::Function(head, args) => {
                let v: Vec<String> = args.iter().map(|v| v.to_string()).collect();
                if head.to_string() == "List" { write!(f, "{{{}}}", v.join(",")) } else { write!(f, "{}[{}]", head.to_string(), v.join(",")) }
            }
            WolframValue::String(s) => write!(f, "{:?}", s),
            WolframValue::Bytes(b) => {
                let v: Vec<String> = b.iter().map(|s| format!("{}", s)).collect();
                write!(f, "ByteArray[{{{}}}]", v.join(","))
            }
            WolframValue::Symbol(s) => write!(f, "{}", s),
            WolframValue::Integer8(i) => write!(f, "{}", i),
            WolframValue::Integer16(i) => write!(f, "{}", i),
            WolframValue::Integer32(i) => write!(f, "{}", i),
            WolframValue::Integer64(i) => write!(f, "{}", i),
            WolframValue::BigInteger(i) => write!(f, "{}", i),
            WolframValue::Decimal64(d) => write!(f, "{}`", f64::from_le_bytes(*d)),
            WolframValue::BigDecimal(d) => write!(f, "{}", d),
            WolframValue::PackedArray(_) => unimplemented!(),
            WolframValue::NumericArray(_) => unimplemented!(),
            WolframValue::Association(dict) => {
                let v: Vec<String> = dict.iter().map(|(k, (r, v))| format!("{}{}{}", k, r, v)).collect();
                write!(f, "<|{}|>", v.join(","))
            }
            WolframValue::Rule => write!(f, "->"),
            WolframValue::RuleDelayed => write!(f, ":>"),
        }
    }
}
