#![allow(unused_comparisons)]

mod config;
mod data_structure;
mod error;
mod ext;
mod logger;
mod macros;
mod validator;

pub mod convertions;

pub use config::*;
pub use data_structure::*;
pub use error::*;
pub use ext::*;
pub use logger::*;
pub use validator::*;

pub use bincode;
pub use derive_more;
pub use serde;
pub use serde_json;
pub use toml;

#[cfg(any(feature = "backend", feature = "frontend"))]
pub mod server;

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

pub use lazy_static;

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
pub const GRAVITY_SPEED: Float = 9.806_65;

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

#[macro_export]
macro_rules! ternary {
    (($logic:expr) ? ($trues:expr) : ($falsies:expr)) => {
        if $logic {
            $trues
        } else {
            $falsies
        }
    };
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
