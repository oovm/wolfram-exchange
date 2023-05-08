use crate::{WolframFunction, WolframValue};
use std::fmt::{Display, Formatter, Write};

/// A serializer for the Wolfram Language.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ReadableWriter {
    /// Whether to pretty print the output.
    pub pretty: bool,
    /// The number of spaces to indent.
    pub indent_string: &'static str,
}

impl Default for ReadableWriter {
    fn default() -> Self {
        Self { pretty: true, indent_string: "    " }
    }
}

struct ReadableBuffer {
    buffer: String,
    indent_level: usize,
    config: ReadableWriter,
}

impl ReadableBuffer {
    pub fn new(config: ReadableWriter) -> ReadableBuffer {
        Self { buffer: String::new(), indent_level: 0, config }
    }
    pub fn write_value(&mut self, value: &WolframValue) -> std::fmt::Result {
        match value {
            WolframValue::Skip => Ok(()),
            WolframValue::Function(v) => self.write_function(v),
            WolframValue::Boolean(v) => match v {
                true => write!(self.buffer, "True"),
                false => write!(self.buffer, "False"),
            },
            WolframValue::String(s) => write!(self.buffer, "{:?}", s),
            WolframValue::Bytes(b) => {
                let v: Vec<String> = b.iter().map(|s| format!("{}", s)).collect();
                write!(self.buffer, "ByteArray[{{{}}}]", v.join(","))
            }
            WolframValue::Symbol(s) => write!(self.buffer, "{}", s),
            WolframValue::Integer8(i) => write!(self.buffer, "{}", i),
            WolframValue::Integer16(i) => write!(self.buffer, "{}", i),
            WolframValue::Integer32(i) => write!(self.buffer, "{}", i),
            WolframValue::Integer64(i) => write!(self.buffer, "{}", i),
            WolframValue::BigInteger(i) => write!(self.buffer, "{}", i),
            WolframValue::Decimal64(d) => write!(self.buffer, "{}`", f64::from_le_bytes(*d)),
            WolframValue::BigDecimal(d) => write!(self.buffer, "{}", d),
            WolframValue::PackedArray(_) => unimplemented!(),
            WolframValue::NumericArray(_) => unimplemented!(),
            WolframValue::Association(dict) => {
                let v: Vec<String> = dict.iter().map(|(k, (r, v))| format!("{}{}{}", k, r, v)).collect();
                write!(self.buffer, "<|{}|>", v.join(","))
            }
            WolframValue::Rule => write!(self.buffer, "->"),
            WolframValue::RuleDelayed => write!(self.buffer, ":>"),
        }
    }
    pub fn write_function(&mut self, function: &WolframFunction) -> std::fmt::Result {
        let head = function.get_head();
        let args = function.get_rest();
        let v: Vec<String> = args.iter().map(|v| v.to_string()).collect();
        self.indent();
        if head.to_string() == "List" {
            write!(self.buffer, "{{{}}}", v.join(","))?
        }
        else {
            write!(self.buffer, "{}[{}]", head.to_string(), v.join(","))?
        }
        self.dedent();
        Ok(())
    }
    pub fn write_new_line(&mut self) -> std::fmt::Result {
        self.buffer.write_char('\n')?;
        if self.config.pretty {
            for _ in 0..self.indent_level {
                self.buffer.write_str(self.config.indent_string)?;
            }
        }
        Ok(())
    }
    pub fn indent(&mut self) {
        self.indent_level += 1;
    }
    pub fn dedent(&mut self) {
        self.indent_level -= 1;
    }
}

impl Display for WolframFunction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut buffer = ReadableBuffer::new(ReadableWriter { pretty: f.alternate(), indent_string: "    " });
        buffer.write_function(self)?;
        write!(f, "{}", buffer.buffer)
    }
}

impl Display for WolframValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut buffer = ReadableBuffer::new(ReadableWriter { pretty: f.alternate(), indent_string: "    " });
        buffer.write_value(self)?;
        write!(f, "{}", buffer.buffer)
    }
}
