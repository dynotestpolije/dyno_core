#![allow(unused_comparisons)]

mod data_structure;
mod error;
mod ext;
mod logger;
mod macros;

pub mod convertions;

pub use data_structure::*;
pub use error::*;
pub use ext::*;

#[cfg(any(feature = "backend", feature = "frontend"))]
pub mod server;

#[cfg(feature = "use_serde")]
pub use bincode;
#[cfg(feature = "use_serde")]
pub use derive_more;
#[cfg(feature = "use_serde")]
pub use serde_json;
#[cfg(feature = "use_serde")]
pub use toml;

#[cfg(feature = "use_chrono")]
pub use chrono;

#[cfg(feature = "use_log")]
pub use log;

#[cfg(feature = "use_once_cell")]
pub use once_cell;

#[cfg(feature = "use_regex")]
pub use regex;

#[cfg(feature = "frontend")]
pub use reqwest;

pub use paste;

pub mod float {
    #[cfg(not(feature = "bigger_float"))]
    pub use std::f32::*;
    #[cfg(feature = "bigger_float")]
    pub use std::f64::*;
}

#[cfg(feature = "bigger_float")]
pub type Float = f64;
#[cfg(feature = "bigger_float")]
pub const PI: Float = float::consts::PI;

#[cfg(not(feature = "bigger_float"))]
pub type Float = f32;
#[cfg(not(feature = "bigger_float"))]
pub const PI: Float = float::consts::PI;

#[inline(always)]
pub fn linspace<N>(start: N, stop: N, nstep: u32) -> impl Iterator<Item = N>
where
    N: ext::Numeric,
{
    let delta: N = (stop - start) / N::from_u32(nstep - 1);
    (0..(nstep)).map(move |i| start + N::from_u32(i) * delta)
}

#[macro_export]
macro_rules! decl_constants {
    ($($sv:vis $name:ident => $content:expr),*) => (
        $(#[allow(missing_docs)] $sv const $name: &'static str = $content;)*
    );
    ($($name:ident => $content:expr),*) => (
        $(#[allow(missing_docs)] const $name: &'static str = $content;)*
    );
}

pub trait ResultHandler<'err, T, E> {
    fn dyn_err(self) -> DynoResult<'err, T>;
}

impl<'err, T, E> ResultHandler<'err, T, E> for std::result::Result<T, E>
where
    DynoErr<'err>: From<E>,
{
    #[inline(always)]
    fn dyn_err(self) -> std::result::Result<T, DynoErr<'err>> {
        match self {
            Ok(res) => Ok(res),
            Err(err) => Err(DynoErr::from(err)),
        }
    }
}

#[macro_export]
macro_rules! validate_error {
    ($($args:tt)*) => (Err($crate::DynoErr::new(format!($($args)*), $crate::ErrKind::ValidateError)));
}

pub trait Validate: Sized + std::fmt::Display {
    fn validate(&self) -> DynoResult<'_, ()>;
}
