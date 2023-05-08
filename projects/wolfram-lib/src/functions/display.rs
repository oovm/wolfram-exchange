use super::*;

impl Debug for WolframSymbol {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.namespace.is_empty() { write!(f, "{}", self.name) } else { write!(f, "{}`{}", self.namespace, self.name) }
    }
}

impl Display for WolframSymbol {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl ToWolfram for WolframFunction {
    fn to_wolfram(&self) -> WolframValue {
        WolframValue::Function(Box::new(self.clone()))
    }
}

impl ToWolfram for WolframSymbol {
    fn to_wolfram(&self) -> WolframValue {
        WolframValue::Symbol(Box::new(self.clone()))
    }
}
