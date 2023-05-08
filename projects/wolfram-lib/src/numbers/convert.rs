use super::*;
use crate::{ToWolfram, WolframFunction, WolframValue};

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

impl From<f32> for WolframValue {
    fn from(value: f32) -> Self {
        if value == f32::INFINITY {
            WolframValue::system_symbol("Infinity")
        }
        else if value == f32::NEG_INFINITY {
            WolframFunction::system("Neg", vec![WolframValue::system_symbol("Infinity")]).to_wolfram()
        }
        else if value.is_nan() {
            WolframValue::system_symbol("Null")
        }
        else {
            WolframDecimal { repr: FloatRepr::Safe(value as f64) }
        }
    }
}
