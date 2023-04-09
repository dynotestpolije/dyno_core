pub mod angular;
pub mod length;
pub mod speed;
pub mod temperature;

macro_rules! declare_convertion_type {
    ($impls_type:ty => $s:ident {
            $($types:ident[$fmt:literal] [$($impls_func:ident => $tp_impl:ident {$factor:expr}),*]),*
        }) => {
        $(
            #[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd, derive_more::AddAssign)]
            #[cfg_attr(feature = "use_serde", derive(serde::Deserialize, serde::Serialize))]
            pub struct $types($crate::Float);
            impl $types {
                #[inline(always)]
                pub fn new(num: impl $crate::ext::Numeric) -> Self {
                    Self(num.to_float())
                }
                pub const fn name_type(&self) -> &'static str {
                    stringify!($types)
                }
            }
            impl $impls_type for $types {
                $(
                    #[inline(always)]
                    fn $impls_func($s) -> $tp_impl {
                        $tp_impl::new($factor)
                    }
                )*
            }
            impl std::fmt::Display for $types {
                #[inline(always)]
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "{:.2} {}", self.0, $fmt)
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
                fn add(self, rhs: Self) -> Self{
                    Self(self.0.add(rhs.0))
                }
            }
            impl std::ops::Sub<Self> for $types {
                type Output = Self;
                #[inline(always)]
                fn sub(self, rhs: Self) -> Self{
                    Self(self.0.sub(rhs.0))
                }
            }
            impl std::ops::Div<Self> for $types {
                type Output = Self;
                #[inline(always)]
                fn div(self, rhs: Self) -> Self{
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
        )*
    };
}

pub(self) use declare_convertion_type;

pub mod prelude {
    pub use super::angular::*;
    pub use super::length::*;
    pub use super::speed::*;
    pub use super::temperature::*;

    pub const MAX_TICKS: crate::Float = 2.77777e-03;
    pub trait EncoderTicks: crate::ext::Numeric + crate::ext::SafeMath {
        fn per_minute(self, ms: Self) -> Self;
        fn per_second(self, ms: Self) -> Self;
        fn per_hour(self, ms: Self) -> Self;
        fn araund(self, diameter_cm: Self) -> CentiMetres;
    }

    impl EncoderTicks for crate::Float {
        #[inline(always)]
        fn per_minute(self, ms: Self) -> Self {
            let m = ms * 1.66667e-5;
            if m.is_normal() {
                self / m
            } else {
                Default::default()
            }
        }
        #[inline(always)]
        fn per_second(self, ms: Self) -> Self {
            let s = ms * 0.001;
            if s.is_normal() {
                self / s
            } else {
                Default::default()
            }
        }

        #[inline(always)]
        fn per_hour(self, ms: Self) -> Self {
            let h = ms * 2.77778e-7;
            if h.is_normal() {
                self / h
            } else {
                Default::default()
            }
        }

        #[inline]
        fn araund(self, diameter_cm: Self) -> CentiMetres {
            CentiMetres::new(self * (crate::PI * diameter_cm))
        }
    }
}
