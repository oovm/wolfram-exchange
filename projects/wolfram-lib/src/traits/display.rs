use super::*;
use crate::{ToWolfram, WolframFunction};

impl Display for WolframValue {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let mut indent = 0;
        self.fmt_indent(f, &mut indent)
    }
}

impl WolframFunction {
    fn fmt_indent(&self, f: &mut Formatter, indent: &mut usize) -> std::fmt::Result {
        let head = self.get_head();
        let args = self.get_rest();
        let v: Vec<String> = args.iter().map(|v| v.to_string()).collect();
        *indent += 4;
        if head.to_string() == "List" {
            write!(f, "{{{}}}", v.join(","))?
        }
        else {
            write!(f, "{}[{}]", head.to_string(), v.join(","))?
        }
        *indent -= 4;
        Ok(())
    }
}

impl WolframValue {
    fn fmt_indent(&self, f: &mut Formatter, indent: &mut usize) -> std::fmt::Result {
        match self {
            WolframValue::Skip => write!(f, ""),
            WolframValue::Function(v) => v.fmt_indent(f, indent),
            WolframValue::Boolean(v) => v.to_wolfram().fmt_indent(f, indent),

            WolframValue::String(s) => write!(f, "{:?}", s),
            WolframValue::Bytes(b) => {
                let v: Vec<String> = b.iter().map(|s| format!("{}", s)).collect();
                write!(f, "ByteArray[{{{}}}]", v.join(","))
            }
            WolframValue::Symbol(s) => write!(f, "{}", s),
            WolframValue::Integer8(i) => write!(f, "{}", i),
            WolframValue::Integer16(i) => write!(f, "{}", i),
            WolframValue::Integer32(i) => write!(f, "{}", i),
            WolframValue::Integer64(i) => write!(f, "{}", i),
            WolframValue::BigInteger(i) => write!(f, "{}", i),
            WolframValue::Decimal64(d) => write!(f, "{}`", f64::from_le_bytes(*d)),
            WolframValue::BigDecimal(d) => write!(f, "{}", d),
            WolframValue::PackedArray(_) => unimplemented!(),
            WolframValue::NumericArray(_) => unimplemented!(),
            WolframValue::Association(dict) => {
                let v: Vec<String> = dict.iter().map(|(k, (r, v))| format!("{}{}{}", k, r, v)).collect();
                write!(f, "<|{}|>", v.join(","))
            }
            WolframValue::Rule => write!(f, "->"),
            WolframValue::RuleDelayed => write!(f, ":>"),
        }
    }
}
