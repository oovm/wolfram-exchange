use crate::{Result, ToWolfram, WolframError, WolframValue};
use std::collections::BTreeMap;
use yaml_rust::{ScanError, Yaml, YamlLoader};

impl ToWolfram for Yaml {
    fn to_wolfram(&self) -> WolframValue {
        match self {
            Yaml::Null => WolframValue::system_symbol("None"),
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
            Yaml::BadValue => WolframValue::system_symbol("Null"),
        }
    }
}

impl From<ScanError> for WolframError {
    fn from(e: ScanError) -> Self {
        WolframError::SyntaxError(format!("{}", e))
    }
}

/// Convert a YAML string to a Wolfram value.
pub fn parse_yaml(input: &str) -> Result<WolframValue> {
    let parsed = YamlLoader::load_from_str(input)?;
    let v = match parsed.len() {
        0 => WolframValue::system_symbol("None"),
        1 => parsed[0].to_wolfram(),
        _ => parsed.to_wolfram(),
    };
    Ok(v)
}
