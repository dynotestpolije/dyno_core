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

#[cfg(feature = "default")]
pub use derive_more;
#[cfg(feature = "default")]
pub use serde;
#[cfg(feature = "default")]
pub use serde_json;
#[cfg(feature = "default")]
pub use toml;
#[cfg(feature = "default")]
pub use uuid;

pub mod server;
pub use server::*;

#[cfg(feature = "use_chrono")]
pub use chrono;

#[cfg(feature = "use_log")]
pub use log;

#[cfg(feature = "use_regex")]
pub use regex;

#[cfg(feature = "backend")]
pub use actix_web;
#[cfg(feature = "backend")]
pub use sqlx;

pub use lazy_static;

pub use paste;

pub mod float {
    #[cfg(target_pointer_width = "32")]
    pub use std::f32::*;

    #[cfg(target_pointer_width = "64")]
    pub use std::f64::*;
}

#[cfg(target_pointer_width = "64")]
pub type Float = f64;

#[cfg(target_pointer_width = "32")]
pub type Float = f32;

pub const PI: Float = float::consts::PI;
pub const GRAVITY_SPEED: Float = 9.806_65;

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
