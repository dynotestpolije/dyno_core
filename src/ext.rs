use std::{
    fmt::{Debug, Display},
    ops::{Add, Div, Mul, Sub},
};

pub trait FloatMath {
    type Output;
    const DC: super::Float = 10.0;
    fn round_decimal(self, decimal: i32) -> Self::Output;
}
impl FloatMath for super::Float {
    type Output = super::Float;

    fn round_decimal(self, decimal: i32) -> Self::Output {
        let dp = crate::Float::powi(Self::DC, decimal);
        crate::Float::trunc(self * dp) / dp
    }
}

pub trait SafeMath {
    type Output;
    type Rhs;
    fn safe_div(self, rhs: Self::Rhs) -> Self::Output;
}

crate::macros::impl_safe_math!(i8 u8 i16 u16 i32 u32 i64 u64 isize usize);
#[cfg(has_i128)]
crate::macros::impl_safe_math!(i128);

impl SafeMath for super::Float {
    type Output = Option<super::Float>;
    type Rhs = super::Float;
    #[inline(always)]
    fn safe_div(self, rhs: Self) -> Self::Output {
        if rhs.is_normal() {
            return Some(self / rhs);
        }
        None
    }
}

pub trait FuzzyEq<Rhs: ?Sized = Self> {
    /// Returns `true` if values are approximately equal.
    fn fuzzy_eq(&self, other: &Rhs) -> bool;

    /// Returns `true` if values are not approximately equal.
    #[inline]
    fn fuzzy_ne(&self, other: &Rhs) -> bool {
        !self.fuzzy_eq(other)
    }
}
crate::macros::impl_fuzzyeq!(f32, i32);
crate::macros::impl_fuzzyeq!(f64, i64);
crate::macros::impl_fuzzyeq!(i8 u8 i16 u16 i32 u32 i64 u64 isize usize);
#[cfg(has_i128)]
crate::macros::impl_fuzzyeq!(i128);

pub trait MinMaxNumeric<Rhs = Self> {
    fn min(self, rhs: Rhs) -> Self;
    fn max(self, rhs: Rhs) -> Self;
}
/// Implemented for all builtin numeric types
pub trait Numeric:
    Sized
    + Clone
    + Copy
    + PartialEq
    + PartialOrd
    + Display
    + Debug
    + Default
    + Add<Self, Output = Self>
    + Sub<Self, Output = Self>
    + Mul<Self, Output = Self>
    + Div<Self, Output = Self>
    + MinMaxNumeric
    + FuzzyEq
{
    /// Is this an integer type?
    const INTEGRAL: bool;

    /// Smallest finite value
    const MIN: Self;

    /// Largest finite value
    const MAX: Self;

    fn to_f64(self) -> f64;
    fn to_f32(self) -> f32;
    fn from_f64(num: f64) -> Self;
    fn from_f32(num: f32) -> Self;

    fn to_float(self) -> crate::Float;
    fn from_float(num: crate::Float) -> Self;

    fn from_u64(num: u64) -> Self;
    fn from_u32(num: u32) -> Self;

    fn to_u64(self) -> u64;
    fn to_u32(self) -> u32;
}

pub struct Num<N: Numeric>(N);
impl<N: Numeric> Num<N> {
    pub fn inner(&self) -> N {
        self.0
    }
}

crate::macros::impl_numeric_float!(f32 f64);
crate::macros::impl_numeric_integer!(i8 u8 i16 u16 i32 u32 i64 u64 isize usize);

#[cfg(has_i128)]
crate::macros::impl_numeric_integer!(i128);
