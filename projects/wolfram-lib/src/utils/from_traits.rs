use crate::{ToWolfram, WolframValue};

impl ToWolfram for bool {
    fn to_wolfram(&self) -> WolframValue {
        if *self { WolframValue::new_symbol("True") } else { WolframValue::new_symbol("False") }
    }
}

impl ToWolfram for String {
    fn to_wolfram(&self) -> WolframValue {
        WolframValue::String(self.clone())
    }
}

impl ToWolfram for &str {
    fn to_wolfram(&self) -> WolframValue {
        WolframValue::String(self.to_string())
    }
}
