use crate::{ToWolfram, WolframValue};
use num::{bigint::Sign, rational::Ratio, BigInt, BigUint, Complex};
use std::{
    collections::{BTreeMap, BTreeSet, HashMap, HashSet, LinkedList, VecDeque},
    time::{Duration, Instant, SystemTime},
};

impl ToWolfram for WolframValue {
    fn to_wolfram(&self) -> WolframValue {
        self.clone()
    }
}

impl ToWolfram for String {
    fn to_wolfram(&self) -> WolframValue {
        self.as_str().to_wolfram()
    }
}

impl ToWolfram for &str {
    fn to_wolfram(&self) -> WolframValue {
        WolframValue::String(self.to_string())
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
        WolframValue::integer(*self)
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
        if *self <= 9223372036854775807 { (*self as i64).to_wolfram() } else { WolframValue::integer(*self) }
    }
}

impl ToWolfram for u128 {
    fn to_wolfram(&self) -> WolframValue {
        WolframValue::integer(*self)
    }
}

impl ToWolfram for isize {
    fn to_wolfram(&self) -> WolframValue {
        WolframValue::integer(*self as i64)
    }
}

impl ToWolfram for usize {
    fn to_wolfram(&self) -> WolframValue {
        WolframValue::integer(*self as u64)
    }
}

impl ToWolfram for BigUint {
    fn to_wolfram(&self) -> WolframValue {
        BigInt::from_biguint(Sign::NoSign, self.clone()).to_wolfram()
    }
}

impl ToWolfram for BigInt {
    fn to_wolfram(&self) -> WolframValue {
        WolframValue::BigInteger(self.clone())
    }
}

impl ToWolfram for f32 {
    fn to_wolfram(&self) -> WolframValue {
        WolframValue::from(*self)
    }
}

impl ToWolfram for f64 {
    fn to_wolfram(&self) -> WolframValue {
        WolframValue::from(*self)
    }
}

impl ToWolfram for Duration {
    fn to_wolfram(&self) -> WolframValue {
        todo!()
    }
}

impl ToWolfram for Instant {
    fn to_wolfram(&self) -> WolframValue {
        todo!()
    }
}

impl ToWolfram for SystemTime {
    fn to_wolfram(&self) -> WolframValue {
        todo!()
    }
}

impl<T> ToWolfram for Ratio<T>
where
    T: ToWolfram + Clone,
{
    fn to_wolfram(&self) -> WolframValue {
        let r = vec![(*self.numer()).clone(), (*self.denom()).clone()];
        WolframValue::function("Rational", r)
    }
}

impl<T> ToWolfram for Complex<T>
where
    T: ToWolfram + Copy,
{
    fn to_wolfram(&self) -> WolframValue {
        let r = vec![self.re, self.im];
        WolframValue::function("Complex", r)
    }
}

macro_rules! convert_list {
    ($($t:tt),*) => {$(
        impl<T> ToWolfram for $t<T>
        where
            T: ToWolfram,
        {
            fn to_wolfram(&self) -> WolframValue {
                WolframValue::list(self.iter().map(|s| s.to_wolfram()).collect())
            }
        }
    )*}
}
convert_list![Vec, VecDeque, LinkedList, HashSet, BTreeSet];

fn rule_pair<K, V>(pair: (&K, &V)) -> (WolframValue, (WolframValue, WolframValue))
where
    K: ToWolfram,
    V: ToWolfram,
{
    (pair.0.to_wolfram(), (WolframValue::Rule, pair.1.to_wolfram()))
}

macro_rules! convert_map {
    ($($t:tt),*) => {$(
        impl<K, V> ToWolfram for $t<K, V>
        where
            K: ToWolfram,
            V: ToWolfram,
        {
            fn to_wolfram(&self) -> WolframValue {
                WolframValue::Association(BTreeMap::from_iter(self.into_iter().map(rule_pair)))
            }
        }
    )*}
}

convert_map![HashMap, BTreeMap];
