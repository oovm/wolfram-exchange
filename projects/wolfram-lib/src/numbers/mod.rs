use num::complex::ComplexFloat;
use std::{
    cmp::Ordering,
    fmt::{Debug, Formatter},
};

mod display;

/// A [`WolframValue`] is a value that can be converted to a [`WolframValue`]
#[derive(Clone)]
pub struct WolframDecimal {
    repr: FloatRepr,
}

#[derive(Clone)]
enum FloatRepr {
    Null,
    PositiveInfinity,
    Safe(f64),
    BigDecimal(String),
    NegativeInfinity,
}

impl From<f32> for WolframDecimal {
    fn from(value: f32) -> Self {
        if value == f32::INFINITY {
            WolframDecimal { repr: FloatRepr::PositiveInfinity }
        }
        else if value == f32::NEG_INFINITY {
            WolframDecimal { repr: FloatRepr::NegativeInfinity }
        }
        else if value.is_nan() {
            WolframDecimal { repr: FloatRepr::Null }
        }
        else {
            WolframDecimal { repr: FloatRepr::Safe(value as f64) }
        }
    }
}

impl Eq for WolframDecimal {}

impl PartialEq for WolframDecimal {
    fn eq(&self, other: &Self) -> bool {
        match (&self.repr, &other.repr) {
            (FloatRepr::PositiveInfinity, FloatRepr::PositiveInfinity) => true,
            (FloatRepr::Safe(a), FloatRepr::Safe(b)) => a == b,
            (FloatRepr::BigDecimal(a), FloatRepr::BigDecimal(b)) => a == b,
            (FloatRepr::NegativeInfinity, FloatRepr::NegativeInfinity) => true,
            _ => false,
        }
    }
}

impl Ord for WolframDecimal {
    fn cmp(&self, other: &Self) -> Ordering {
        match (&self.repr, &other.repr) {
            (FloatRepr::PositiveInfinity, FloatRepr::PositiveInfinity) => Ordering::Equal,
            (FloatRepr::PositiveInfinity, _) => Ordering::Greater,
            (_, FloatRepr::PositiveInfinity) => Ordering::Less,
            (FloatRepr::Safe(a), FloatRepr::Safe(b)) => a.partial_cmp(b).unwrap(),
            (FloatRepr::BigDecimal(a), FloatRepr::BigDecimal(b)) => a.cmp(b),
            (FloatRepr::NegativeInfinity, FloatRepr::NegativeInfinity) => Ordering::Equal,
            (FloatRepr::NegativeInfinity, _) => Ordering::Less,
            (_, FloatRepr::NegativeInfinity) => Ordering::Greater,
            _ => unreachable!(),
        }
    }
}

impl PartialOrd for WolframDecimal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (&self.repr, &other.repr) {
            (FloatRepr::PositiveInfinity, FloatRepr::PositiveInfinity) => Some(Ordering::Equal),
            (FloatRepr::PositiveInfinity, _) => Some(Ordering::Greater),
            (_, FloatRepr::PositiveInfinity) => Some(Ordering::Less),
            (FloatRepr::Safe(a), FloatRepr::Safe(b)) => a.partial_cmp(b),
            (FloatRepr::BigDecimal(a), FloatRepr::BigDecimal(b)) => a.partial_cmp(b),
            (FloatRepr::NegativeInfinity, FloatRepr::NegativeInfinity) => Some(Ordering::Equal),
            (FloatRepr::NegativeInfinity, _) => Some(Ordering::Less),
            (_, FloatRepr::NegativeInfinity) => Some(Ordering::Greater),
            _ => unreachable!(),
        }
    }
}
