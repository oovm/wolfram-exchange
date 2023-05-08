use crate::{ToWolfram, WolframValue};
use std::fmt::{Debug, Display, Formatter};

mod display;

/// A symbol in the Wolfram Language.
#[derive(Clone, PartialEq, Eq, Ord, PartialOrd)]
pub struct WolframSymbol {
    namespace: String,
    name: String,
}

impl WolframSymbol {
    /// Creates a new [`WolframSymbol`] with the given name.
    pub fn new<T, S>(namespace: T, name: S) -> Self
    where
        T: ToString,
        S: ToString,
    {
        let namespace = namespace.to_string();
        let name = name.to_string();
        debug_assert!(Self::is_valid(namespace.as_ref()));
        debug_assert!(Self::is_valid(name.as_ref()));
        Self { namespace, name }
    }
    /// Creates a new [`WolframSymbol`] with the given name.
    pub fn global<S>(name: S) -> Self
    where
        S: ToString,
    {
        let name = name.to_string();
        debug_assert!(Self::is_valid(name.as_ref()));
        Self { namespace: "Global".to_string(), name }
    }
    /// Creates a new [`WolframSymbol`] with the given name.
    pub fn boolean(bool: bool) -> Self {
        Self::system(if bool { "True" } else { "False" })
    }

    /// Creates a new [`WolframSymbol`] with the given name.
    pub fn system<S>(name: S) -> Self
    where
        S: ToString,
    {
        let name = name.to_string();
        debug_assert!(Self::is_valid(name.as_ref()));
        Self { namespace: String::new(), name }
    }
    /// Creates a new [`WolframSymbol`] with the given name.
    pub fn is_valid(name: &str) -> bool {
        name.chars().all(|c| c.is_ascii_alphanumeric() || c == '`')
    }
}

/// f[x, y, z, ...]
#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub struct WolframFunction {
    head: WolframValue,
    rest: Vec<WolframValue>,
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
    ///
    pub fn namespace<S, I>(namespace: S, name: S, arguments: I) -> Self
    where
        S: ToString,
        I: IntoIterator<Item = WolframValue>,
    {
        Self { head: WolframSymbol::new(namespace, name).to_wolfram(), rest: arguments.into_iter().collect() }
    }
    /// Creates a new [`WolframFunction`] with the given name and arguments.
    pub fn global<S, I>(name: S, arguments: I) -> Self
    where
        S: ToString,
        I: IntoIterator<Item = WolframValue>,
    {
        Self { head: WolframSymbol::global(name).to_wolfram(), rest: arguments.into_iter().collect() }
    }
    /// Creates a new [`WolframFunction`] with the given name and arguments.
    pub fn system<S, I>(name: S, arguments: I) -> Self
    where
        S: ToString,
        I: IntoIterator<Item = WolframValue>,
    {
        Self { head: WolframSymbol::system(name).to_wolfram(), rest: arguments.into_iter().collect() }
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
