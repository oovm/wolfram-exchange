use super::*;
use crate::{ToWolfram, WolframValue};

impl Debug for WolframDecimal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.repr, f)
    }
}

impl Debug for FloatRepr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FloatRepr::Null => write!(f, "Null"),
            FloatRepr::PositiveInfinity => write!(f, "Infinity"),
            FloatRepr::Safe(value) => write!(f, "{:?}", value),
            FloatRepr::BigDecimal(value) => write!(f, "{:?}", value),
            FloatRepr::NegativeInfinity => write!(f, "-Infinity"),
        }
    }
}

impl ToWolfram for WolframDecimal {
    fn to_wolfram(&self) -> WolframValue {
        todo!()
    }
}
