use num::BigInt;
use std::{
    collections::BTreeMap,
    fmt::{self, Display},
};

pub mod utils;

pub trait ToWolfram {
    fn to_wolfram(&self) -> WolframValue;
    fn to_wolfram_string(&self) -> String {
        format!("{}", self.to_wolfram())
    }
    fn to_wolfram_bytes(&self) -> Vec<u8> {
        self.to_wolfram().to_bytes()
    }
    fn to_wolfram_solid(&self) -> Vec<u8> {
        let normal = self.to_wolfram().to_bytes();
        let solid = self.to_wolfram().to_compressed();
        if normal.len() > solid.len() { solid } else { normal }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub enum WolframValue {
    /// Function with name, args
    Function(Box<str>, Vec<WolframValue>),
    String(Box<str>),
    Bytes(Vec<u8>),
    Symbol(Box<str>),
    Integer8(i8),
    Integer16(i16),
    Integer32(i32),
    Integer64(i64),
    BigInteger(BigInt),
    /// Do not use `f64`, because partial order cannot be defined
    Decimal64([u8; 8]),
    BigDecimal(Box<str>),
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
            WolframValue::Function(name, args) => {
                let v: Vec<String> = args.iter().map(|v| v.to_string()).collect();
                if name.to_string() == "List" { write!(f, "{{{}}}", v.join(",")) } else { write!(f, "{}[{}]", name.to_string(), v.join(",")) }
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
