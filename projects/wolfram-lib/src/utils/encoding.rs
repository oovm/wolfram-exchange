use crate::{utils::SYSTEM_SYMBOLS, WolframValue};
use serde_json::from_str;
use std::{collections::BTreeSet, mem::transmute};

impl WolframValue {
    pub fn to_string(&self) -> String {
        format!("{}", self)
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut out = Vec::new();
        out.push(b'8');
        out.push(b':');
        out.extend_from_slice(&self.to_bytes_inner());
        return out;
    }
    pub fn to_compressed(&self) -> Vec<u8> {
        unimplemented!()
    }

    fn to_bytes_inner(&self) -> Vec<u8> {
        match self {
            WolframValue::Function(name, args) => {
                let mut out = Vec::new();
                out.push(b'f');
                out.extend_from_slice(&length_encoding(args.len()));
                out.extend_from_slice(&name.to_bytes_inner());
                for v in args {
                    out.extend_from_slice(&v.to_bytes_inner())
                }
                return out;
            }
            WolframValue::String(s) => {
                let mut out = Vec::with_capacity(2 * s.len());
                out.push(b'S');
                for c in length_encoding(s.len()) {
                    out.push(c)
                }
                for c in s.as_bytes() {
                    out.push(*c)
                }
                return out;
            }
            WolframValue::Bytes(v) => {
                let len = length_encoding(v.len());
                let mut out = Vec::with_capacity(1 + len.len() + v.len());
                out.push(b'B');
                out.extend_from_slice(&len);
                out.extend_from_slice(&v);
                return out;
            }
            WolframValue::Symbol(s) => {
                let symbol = standardized_symbol_name(s);
                let mut out = Vec::with_capacity(2 * s.len());
                out.push(b's');
                for c in length_encoding(symbol.len()) {
                    out.push(c)
                }
                for c in symbol.as_bytes() {
                    out.push(*c)
                }
                return out;
            }
            WolframValue::Integer8(n) => {
                let mut v = Vec::with_capacity(2);
                v.push(b'C');
                let le: [u8; 1] = unsafe { transmute(n.to_le()) };
                for c in le.iter() {
                    v.push(*c)
                }
                return v;
            }
            WolframValue::Integer16(n) => {
                let mut v = Vec::with_capacity(3);
                v.push(b'j');
                let le: [u8; 2] = unsafe { transmute(n.to_le()) };
                for c in le.iter() {
                    v.push(*c)
                }
                return v;
            }
            WolframValue::Integer32(n) => {
                let mut v = Vec::with_capacity(5);
                v.push(b'i');
                let le: [u8; 4] = unsafe { transmute(n.to_le()) };
                for c in le.iter() {
                    v.push(*c)
                }
                return v;
            }
            WolframValue::Integer64(n) => {
                let mut v = Vec::with_capacity(9);
                v.push(b'L');
                let le: [u8; 8] = unsafe { transmute(n.to_le()) };
                for c in le.iter() {
                    v.push(*c)
                }
                return v;
            }
            WolframValue::BigInteger(i) => {
                let mut v = Vec::new();
                v.push(b'I');
                let n = i.to_str_radix(10);
                for c in length_encoding(n.len()) {
                    v.push(c)
                }
                for c in n.as_bytes() {
                    v.push(*c)
                }
                return v;
            }
            WolframValue::Decimal64(s) => {
                let f = from_str::<f64>(s).unwrap_or(0.0);
                let mut v = Vec::with_capacity(9);
                v.push(b'r');
                v.extend_from_slice(unsafe { transmute::<_, [u8; 8]>(f).as_ref() });
                return v;
            }
            WolframValue::BigDecimal(_) => unimplemented!(),
            WolframValue::PackedArray(_) => unimplemented!(),
            WolframValue::NumericArray(_) => unimplemented!(),
            WolframValue::Association(dict) => {
                let mut out = vec![];
                out.push(b'A');
                out.extend_from_slice(&length_encoding(dict.len()));
                for (k, (r, v)) in dict {
                    out.extend_from_slice(&r.to_bytes_inner());
                    out.extend_from_slice(&k.to_bytes_inner());
                    out.extend_from_slice(&v.to_bytes_inner());
                }
                return out;
            }
            WolframValue::Rule => vec![b'-'],
            WolframValue::RuleDelayed => vec![b':'],
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

///
/// ```wl
/// bits = IntegerDigits[9999, 2]
/// grouped7 = Partition[Reverse[bits], UpTo[7]]
/// grouped8 = Map[Composition[PadLeft[#, 8] &, Reverse], grouped7]
/// varint = ReplacePart[grouped8, {i_, 1} /; i < Length[grouped8] :> 1]
/// Map[FromDigits[#, 2] &, varint]
///```
fn length_encoding(len: usize) -> Vec<u8> {
    //TODO: use &[u8]
    if len <= 127 {
        return [len as u8].to_vec();
    }
    else {
        unimplemented!()
    }
}
