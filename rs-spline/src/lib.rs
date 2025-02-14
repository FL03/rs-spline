/*
    Appellation: rs-spline <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # rs-spline
//!
//! A generic spline library for Rust optimized for AI workloads
#![cfg_attr(not(feature = "std"), no_std)]
#![crate_name = "rs_spline"]

#[cfg(feature = "alloc")]
extern crate alloc;

pub use self::error::{Result, SplineError};
pub use self::spline::Spline;
pub use self::traits::prelude::*;

#[macro_use]
pub(crate) mod macros;
pub(crate) mod spline;

pub mod bspline;
pub mod error;
pub mod traits;

pub mod prelude {
    pub use super::bspline::prelude::*;
    pub use super::error::*;
    pub use super::traits::prelude::*;
}
