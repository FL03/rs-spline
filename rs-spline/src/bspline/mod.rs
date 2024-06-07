/*
    Appellation: bspline <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::{entry::Entry, spline::*};

pub(crate) mod spline;

pub mod entry;

pub(crate) mod prelude {
    pub use super::spline::BSpline;
}
