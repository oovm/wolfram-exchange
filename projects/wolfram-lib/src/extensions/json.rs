use crate::{Result, ToWolfram, WolframError, WolframValue};
use serde_json::{Number, Value};
use std::collections::BTreeMap;

impl ToWolfram for Value {
    fn to_wolfram(&self) -> WolframValue {
        match self {
            Value::Null => WolframValue::system_symbol("None"),
            Value::Bool(o) => o.to_wolfram(),
            Value::Number(n) => n.to_wolfram(),
            Value::String(s) => s.to_wolfram(),
            Value::Array(a) => a.to_wolfram(),
            Value::Object(o) => {
                let ref rule = WolframValue::Rule;
                let mut map = BTreeMap::new();
                for (k, v) in o {
                    map.insert(k.to_wolfram(), (rule.clone(), v.to_wolfram()));
                }
                WolframValue::Association(map)
            }
        }
    }
}

impl ToWolfram for Number {
    fn to_wolfram(&self) -> WolframValue {
        if self.is_u64() {
            self.as_u64().unwrap_or(0).to_wolfram()
        }
        else if self.is_i64() {
            self.as_i64().unwrap_or(0).to_wolfram()
        }
        else {
            self.as_f64().unwrap_or(0.0).to_wolfram()
        }
    }
}

impl From<serde_json::Error> for WolframError {
    fn from(e: serde_json::Error) -> Self {
        WolframError::SyntaxError(e.to_string())
    }
}

impl From<json5::Error> for WolframError {
    fn from(e: json5::Error) -> Self {
        WolframError::SyntaxError(e.to_string())
    }
}

/// Converts a `serde_json::Value` to a `WolframValue`.
pub fn parse_json(input: &str) -> Result<WolframValue> {
    let v: Value = serde_json::from_str(input)?;
    Ok(v.to_wolfram())
}

/// Converts a `serde_json::Value` to a `WolframValue`.
pub fn parse_json5(input: &str) -> Result<WolframValue> {
    let v: Value = json5::from_str(input)?;
    Ok(v.to_wolfram())
}
