use super::*;
use std::fmt::{Display, Formatter};

/// A [`WolframRule`] is a rule that can be converted to a [`WolframValue`]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Ord, PartialOrd)]
pub enum WolframRule {
    /// Represents the [->](https://reference.wolfram.com/language/ref/Rule.html) symbol.
    Rule,
    /// Represents the [:>](https://reference.wolfram.com/language/ref/RuleDelayed.html) symbol.
    RuleDelayed,
}

impl Display for WolframRule {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Rule => write!(f, "->"),
            Self::RuleDelayed => write!(f, ":>"),
        }
    }
}

impl WolframRule {
    /// Returns the byte representation of the [`WolframRule`].
    pub fn as_byte(&self) -> u8 {
        match self {
            Self::Rule => b'-',
            Self::RuleDelayed => b':',
        }
    }
}

/// A [`WolframValue`] is a value that can be converted to a [`WolframValue`]
#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub struct WolframPair {
    rule: WolframRule,
    key: WolframValue,
    value: WolframValue,
}
