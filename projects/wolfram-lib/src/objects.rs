use crate::WolframValue;

pub fn date_object(input:&str) ->WolframValue{
    WolframValue::new_function("DateObject", vec![input])
}