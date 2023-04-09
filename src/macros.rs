macro_rules! impl_numeric_float {
    ($($t: ident)*) => {
        $(
            impl Numeric for $t {
                const INTEGRAL: bool = false;
                const MIN: Self = std::$t::MIN;
                const MAX: Self = std::$t::MAX;


                #[inline(always)]
                fn to_f64(self) -> f64 {
                    #[allow(trivial_numeric_casts)]
                    {
                        self as f64
                    }
                }
                #[inline(always)]
                fn to_f32(self) -> f32 {
                    #[allow(trivial_numeric_casts)]
                    {
                        self as f32
                    }
                }
                #[inline(always)]
                fn from_f64(num: f64) -> Self {
                    #[allow(trivial_numeric_casts)]
                    {
                        num as Self
                    }
                }
                #[inline(always)]
                fn from_f32(num: f32) -> Self {
                    #[allow(trivial_numeric_casts)]
                    {
                        num as Self
                    }
                }
                #[inline(always)]
                fn to_u32(self) -> u32 {
                    #[allow(trivial_numeric_casts)]
                    {
                        self as u32
                    }
                }
                #[inline(always)]
                fn to_u64(self) -> u64 {
                    #[allow(trivial_numeric_casts)]
                    {
                        self as u64
                    }
                }
                #[inline(always)]
                fn from_u64(num: u64) -> Self {
                    #[allow(trivial_numeric_casts)]
                    {
                        num as Self
                    }
                }
                #[inline(always)]
                fn from_u32(num: u32) -> Self {
                    #[allow(trivial_numeric_casts)]
                    {
                        num as Self
                    }
                }
                #[inline(always)]
                fn to_float(self) -> crate::Float {
                    #[allow(trivial_numeric_casts)]
                    {
                        self as crate::Float
                    }
                }
                #[inline(always)]
                fn from_float(num: crate::Float) -> Self {
                    #[allow(trivial_numeric_casts)]
                    {
                        num as Self
                    }
                }
            }
            impl MinMaxNumeric for $t {
                #[inline(always)]
                fn min(self, rhs: Self) -> Self {
                    $t::min(self, rhs)
                }
                #[inline(always)]
                fn max(self, rhs: Self) -> Self {
                    $t::max(self, rhs)
                }
            }
            paste::paste! {
                impl<T: Numeric> From<Num<T>> for $t {
                    fn from(item: Num<T>) -> Self {
                        item.0.to_float() as $t
                    }
                }
            }
        )*
    };
}

macro_rules! impl_numeric_integer {
    ($($t: ident)*) => {
        $(
            impl Numeric for $t {
                const INTEGRAL: bool = true;
                const MIN: Self = std::$t::MIN;
                const MAX: Self = std::$t::MAX;

                #[inline(always)]
                fn to_f64(self) -> f64 {
                    self as f64
                }
                #[inline(always)]
                fn to_f32(self) -> f32 {
                    self as f32
                }
                #[inline(always)]
                fn to_u32(self) -> u32 {
                    #[allow(trivial_numeric_casts)]
                    {
                        self as u32
                    }
                }
                #[inline(always)]
                fn to_u64(self) -> u64 {
                    #[allow(trivial_numeric_casts)]
                    {
                        self as u64
                    }
                }

                #[inline(always)]
                fn from_f64(num: f64) -> Self {
                    num as Self
                }

                #[inline(always)]
                fn from_f32(num: f32) -> Self {
                    num as Self
                }
                #[inline(always)]
                fn from_u64(num: u64) -> Self {
                    num as Self
                }

                #[inline(always)]
                fn from_u32(num: u32) -> Self {
                    num as Self
                }

                #[inline(always)]
                fn to_float(self) -> crate::Float {
                    self as crate::Float
                }
                #[inline(always)]
                fn from_float(num: crate::Float) -> Self {
                    num as Self
                }
            }
            impl MinMaxNumeric for $t {
                #[inline(always)]
                fn min(self, rhs: Self) -> Self {
                    Ord::min(self, rhs)
                }
                #[inline(always)]
                fn max(self, rhs: Self) -> Self {
                    Ord::max(self, rhs)
                }
            }

            impl<T: Numeric> From<Num<T>> for $t {
                fn from(item: Num<T>) -> Self {
                    #[allow(trivial_numeric_casts)]
                    {
                        item.0.to_float() as $t
                    }
                }
            }
        )*
    };
}

macro_rules! impl_fuzzyeq {
    ($tp: ty, $ulps: ty) => {
        impl $crate::ext::FuzzyEq for $tp {
            #[inline]
            fn fuzzy_eq(&self, other: &$tp) -> bool {
                if *self == *other {
                    return true;
                }

                if self.is_sign_positive() != other.is_sign_positive() {
                    return false;
                }
                let diff: $ulps = {
                    let a: $ulps = self.to_bits() as $ulps;
                    let b: $ulps = other.to_bits() as $ulps;
                    a.wrapping_sub(b)
                };
                (-4..=4).contains(&diff)
            }
        }
    };
    ($($tp:ty)*) => {
        $(
            impl $crate::ext::FuzzyEq for $tp {
                #[inline]
                fn fuzzy_eq(&self, other: &$tp) -> bool {
                    self.eq(other)
                }
            }
        )*
    };
}

macro_rules! impl_safe_math {
    ($($t:ty)*) => {
        $(impl SafeMath for $t {
            type Output = Option<$t>;
            type Rhs = Self;
            #[inline(always)]
            fn safe_div(self, rhs: $t) -> Self::Output {
                if rhs == 0 {
                    return None
                }
                Some(self / rhs)
            }
        })*
    };
}

pub(crate) use {impl_fuzzyeq, impl_numeric_float, impl_numeric_integer, impl_safe_math};
