use crate::{ToWolfram, WolframValue};
use num::rational::Ratio;
use std::{
    collections::{BTreeMap, HashMap, HashSet, LinkedList, VecDeque},
    intrinsics::transmute,
};

impl ToWolfram for bool {
    fn to_wolfram(&self) -> WolframValue {
        if *self { WolframValue::new_symbol("True") } else { WolframValue::new_symbol("False") }
    }
}

impl ToWolfram for String {
    fn to_wolfram(&self) -> WolframValue {
        self.as_str().to_wolfram()
    }
}

impl ToWolfram for &str {
    fn to_wolfram(&self) -> WolframValue {
        WolframValue::String(Box::from(*self))
    }
}

impl ToWolfram for char {
    fn to_wolfram(&self) -> WolframValue {
        unsafe { std::str::from_utf8_unchecked(&[*self as u8]).to_wolfram() }
    }
}

impl ToWolfram for i8 {
    fn to_wolfram(&self) -> WolframValue {
        WolframValue::Integer8(*self)
    }
}

impl ToWolfram for i16 {
    fn to_wolfram(&self) -> WolframValue {
        WolframValue::Integer16(*self)
    }
}

impl ToWolfram for i32 {
    fn to_wolfram(&self) -> WolframValue {
        WolframValue::Integer32(*self)
    }
}

impl ToWolfram for i64 {
    fn to_wolfram(&self) -> WolframValue {
        WolframValue::Integer64(*self)
    }
}

impl ToWolfram for i128 {
    fn to_wolfram(&self) -> WolframValue {
        WolframValue::new_integer(*self)
    }
}

impl ToWolfram for u8 {
    fn to_wolfram(&self) -> WolframValue {
        if *self <= 127 { (*self as i8).to_wolfram() } else { (*self as i16).to_wolfram() }
    }
}

impl ToWolfram for u16 {
    fn to_wolfram(&self) -> WolframValue {
        if *self <= 32767 { (*self as i16).to_wolfram() } else { (*self as i32).to_wolfram() }
    }
}

impl ToWolfram for u32 {
    fn to_wolfram(&self) -> WolframValue {
        if *self <= 2147483647 { (*self as i32).to_wolfram() } else { (*self as i64).to_wolfram() }
    }
}

impl ToWolfram for u64 {
    fn to_wolfram(&self) -> WolframValue {
        if *self <= 9223372036854775807 { (*self as i64).to_wolfram() } else { WolframValue::new_integer(*self) }
    }
}

impl ToWolfram for u128 {
    fn to_wolfram(&self) -> WolframValue {
        WolframValue::new_integer(*self)
    }
}

impl ToWolfram for isize {
    fn to_wolfram(&self) -> WolframValue {
        WolframValue::new_integer(*self as i64)
    }
}

impl ToWolfram for usize {
    fn to_wolfram(&self) -> WolframValue {
        WolframValue::new_integer(*self as u64)
    }
}

impl ToWolfram for f32 {
    fn to_wolfram(&self) -> WolframValue {
        WolframValue::Decimal64(unsafe { transmute::<_, [u8; 8]>(*self as f64) })
    }
}

impl ToWolfram for f64 {
    fn to_wolfram(&self) -> WolframValue {
        WolframValue::Decimal64(unsafe { transmute::<_, [u8; 8]>(*self) })
    }
}

impl<T: ToWolfram + Clone> ToWolfram for Ratio<T> {
    fn to_wolfram(&self) -> WolframValue {
        let r: Vec<T> = vec![(*self.numer()).clone(), (*self.denom()).clone()];
        WolframValue::new_function("Rational", r)
    }
}

impl<T: ToWolfram> ToWolfram for Vec<T> {
    fn to_wolfram(&self) -> WolframValue {
        WolframValue::new_list(self.iter().map(|s| s.to_wolfram()).collect())
    }
}

impl<T: ToWolfram> ToWolfram for VecDeque<T> {
    fn to_wolfram(&self) -> WolframValue {
        WolframValue::new_list(self.iter().map(|s| s.to_wolfram()).collect())
    }
}

impl<T: ToWolfram> ToWolfram for LinkedList<T> {
    fn to_wolfram(&self) -> WolframValue {
        WolframValue::new_list(self.iter().map(|s| s.to_wolfram()).collect())
    }
}

impl<T: ToWolfram> ToWolfram for HashSet<T> {
    fn to_wolfram(&self) -> WolframValue {
        WolframValue::new_list(self.iter().map(|s| s.to_wolfram()).collect())
    }
}

impl<K, V> ToWolfram for BTreeMap<K, V>
where
    K: ToWolfram,
    V: ToWolfram,
{
    fn to_wolfram(&self) -> WolframValue {
        let ref rule = WolframValue::Rule;
        let mut map = BTreeMap::new();
        for (k, v) in self {
            map.insert(k.to_wolfram(), (rule.clone(), v.to_wolfram()));
        }
        WolframValue::Association(map)
    }
}

impl<K, V> ToWolfram for HashMap<K, V>
where
    K: ToWolfram,
    V: ToWolfram,
{
    fn to_wolfram(&self) -> WolframValue {
        let ref rule = WolframValue::Rule;
        let mut map = BTreeMap::new();
        for (k, v) in self {
            map.insert(k.to_wolfram(), (rule.clone(), v.to_wolfram()));
        }
        WolframValue::Association(map)
    }
}
