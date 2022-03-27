use crate::WolframValue;

/// Constructs a [DateObject](https://reference.wolfram.com/language/ref/DateObject.html) from a string.
pub fn date_object(input: &str) -> WolframValue {
    WolframValue::function("DateObject", vec![input])
}
