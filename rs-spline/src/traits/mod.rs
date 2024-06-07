/*
    Appellation: traits <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(inline)]
pub use self::prelude::*;

pub mod interpolate;
pub mod sample;
pub mod spline;

pub(crate) mod prelude {
    pub use super::interpolate::*;
    pub use super::sample::*;
    pub use super::spline::*;
}
