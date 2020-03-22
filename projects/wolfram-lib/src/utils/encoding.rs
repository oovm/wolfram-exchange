use crate::{utils::SYSTEM_SYMBOLS, WolframValue};
use flate2::{write::ZlibEncoder, Compression};
use std::{collections::BTreeSet, io::Write, mem::transmute};

impl WolframValue {
    pub fn to_string(&self) -> String {
        format!("{}", self)
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut out = Vec::new();
        out.extend_from_slice(b"8:");
        out.extend_from_slice(&self.to_bytes_inner());
        return out;
    }
    pub fn to_compressed(&self) -> Vec<u8> {
        let mut out = Vec::new();
        let mut e = ZlibEncoder::new(vec![], Compression::new(9));
        if let Ok(_) = e.write_all(&self.to_bytes_inner()) {
            out.extend_from_slice(b"8C:")
        };
        if let Ok(o) = e.finish() {
            out.extend_from_slice(&o)
        };
        return out;
    }
    fn to_bytes_inner(&self) -> Vec<u8> {
        match self {
            WolframValue::Function(name, args) => {
                let mut out = Vec::new();
                out.push(b'f');
                out.extend_from_slice(&length_encoding(args.len()));
                out.extend_from_slice(&WolframValue::new_symbol(name).to_bytes_inner());
                for v in args {
                    out.extend_from_slice(&v.to_bytes_inner())
                }
                return out;
            }
            WolframValue::String(s) => {
                let ref len = length_encoding(s.len());
                let mut out = Vec::with_capacity(1 + len.len() + s.len());
                out.push(b'S');
                out.extend_from_slice(len);
                out.extend_from_slice(s.as_bytes());
                return out;
            }
            WolframValue::Bytes(v) => {
                let ref len = length_encoding(v.len());
                let mut out = Vec::with_capacity(1 + len.len() + v.len());
                out.push(b'B');
                out.extend_from_slice(len);
                out.extend_from_slice(&v);
                return out;
            }
            WolframValue::Symbol(symbol) => {
                let s = standardized_symbol_name(symbol);
                let ref len = length_encoding(symbol.len());
                let mut out = Vec::with_capacity(1 + len.len() + s.len());
                out.push(b's');
                out.extend_from_slice(len);
                out.extend_from_slice(s.as_bytes());
                return out;
            }
            WolframValue::Integer8(n) => {
                let le: [u8; 1] = unsafe { transmute(n.to_le()) };
                let mut v = Vec::with_capacity(2);
                v.push(b'C');
                v.extend_from_slice(le.as_ref());
                return v;
            }
            WolframValue::Integer16(n) => {
                let le: [u8; 2] = unsafe { transmute(n.to_le()) };
                let mut v = Vec::with_capacity(3);
                v.push(b'j');
                v.extend_from_slice(le.as_ref());
                return v;
            }
            WolframValue::Integer32(n) => {
                let le: [u8; 4] = unsafe { transmute(n.to_le()) };
                let mut v = Vec::with_capacity(5);
                v.push(b'i');
                v.extend_from_slice(le.as_ref());
                return v;
            }
            WolframValue::Integer64(n) => {
                let le: [u8; 8] = unsafe { transmute(n.to_le()) };
                let mut v = Vec::with_capacity(9);
                v.push(b'L');
                v.extend_from_slice(le.as_ref());
                return v;
            }
            WolframValue::BigInteger(i) => {
                let n = i.to_str_radix(10);
                let ref len = length_encoding(n.len());
                let mut v = Vec::with_capacity(1 + len.len() + n.len());
                v.push(b'I');
                v.extend_from_slice(len);
                v.extend_from_slice(n.as_bytes());
                return v;
            }
            WolframValue::Decimal64(s) => {
                let mut v = Vec::with_capacity(9);
                v.push(b'r');
                v.extend_from_slice(s);
                return v;
            }
            WolframValue::BigDecimal(_) => unimplemented!(),
            WolframValue::PackedArray(_) => unimplemented!(),
            WolframValue::NumericArray(_) => unimplemented!(),
            WolframValue::Association(dict) => {
                let mut out = Vec::with_capacity(dict.len());
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
