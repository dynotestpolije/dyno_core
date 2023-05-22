pub mod angular;
pub mod length;
pub mod power;
pub mod speed;
pub mod temperature;
pub mod torque;
pub mod weight;

#[repr(i32)]
#[derive(Default)]
pub enum Metrix {
    Pico = -12,
    Nano = -9,
    Micro = -6,
    Milli = -3,
    Centi = -2,
    #[default]
    One = 0,
    Kilo = 3,
    Mega = 6,
    Giga = 9,
    Tera = 12,
}
impl Metrix {
    pub fn from_num<N: crate::Numeric>(self, num: N) -> N {
        num * N::from_float(crate::Float::powi(10.0, self as i32))
    }
}

macro_rules! declare_std_convertion_type {
    ($types:ident[$fmt:literal]) => {
        #[derive(
            Debug,
            Default,
            Clone,
            Copy,
            PartialEq,
            PartialOrd,
            derive_more::AddAssign,
            serde::Deserialize,
            serde::Serialize,
        )]
        pub struct $types($crate::Float);
        impl $types {
            #[inline(always)]
            pub fn new(num: impl $crate::ext::Numeric) -> Self {
                Self(num.to_float())
            }
            #[inline]
            pub const fn name_type(&self) -> &'static str {
                stringify!($types)
            }
            #[inline]
            pub fn value_fmt(self) -> String {
                format!("{}", self.0)
            }
            #[inline]
            pub fn value(self) -> $crate::Float {
                self.0
            }
        }
        impl std::fmt::Display for $types {
            #[inline(always)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{:.2}", self.0)
            }
        }

        impl $crate::ext::SafeMath for $types {
            type Output = Option<$crate::Float>;
            type Rhs = $crate::Float;

            #[inline(always)]
            fn safe_div(self, rhs: Self::Rhs) -> Self::Output {
                if self.0.is_normal() {
                    Some(self.0 / rhs)
                } else {
                    None
                }
            }
        }

        impl $crate::ext::FloatMath for $types {
            type Output = $crate::Float;

            #[inline(always)]
            fn round_decimal(self, decimal: i32) -> Self::Output {
                let dp = Self::DC.powi(decimal);
                $crate::Float::trunc(self.0 * dp) / dp
            }
        }

        impl From<$crate::Float> for $types {
            #[inline(always)]
            fn from(item: $crate::Float) -> Self {
                Self(item)
            }
        }

        impl $crate::ext::FuzzyEq for $types {
            #[inline(always)]
            fn fuzzy_eq(&self, other: &Self) -> bool {
                self.0.fuzzy_eq(&other.0)
            }
        }

        impl std::ops::Add<Self> for $types {
            type Output = Self;
            #[inline(always)]
            fn add(self, rhs: Self) -> Self {
                Self(self.0.add(rhs.0))
            }
        }
        impl std::ops::Sub<Self> for $types {
            type Output = Self;
            #[inline(always)]
            fn sub(self, rhs: Self) -> Self {
                Self(self.0.sub(rhs.0))
            }
        }
        impl std::ops::Div<Self> for $types {
            type Output = Self;
            #[inline(always)]
            fn div(self, rhs: Self) -> Self {
                Self(self.0.div(rhs.0))
            }
        }

        impl std::ops::Mul<Self> for $types {
            type Output = Self;
            #[inline(always)]
            fn mul(self, rhs: Self) -> Self {
                Self(self.0.mul(rhs.0))
            }
        }

        impl std::ops::Add<$crate::Float> for $types {
            type Output = Self;
            #[inline(always)]
            fn add(self, rhs: $crate::Float) -> Self::Output {
                Self(self.0.add(rhs))
            }
        }
        impl std::ops::Sub<$crate::Float> for $types {
            type Output = Self;
            #[inline(always)]
            fn sub(self, rhs: $crate::Float) -> Self::Output {
                Self(self.0.sub(rhs))
            }
        }
        impl std::ops::Div<$crate::Float> for $types {
            type Output = Self;
            #[inline(always)]
            fn div(self, rhs: $crate::Float) -> Self::Output {
                Self(self.0.div(rhs))
            }
        }

        impl std::ops::Mul<$crate::Float> for $types {
            type Output = Self;
            #[inline(always)]
            fn mul(self, rhs: $crate::Float) -> Self::Output {
                Self(self.0.mul(rhs))
            }
        }
        impl std::iter::Sum for $types {
            fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
                iter.sum()
            }
        }

        impl $crate::ext::Numeric for $types {
            const INTEGRAL: bool = false;
            const MIN: Self = Self($crate::Float::MAX);
            const MAX: Self = Self($crate::Float::MIN);

            #[inline(always)]
            fn to_f64(self) -> f64 {
                self.0.to_f64()
            }
            #[inline(always)]
            fn to_f32(self) -> f32 {
                self.0.to_f32()
            }
            #[inline(always)]
            fn from_f64(num: f64) -> Self {
                Self::new(num as $crate::Float)
            }
            #[inline(always)]
            fn from_f32(num: f32) -> Self {
                Self::new(num as $crate::Float)
            }
            #[inline(always)]
            fn from_u64(num: u64) -> Self {
                Self::new(num as $crate::Float)
            }
            #[inline(always)]
            fn from_u32(num: u32) -> Self {
                Self::new(num as $crate::Float)
            }
            #[inline(always)]
            fn to_float(self) -> $crate::Float {
                self.0
            }
            #[inline(always)]
            fn from_float(num: $crate::Float) -> Self {
                Self::new(num)
            }
            #[inline(always)]
            fn to_u32(self) -> u32 {
                #[allow(trivial_numeric_casts)]
                {
                    self.0 as u32
                }
            }
            #[inline(always)]
            fn to_u64(self) -> u64 {
                #[allow(trivial_numeric_casts)]
                {
                    self.0 as u64
                }
            }
        }

        impl $crate::ext::MinMaxNumeric for $types {
            #[inline(always)]
            fn min(self, rhs: Self) -> Self {
                Self(self.0.min(rhs.0))
            }
            #[inline(always)]
            fn max(self, rhs: Self) -> Self {
                Self(self.0.max(rhs.0))
            }
        }
    };
}

