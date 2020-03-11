mod encoding;
mod from_traits;
mod from_traits_extension;
mod systems;

use crate::{ToWolfram, WolframValue};
use serde_json::Value;
use std::collections::BTreeSet;

pub use systems::SYSTEM_SYMBOLS;

impl WolframValue {
    pub fn new_symbol(s: &str) -> WolframValue {
        WolframValue::Symbol(Box::from(s))
    }
}
