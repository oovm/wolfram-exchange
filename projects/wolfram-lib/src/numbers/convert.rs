use super::*;

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
            WolframValue::Decimal64(OrderedFloat::from(value as f64))
        }
    }
}

impl From<f64> for WolframValue {
    fn from(value: f64) -> Self {
        if value == f64::INFINITY {
            WolframValue::system_symbol("Infinity")
        }
        else if value == f64::NEG_INFINITY {
            WolframFunction::system("Neg", vec![WolframValue::system_symbol("Infinity")]).to_wolfram()
        }
        else if value.is_nan() {
            WolframValue::system_symbol("Null")
        }
        else {
            WolframValue::Decimal64(OrderedFloat::from(value))
        }
    }
}