macro_rules! declare_convertion_type {
    ($impls_type:ty => $s:ident {
            $($types:ident[$fmt:literal] [$($impls_func:ident => $tp_impl:ident {$factor:expr}),*]),*
        }) => {
        $(
            super::declare_std_convertion_type!( $types[$fmt] );
            impl $impls_type for $types {
                $(
                    #[inline(always)]
                    fn $impls_func($s) -> $tp_impl {
                        $tp_impl::new($factor)
                    }
                )*
            }

            impl std::str::FromStr for $types {
                type Err = <crate::Float as std::str::FromStr>::Err;
                fn from_str(s: &str) -> Result<Self, Self::Err> {
                    Ok(Self(s.parse::<crate::Float>()?))
                }
            }

            impl $types {
                /// ## check if self is negative sign number and normal number (Nan, Inf, etc)
                /// if true, then return the `def` default parameter
                /// ```
                /// use dyno_core::convertions::length::Metres;
                /// let def_param = Metres::new(1.0);
                /// let m = Metres::new(dyno_core::Float::NAN).if_not_normal(def_param);
                /// assert_eq!(m, def_param);
                /// ```
                #[inline(always)]
                pub fn if_not_normal(self, def: Self) -> Self {
                    if self.0.is_infinite() || self.0.is_nan() {
                        return def;
                    }
                    self
                }
                /// ## check if self is negative sign number and normal number (Nan, Inf, etc)
                /// if true, then return the `def` default parameter
                /// ```
                /// use dyno_core::convertions::length::Metres;
                /// let def_param = Metres::new(1.0);
                /// let negative = Metres::new(-1.).if_negative_normal(def_param);
                /// assert_eq!(negative, def_param);
                ///
                /// let not_normal = Metres::new(dyno_core::Float::NAN).if_negative_normal(def_param);
                /// assert_eq!(not_normal, def_param);
                /// ```
                #[inline(always)]
                pub fn if_negative_normal(self, def: Self) -> Self {
                    if self.0.is_sign_negative() || self.0.is_infinite() || self.0.is_nan()  {
                        return def;
                    }
                    self
                }
            }
        )*
    };
}

pub(self) use {declare_convertion_type, declare_std_convertion_type};

pub mod prelude {
    use crate::SafeMath;

    pub use super::angular::*;
    pub use super::length::*;
    pub use super::power::*;
    pub use super::speed::*;
    pub use super::temperature::*;
    pub use super::torque::*;
    pub use super::weight::*;

    pub trait EncoderTicks: crate::ext::Numeric + crate::ext::SafeMath {
        /// # implements per time function from milliseconds (minute)
        /// ```
        /// use dyno_core::convertions::prelude::EncoderTicks;
        /// use dyno_core::FloatMath;
        /// let value = 10.0.per_minute(100.0).round_decimal(2);
        /// assert_eq!(value, 5999.99)
        /// ```
        #[inline(always)]
        fn per_minute(self, ms: Self) -> Self {
            Self::from_float(
                self.to_float()
                    .safe_div(ms.to_float() * 1.66667e-5)
                    .unwrap_or_default(),
            )
        }

        /// # implements per time function from milliseconds (seconds)
        /// ```
        /// use dyno_core::convertions::prelude::EncoderTicks;
        /// use dyno_core::FloatMath;
        /// let value = 10.0.per_second(100.0).round_decimal(1);
        /// assert_eq!(value, 100.0)
        /// ```
        #[inline(always)]
        fn per_second<N: crate::ext::Numeric>(self, ms: N) -> Self {
            Self::from_float(
                self.to_float()
                    .safe_div(ms.to_float() * 0.001)
                    .unwrap_or_default(),
            )
        }

        /// # implements per time function from milliseconds (hour)
        /// ```
        /// use dyno_core::convertions::prelude::EncoderTicks;
        /// use dyno_core::FloatMath;
        /// let value = 10.0.per_hour(100.0).round_decimal(4);
        /// assert_eq!(value, 359999.712)
        /// ```
        #[inline(always)]
        fn per_hour(self, ms: Self) -> Self {
            Self::from_float(
                self.to_float()
                    .safe_div(ms.to_float() * 2.77778e-7)
                    .unwrap_or_default(),
            )
        }
        fn araund(self, diameter_cm: Metres) -> Metres {
            (diameter_cm * crate::PI) * Metres::new(self)
        }
    }

    impl<N: crate::ext::Numeric + crate::ext::SafeMath> EncoderTicks for N {}
}
