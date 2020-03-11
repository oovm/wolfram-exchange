use std::collections::BTreeMap;

mod utils;

pub trait ToWolfram {
    fn to_wolfram(&self) -> WolframValue;
    fn to_wolfram_string(&self) -> String {
        self.to_wolfram().to_string()
    }
    fn to_wolfram_bytes(&self) -> Vec<u8> {
        self.to_wolfram().to_bytes()
    }
}

pub enum WolframValue {
    Function,
    String(String),
    Bytes,
    Symbol(Box<str>),
    Integer8(i8),
    Integer16(i16),
    Integer32(i32),
    Integer64(i64),
    Decimal64(f64),
    BigInteger(String),
    BigDecimal(String),
    PackedArray,
    NumericArray(Vec<WolframValue>),
    Association,
    Rule,
    RuleDelayed,
}
