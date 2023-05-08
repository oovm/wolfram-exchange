use crate::{ToWolfram, WolframValue};
use serde_pickle::{HashableValue, Value};

impl ToWolfram for Value {
    fn to_wolfram(&self) -> WolframValue {
        match self {
            Value::None => WolframValue::system_symbol("None"),
            Value::Bool(b) => b.to_wolfram(),
            Value::I64(i) => i.to_wolfram(),
            Value::Int(i) => i.to_wolfram(),
            Value::F64(f) => f.to_wolfram(),
            Value::Bytes(b) => WolframValue::Bytes(b.clone()),
            Value::String(s) => s.to_wolfram(),
            Value::List(o) => o.to_wolfram(),
            Value::Tuple(o) => o.to_wolfram(),
            Value::Set(o) => o.to_wolfram(),
            Value::FrozenSet(o) => o.to_wolfram(),
            Value::Dict(o) => o.to_wolfram(),
        }
    }
}

impl ToWolfram for HashableValue {
    fn to_wolfram(&self) -> WolframValue {
        match self {
            HashableValue::None => WolframValue::system_symbol("None"),
            HashableValue::Bool(b) => b.to_wolfram(),
            HashableValue::I64(i) => i.to_wolfram(),
            HashableValue::Int(i) => i.to_wolfram(),
            HashableValue::F64(f) => f.to_wolfram(),
            HashableValue::Bytes(b) => WolframValue::Bytes(b.clone()),
            HashableValue::String(s) => s.to_wolfram(),
            HashableValue::Tuple(o) => o.to_wolfram(),
            HashableValue::FrozenSet(o) => o.to_wolfram(),
        }
    }
}
