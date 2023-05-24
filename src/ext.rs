use std::{
    fmt::{Debug, Display},
    ops::{Add, Div, Mul, Sub},
};

pub trait AsStr<'s> {
    fn as_str(&self) -> &'s str;
}

pub trait FloatMath {
    type Output;
    const DC: super::Float = 10.0;

    /// # rounding floating point number in specified decimal digit place
    /// ```
    /// use dyno_core::FloatMath;
    /// let value = 69.6969.round_decimal(2);
    /// assert_eq!(value, 69.70);
    /// ```
    fn round_decimal(self, decimal: i32) -> Self::Output;
}
impl FloatMath for super::Float {
    type Output = super::Float;

    fn round_decimal(self, decimal: i32) -> Self::Output {
        let factor = crate::Float::powi(Self::DC, decimal);
        crate::Float::round(self * factor) / factor
    }
}

pub trait SafeMath {
    type Output;
    type Rhs;
    /// # interface for save way to devide beetween numbers
    /// ```
    /// use dyno_core::SafeMath;
    ///
    /// let is_safe_value = 10.0.safe_div(2.0);
    /// let is_not_safe_value = 10.0.safe_div(f64::NAN);
    /// let devide_by_zero = 10.0.safe_div(0.0);
    ///
    /// assert_eq!(is_safe_value, Some(5.0));
    /// assert_eq!(is_not_safe_value, None);
    /// assert_eq!(devide_by_zero, None);
    /// ```
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

#[inline(always)]
pub fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    unsafe {
        ::core::slice::from_raw_parts((p as *const T) as *const u8, ::core::mem::size_of::<T>())
    }
}
#[inline(always)]
pub fn any_from_u8_slice<T: Sized>(b: &[u8]) -> T {
    assert!(b.len() == ::core::mem::size_of::<T>());
    unsafe { ::core::ptr::read::<T>(b.as_ptr() as *const T) }
}

pub trait BinSerializeDeserialize: serde::Serialize + serde::de::DeserializeOwned {
    #[inline(always)]
    fn serialize_bin(&self) -> crate::DynoResult<Vec<u8>> {
        bincode::serialize(self).map_err(From::from)
    }

    #[inline(always)]
    fn deserialize_bin(bin: &[u8]) -> crate::DynoResult<Self> {
        bincode::deserialize(bin).map_err(From::from)
    }

    #[deprecated(note = "use the `CompresedSaver::compress_to_file()` instead")]
    fn serialize_to_file<P: AsRef<std::path::Path>>(&self, path: P) -> crate::DynoResult<()> {
        let data = self.serialize_bin()?;
        std::fs::write(path, data).map_err(From::from)
    }

    #[deprecated(note = "use the `CompresedSaver::decompress_from_file()` instead")]
    fn deserialize_from_file<P: AsRef<std::path::Path>>(path: P) -> crate::DynoResult<Self> {
        let data = std::fs::read(path)?;
        bincode::deserialize(&data).map_err(From::from)
    }
    // add code here
}

impl<T: serde::Serialize + serde::de::DeserializeOwned> BinSerializeDeserialize for T {}

pub trait CompresedSaver: BinSerializeDeserialize {
    fn compress_to_file<P: AsRef<std::path::Path>>(&self, path: P) -> crate::DynoResult<()> {
        let serialized = self.serialize_bin()?;
        let data = miniz_oxide::deflate::compress_to_vec(&serialized, 6);
        std::fs::write(path, data).map_err(From::from)
    }
    fn decompress_from_file<P: AsRef<std::path::Path>>(path: P) -> crate::DynoResult<Self> {
        let deserialized = std::fs::read(path)?;
        let data = miniz_oxide::inflate::decompress_to_vec(&deserialized)
            .map_err(crate::DynoErr::encoding_decoding_error)?;
        bincode::deserialize(&data).map_err(From::from)
    }
}

impl<T: BinSerializeDeserialize> CompresedSaver for T {}
