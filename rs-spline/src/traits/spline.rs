/*
    Appellation: spline <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use core::iter::{Product, Sum};
use num::traits::{Num, NumAssign};

pub trait SplineScalar: Clone + Num + NumAssign + PartialOrd + Product + Sum + 'static {}

/// [Sample] provides the `sample` method for producing a value at a given point along the spline
pub trait Sample<T> {
    type Output;

    fn sample(&self, t: T) -> Self::Output;
}

pub trait Spline {
    type Output;

    fn eval(&self, t: f64) -> Self::Output;
}
