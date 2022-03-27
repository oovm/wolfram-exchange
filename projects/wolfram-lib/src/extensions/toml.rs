use crate::{date_object, Result, ToWolfram, WolframError, WolframValue};
use std::collections::BTreeMap;
use toml::Value;

impl ToWolfram for Value {
    fn to_wolfram(&self) -> WolframValue {
        match self {
            Value::String(o) => o.to_wolfram(),
            Value::Integer(o) => o.to_wolfram(),
            Value::Float(o) => o.to_wolfram(),
            Value::Boolean(o) => o.to_wolfram(),
            Value::Datetime(o) => date_object(&format!("{}", o)),
            Value::Array(o) => o.to_wolfram(),
            Value::Table(o) => {
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

impl From<toml::de::Error> for WolframError {
    fn from(e: toml::de::Error) -> Self {
        WolframError::SyntaxError(e.to_string())
    }
}

/// Convert a toml value to a Wolfram value
pub fn parse_toml(input: &str) -> Result<WolframValue> {
    Ok(input.parse::<Value>()?.to_wolfram())
}
