use crate::{ToWolfram, WolframValue};

/// f[x, y, z, ...]
#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub struct WolframFunction {
    head: WolframValue,
    rest: Vec<WolframValue>,
}

impl ToWolfram for WolframFunction {
    fn to_wolfram(&self) -> WolframValue {
        WolframValue::Function(Box::new(self.clone()))
    }
}

impl WolframFunction {
    /// Creates a new [`WolframFunction`] with the given name and arguments.
    pub fn new<V, I>(head: V, arguments: I) -> Self
    where
        V: ToWolfram,
        I: IntoIterator<Item = WolframValue>,
    {
        Self { head: head.to_wolfram(), rest: arguments.into_iter().collect() }
    }
    /// Creates a new [`WolframFunction`] with the given name and arguments.
    pub fn system<S, I>(name: S, arguments: I) -> Self
    where
        S: ToString,
        I: IntoIterator<Item = WolframValue>,
    {
        Self { head: WolframValue::Symbol(name.to_string()), rest: arguments.into_iter().collect() }
    }
    /// Creates a new [`WolframFunction`] with the given name and arguments.
    pub fn get_head(&self) -> &WolframValue {
        &self.head
    }
    /// Creates a new [`WolframFunction`] with the given name and arguments.
    pub fn mut_head(&mut self) -> &mut WolframValue {
        &mut self.head
    }
    /// Creates a new [`WolframFunction`] with the given name and arguments.
    pub fn set_head<V: ToWolfram>(&mut self, head: V) {
        self.head = head.to_wolfram();
    }
    /// Creates a new [`WolframFunction`] with the given name and arguments.
    pub fn with_head(mut self, head: WolframValue) -> Self {
        self.head = head;
        self
    }
    /// Creates a new [`WolframFunction`] with the given name and arguments.
    pub fn get_rest(&self) -> &[WolframValue] {
        self.rest.as_slice()
    }
    /// Creates a new [`WolframFunction`] with the given name and arguments.
    pub fn mut_rest(&mut self) -> &mut Vec<WolframValue> {
        &mut self.rest
    }
    /// Creates a new [`WolframFunction`] with the given name and arguments.
    pub fn add_rest(&mut self, rest: WolframValue) {
        self.rest.push(rest);
    }
}
