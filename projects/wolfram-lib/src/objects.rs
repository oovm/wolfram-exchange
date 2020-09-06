use crate::WolframValue;

pub fn date_object(input: &str) -> WolframValue {
    WolframValue::function("DateObject", vec![input])
}
