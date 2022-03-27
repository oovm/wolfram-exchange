use crate::{date_object, ToWolfram, WolframValue};
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

pub fn parse_toml(input: &str) -> Result<WolframValue, toml::de::Error> {
    Ok(input.parse::<Value>()?.to_wolfram())
}
