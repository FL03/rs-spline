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

pub trait KnotStore<T> {
    fn check_knot_domain(&self) -> bool;
    fn degree(&self, points: usize) -> usize;
}

pub trait IsSorted {
    type Item: PartialOrd;
    fn is_sorted(&self) -> bool;
}

/*
 ************* Implementations *************
*/
impl<T> IsSorted for [T]
where
    T: PartialOrd,
{
    type Item = T;

    fn is_sorted(&self) -> bool {
        self.windows(2).all(|w| w[0] <= w[1])
    }
}

impl<'a, T> IsSorted for &'a [T]
where
    T: PartialOrd,
{
    type Item = T;

    fn is_sorted(&self) -> bool {
        self.windows(2).all(|w| w[0] <= w[1])
    }
}

impl<T> IsSorted for Vec<T>
where
    T: PartialOrd,
{
    type Item = T;

    fn is_sorted(&self) -> bool {
        self.windows(2).all(|w| w[0] <= w[1])
    }
}
