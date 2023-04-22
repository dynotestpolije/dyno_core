use std::cmp::Ordering;

use crate::{Numeric as _, SafeMath as _};

#[derive(Clone, Copy, PartialEq, Default)]
#[cfg_attr(feature = "use_serde", derive(serde::Deserialize, serde::Serialize))]
pub struct ValIdx<T: crate::Numeric> {
    pub index: f64,
    pub value: T,
}

impl<T: crate::Numeric> ValIdx<T> {
    #[inline]
    fn new(index: f64, value: impl Into<T>) -> Self {
        Self {
            index,
            value: value.into(),
        }
    }
}
impl From<[f32; 2]> for ValIdx<f32> {
    #[inline(always)]
    fn from(value: [f32; 2]) -> Self {
        Self {
            index: value[0].to_f64(),
            value: value[1],
        }
    }
}

impl From<[f64; 2]> for ValIdx<f64> {
    #[inline(always)]
    fn from(value: [f64; 2]) -> Self {
        Self {
            index: value[0],
            value: value[1],
        }
    }
}

pub const MAX_CAP_BUFFER: usize = 30_000;
#[derive(Clone)]
#[cfg_attr(feature = "use_serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Buffer<T: crate::Numeric>(Vec<ValIdx<T>>);

impl<T: crate::Numeric> std::ops::Deref for Buffer<T> {
    type Target = Vec<ValIdx<T>>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T: crate::Numeric> std::ops::DerefMut for Buffer<T> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: crate::Numeric> Default for Buffer<T> {
    #[inline(always)]
    fn default() -> Self {
        Self(Vec::with_capacity(MAX_CAP_BUFFER))
    }
}

impl<T> Buffer<T>
where
    T: crate::Numeric + std::iter::Sum + crate::SafeMath,
{
    #[inline(always)]
    pub fn new_buf(cap: usize) -> Self {
        Self(Vec::with_capacity(cap))
    }

    #[inline(always)]
    pub fn push_value(&mut self, value: T) {
        let val = ValIdx::new(self.len() as _, value);
        self.push(val)
    }

    #[inline(always)]
    pub fn iter_value(&self) -> impl Iterator<Item = T> + '_ {
        self.iter().map(|x| x.value)
    }

    #[inline(always)]
    pub fn last_value(&self) -> T {
        self.last().map(|x| x.value).unwrap_or_default()
    }

    #[inline(always)]
    pub fn first_value(&self) -> T {
        self.first().map(|x| x.value).unwrap_or_default()
    }

    #[inline(always)]
    pub fn min_value(&self) -> T {
        self.iter_value()
            .min_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal))
            .unwrap_or_default()
    }

    #[inline(always)]
    pub fn max_value(&self) -> T {
        self.iter_value()
            .max_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal))
            .unwrap_or_default()
    }

    #[inline(always)]
    pub fn sum_value(&self) -> T {
        self.iter_value().sum::<T>()
    }

    #[inline(always)]
    pub fn avg_value(&self) -> crate::Float {
        let sum = self.sum_value().to_float();
        sum.safe_div(self.len().to_float()).unwrap_or_default()
    }

    #[inline(always)]
    pub fn into_points<Out: FromIterator<[f64; 2]>>(&self) -> Out {
        FromIterator::from_iter(self.iter().map(|y| [y.index, y.value.to_f64()]))
    }
}
