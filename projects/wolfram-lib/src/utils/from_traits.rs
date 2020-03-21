use crate::{ToWolfram, WolframValue};

impl ToWolfram for bool {
    fn to_wolfram(&self) -> WolframValue {
        if *self { WolframValue::new_symbol("True") } else { WolframValue::new_symbol("False") }
    }
}

impl ToWolfram for String {
    fn to_wolfram(&self) -> WolframValue {
        WolframValue::String(self.clone())
    }
}

impl ToWolfram for &str {
    fn to_wolfram(&self) -> WolframValue {
        WolframValue::String(self.to_string())
    }
}

impl ToWolfram for i8 {
    fn to_wolfram(&self) -> WolframValue {
        WolframValue::Integer8(*self)
    }
}

impl ToWolfram for i16 {
    fn to_wolfram(&self) -> WolframValue {
        WolframValue::Integer16(*self)
    }
}

impl ToWolfram for i32 {
    fn to_wolfram(&self) -> WolframValue {
        WolframValue::Integer32(*self)
    }
}

impl ToWolfram for i64 {
    fn to_wolfram(&self) -> WolframValue {
        WolframValue::Integer64(*self)
    }
}

impl ToWolfram for i128 {
    fn to_wolfram(&self) -> WolframValue {
        WolframValue::BigInteger(format!("{}", self))
    }
}

impl ToWolfram for u8 {
    fn to_wolfram(&self) -> WolframValue {
        if *self <= 127 { (*self as i8).to_wolfram() } else { (*self as i16).to_wolfram() }
    }
}

impl ToWolfram for u16 {
    fn to_wolfram(&self) -> WolframValue {
        if *self <= 32767 { (*self as i16).to_wolfram() } else { (*self as i32).to_wolfram() }
    }
}

impl ToWolfram for u32 {
    fn to_wolfram(&self) -> WolframValue {
        if *self <= 2147483647 { (*self as i32).to_wolfram() } else { (*self as i64).to_wolfram() }
    }
}

impl ToWolfram for u64 {
    fn to_wolfram(&self) -> WolframValue {
        if *self <= 9223372036854775807 { (*self as i64).to_wolfram() } else { WolframValue::BigInteger(format!("{}", *self)) }
    }
}

impl ToWolfram for u128 {
    fn to_wolfram(&self) -> WolframValue {
        WolframValue::BigInteger(format!("{}", *self))
    }
}

impl ToWolfram for isize {
    fn to_wolfram(&self) -> WolframValue {
        WolframValue::BigInteger(format!("{}", *self))
    }
}

impl ToWolfram for usize {
    fn to_wolfram(&self) -> WolframValue {
        WolframValue::BigInteger(format!("{}", *self))
    }
}

impl ToWolfram for f32 {
    fn to_wolfram(&self) -> WolframValue {
        WolframValue::Decimal64(*self as f64)
    }
}

impl ToWolfram for f64 {
    fn to_wolfram(&self) -> WolframValue {
        WolframValue::Decimal64(*self)
    }
}
