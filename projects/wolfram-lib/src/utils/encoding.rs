use crate::{WolframValue, SYSTEM_SYMBOLS};
use flate2::{write::ZlibEncoder, Compression};
use integer_encoding::VarInt;
use std::{collections::BTreeSet, io::Write, mem::transmute};

impl WolframValue {
    /// Encode a value into a byte vector.
    pub fn to_string(&self) -> String {
        format!("{}", self)
    }
    /// Encode a value into a byte vector.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut out = Vec::new();
        out.extend_from_slice(b"8:");
        self.write_bytes(&mut out);
        return out;
    }
    /// Encode a value into a compressed byte vector.
    pub fn to_compressed(&self) -> Vec<u8> {
        let mut input = Vec::new();
        let mut e = ZlibEncoder::new(vec![], Compression::new(9));
        self.write_bytes(&mut input);
        let mut out = Vec::with_capacity(input.len());
        match e.write_all(&input) {
            Ok(_) => out.extend_from_slice(b"8C:"),
            Err(..) => {}
        };
        match e.finish() {
            Ok(o) => out.extend_from_slice(&o),
            Err(..) => {
                panic!()
            }
        };
        return out;
    }
    /// Write value to a byte vector.
    pub fn write_bytes(&self, out: &mut Vec<u8>) {
        match self {
            WolframValue::Skip => (),
            WolframValue::Function(head, args) => {
                out.push(b'f');
                out.extend_from_slice(&args.len().encode_var_vec());
                head.write_bytes(out);
                for v in args {
                    v.write_bytes(out)
                }
            }
            WolframValue::String(s) => {
                let len = s.len().encode_var_vec();
                out.push(b'S');
                out.extend_from_slice(&len);
                out.extend_from_slice(s.as_bytes());
            }
            WolframValue::Bytes(v) => {
                let len = v.len().encode_var_vec();
                out.push(b'B');
                out.extend_from_slice(&len);
                out.extend_from_slice(&v);
            }
            WolframValue::Symbol(symbol) => {
                let s = standardized_symbol_name(symbol);
                let len = symbol.len().encode_var_vec();
                out.push(b's');
                out.extend_from_slice(&len);
                out.extend_from_slice(s.as_bytes());
            }
            WolframValue::Integer8(n) => {
                out.push(b'C');
                let le: [u8; 1] = unsafe { transmute(n.to_le()) };
                out.extend_from_slice(&le);
            }
            WolframValue::Integer16(n) => {
                out.push(b'j');
                let le: [u8; 2] = unsafe { transmute(n.to_le()) };
                out.extend_from_slice(&le);
            }
            WolframValue::Integer32(n) => {
                out.push(b'i');
                let le: [u8; 4] = unsafe { transmute(n.to_le()) };
                out.extend_from_slice(&le);
            }
            WolframValue::Integer64(n) => {
                out.push(b'L');
                let le: [u8; 8] = unsafe { transmute(n.to_le()) };
                out.extend_from_slice(&le);
            }
            WolframValue::BigInteger(i) => {
                out.push(b'I');
                let n = i.to_str_radix(10);
                let len = n.len().encode_var_vec();
                out.extend_from_slice(&len);
                out.extend_from_slice(n.as_bytes());
            }
            WolframValue::Decimal64(s) => {
                out.push(b'r');
                out.extend_from_slice(s);
            }
            WolframValue::BigDecimal(_) => unimplemented!(),
            WolframValue::PackedArray(_) => unimplemented!(),
            WolframValue::NumericArray(_) => unimplemented!(),
            WolframValue::Association(dict) => {
                out.push(b'A');
                out.extend_from_slice(&dict.len().encode_var_vec());
                for (k, (r, v)) in dict {
                    r.write_bytes(out);
                    k.write_bytes(out);
                    v.write_bytes(out);
                }
            }
            WolframValue::Rule => out.push(b'-'),
            WolframValue::RuleDelayed => out.push(b':'),
        }
    }
}

fn standardized_symbol_name(input: &str) -> String {
    if input.contains('`') {
        return format!("{}", input);
    }
    let mut set = BTreeSet::new();
    for sys in SYSTEM_SYMBOLS.iter() {
        set.insert(*sys);
    }
    if set.contains(input) { format!("{}", input) } else { format!("Global`{}", input) }
}
