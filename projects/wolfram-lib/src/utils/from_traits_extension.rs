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

#[cfg(feature = "pickle")]
mod pickle {
    use crate::{ToWolfram, WolframValue};
    use serde_pickle::Value;
    impl ToWolfram for Value {
        fn to_wolfram(&self) -> WolframValue {
            match self {
                Value::None => WolframValue::new_symbol("None"),
                Value::Bool(b) => b.to_wolfram(),
                Value::I64(i) => i.to_wolfram(),
                Value::Int(i) => i.to_wolfram(),
                Value::F64(f) => f.to_wolfram(),
                Value::Bytes(_) => unimplemented!(),
                Value::String(_) => unimplemented!(),
                Value::List(_) => unimplemented!(),
                Value::Tuple(_) => unimplemented!(),
                Value::Set(_) => unimplemented!(),
                Value::FrozenSet(_) => unimplemented!(),
                Value::Dict(_) => unimplemented!(),
            }
        }
    }
}
