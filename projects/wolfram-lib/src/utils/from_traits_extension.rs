use crate::{ToWolfram, WolframValue};
use serde_json::Value;

impl ToWolfram for Value {
    fn to_wolfram(&self) -> WolframValue {
        match self {
            Value::Null => WolframValue::Symbol(Box::from("None")),
            Value::Bool(o) => {
                if *o {
                    WolframValue::Symbol(Box::from("True"))
                }
                else {
                    WolframValue::Symbol(Box::from("False"))
                }
            }
            Value::Number(_) => unimplemented!(),
            Value::String(s) => WolframValue::String(s.clone()),
            Value::Array(_) => unimplemented!(),
            Value::Object(_) => unimplemented!(),
        }
    }
}
