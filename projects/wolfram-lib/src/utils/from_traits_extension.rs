#[cfg(feature = "yaml")]
pub use yaml::parse_yaml;

#[cfg(feature = "json")]
mod json {
    use crate::{ToWolfram, WolframValue};
    use serde_json::{Number, Value};
    use std::collections::BTreeMap;
    impl ToWolfram for Value {
        fn to_wolfram(&self) -> WolframValue {
            match self {
                Value::Null => WolframValue::new_symbol("None"),
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
}

#[cfg(feature = "yaml")]
mod yaml {
    use crate::{ToWolfram, WolframValue};
    use std::collections::BTreeMap;
    use yaml_rust::{Yaml, YamlLoader};

    impl ToWolfram for Yaml {
        fn to_wolfram(&self) -> WolframValue {
            match self {
                Yaml::Null => WolframValue::new_symbol("None"),
                Yaml::Real(n) => n.parse::<f64>().unwrap_or(0.0).to_wolfram(),
                Yaml::Integer(n) => n.to_wolfram(),
                Yaml::String(s) => s.to_wolfram(),
                Yaml::Boolean(b) => b.to_wolfram(),
                Yaml::Array(o) => o.to_wolfram(),
                Yaml::Hash(o) => {
                    let ref rule = WolframValue::Rule;
                    let mut map = BTreeMap::new();
                    for (k, v) in o {
                        map.insert(k.to_wolfram(), (rule.clone(), v.to_wolfram()));
                    }
                    WolframValue::Association(map)
                }
                Yaml::Alias(o) => o.to_wolfram(),
                Yaml::BadValue => WolframValue::new_symbol("Null"),
            }
        }
    }
    pub fn parse_yaml(input: &str) -> WolframValue {
        let parsed = YamlLoader::load_from_str(input).unwrap_or(vec![]);
        match parsed.len() {
            0 => WolframValue::new_symbol("None"),
            1 => parsed[0].to_wolfram(),
            _ => parsed.to_wolfram(),
        }
    }
}

#[cfg(feature = "pickle")]
mod pickle {
    use crate::{ToWolfram, WolframValue};
    use serde_pickle::{HashableValue, Value};
    impl ToWolfram for Value {
        fn to_wolfram(&self) -> WolframValue {
            match self {
                Value::None => WolframValue::new_symbol("None"),
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
                HashableValue::None => WolframValue::new_symbol("None"),
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
}
