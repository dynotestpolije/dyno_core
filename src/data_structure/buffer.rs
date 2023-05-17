use std::cmp::Ordering;

use crate::{Numeric as _, SafeMath as _};

pub const MAX_CAP_BUFFER: usize = 30_000;
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Buffer<T: crate::Numeric>(Vec<T>);

impl<T: crate::Numeric> std::ops::Deref for Buffer<T> {
    type Target = Vec<T>;
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
        self.push(value)
    }

    #[inline(always)]
    pub fn iter_value(&self) -> impl Iterator<Item = T> + '_ {
        self.iter().copied()
    }

    #[inline(always)]
    pub fn last_value(&self) -> T {
        self.last().copied().unwrap_or_default()
    }

    #[inline(always)]
    pub fn first_value(&self) -> T {
        self.first().copied().unwrap_or_default()
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
        FromIterator::from_iter(
            self.iter()
                .enumerate()
                .map(|(idx, v)| [idx.to_f64(), v.to_f64()]),
        )
    }
}
