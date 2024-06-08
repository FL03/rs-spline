/*
    Appellation: bspline <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::{entry::Entry, shape::*, spline::*, utils::*};

pub(crate) mod shape;
pub(crate) mod spline;

pub mod entry;

pub(crate) mod prelude {
    pub use super::spline::BSpline;
}

pub(crate) mod utils {

    pub(crate) fn _check_knot_domain<T>(knots: &[T]) -> bool
    where
        T: core::cmp::PartialOrd,
    {
        knots.windows(2).all(|w| w[0] <= w[1])
    }
    /// Given the both the number of knots and points, the degree of the spline can be calculated.
    pub fn degree(points: usize, knots: usize) -> usize {
        knots - points - 1
    }
}
