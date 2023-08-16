use crate::{Float, Numeric, SafeMath};

#[derive(serde::Deserialize, serde::Serialize, Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum PointShowed {
    #[default]
    All,
    Half,
    Quarter,
    Num(usize),
}
impl std::fmt::Display for PointShowed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PointShowed::All => f.write_str("All Points"),
            PointShowed::Half => f.write_str("Half Points"),
            PointShowed::Quarter => f.write_str("Quarter Points"),
            PointShowed::Num(n) => write!(f, "{n} Points"),
        }
    }
}

impl PointShowed {
    #[inline]
    const fn showed_len(self, len: usize) -> usize {
        match self {
            PointShowed::All => len,
            PointShowed::Half => len / 2,
            PointShowed::Quarter => len - (len / 4),
            PointShowed::Num(n) => crate::ternary!((n > len)?(len): (len - n)),
        }
    }
    #[inline]
    pub const fn is_all(self) -> bool {
        matches!(self, Self::All)
    }
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Buffer<T>(Vec<T>);

impl<T: Numeric> std::ops::Deref for Buffer<T> {
    type Target = Vec<T>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T: Numeric> std::ops::DerefMut for Buffer<T> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: Sized> Default for Buffer<T> {
    #[inline(always)]
    fn default() -> Self {
        Self(Vec::with_capacity(Self::MAX_CAP_BUFFER))
    }
}

impl<T: Sized> Buffer<T> {
    pub const MAX_CAP_BUFFER: usize = 30_000;
    pub const SIZE_OF_T: usize = core::mem::size_of::<T>();
}

impl<T> Buffer<T>
where
    T: Numeric + std::iter::Sum + SafeMath,
{
    #[inline(always)]
    pub fn new_buf(cap: usize) -> Self {
        Self(Vec::with_capacity(cap))
    }

    #[inline(always)]
    pub fn push_from<I>(&mut self, value: I)
    where
        I: Into<T>,
    {
        self.push(value.into())
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
        self.iter()
            .min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .copied()
            .unwrap_or_default()
    }

    #[inline(always)]
    pub fn max_value(&self) -> T {
        self.iter()
            .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .copied()
            .unwrap_or_default()
    }

    #[inline(always)]
    pub fn sum_value(&self) -> T {
        self.iter().copied().sum::<T>()
    }

    #[inline(always)]
    pub fn avg_value(&self) -> Float {
        let sum = self.sum_value().to_float();
        sum.safe_div(self.len().to_float()).unwrap_or_default()
    }

    #[inline(always)]
    pub fn into_points<Out: FromIterator<[f64; 2]>>(&self, showed: PointShowed) -> Out {
        if showed == PointShowed::All {
            return self
                .iter()
                .enumerate()
                .map(|(idx, v)| [idx.to_f64(), v.to_f64()])
                .collect::<Out>();
        }
        let len = showed.showed_len(self.len());
        self[len..]
            .iter()
            .enumerate()
            .map(|(i, v)| [(i + len).to_f64(), v.to_f64()])
            .collect::<Out>()
    }

    #[inline(always)]
    pub fn into_points_map<Out, F>(&self, showed: PointShowed, mut map_call: F) -> Out
    where
        Out: FromIterator<[f64; 2]>,
        F: FnMut(f64) -> f64,
    {
        if showed == PointShowed::All {
            return self
                .iter()
                .enumerate()
                .map(|(idx, v)| [idx.to_f64(), map_call(v.to_f64())])
                .collect::<Out>();
        }
        let len = showed.showed_len(self.len());
        self[len..]
            .iter()
            .enumerate()
            .map(|(i, v)| [(i + len).to_f64(), map_call(v.to_f64())])
            .collect::<Out>()
    }
}

impl<T> Buffer<T>
where
    T: Sized + Clone + Copy,
{
    #[inline]
    pub fn into_inner(&self) -> Vec<T> {
        self.0.clone()
    }
}
