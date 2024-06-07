/*
    Appellation: spline <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::error::*;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

use num::traits::{Num, NumOps};

pub(crate) fn _check_knot_domain<T>(knots: &[T]) -> bool
where
    T: PartialOrd,
{
    knots.windows(2).all(|w| w[0] <= w[1])
}

pub fn degree(points: usize, knots: usize) -> usize {
    knots - points - 1
}

/// A [B-Spline]((https://mathworld.wolfram.com/B-Spline.html)) is a generalization of the [Bezier curve](https://mathworld.wolfram.com/BezierCurve.html),
///
///
/// ### Resources
///
/// - [Wolfram (B-Spline)](https://mathworld.wolfram.com/B-Spline.html)
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct BSpline<T, P> {
    pub(crate) degree: usize,
    pub(crate) knots: Vec<T>,
    pub(crate) points: Vec<P>,
}

impl<T, P> BSpline<T, P>
where
    P: Num,
    T: Num,
{
    pub fn new(points: Vec<P>, knots: Vec<T>) -> Result<Self>
    where
        T: PartialOrd + core::fmt::Debug,
    {
        let degree = degree(points.len(), knots.len());
        if points.len() < degree {
            return Err(SplineError::NotEnoughPoints);
        }
        if knots.len() != points.len() + degree + 1 {
            return Err(SplineError::not_enough_knots(
                points.len() + degree + 1,
                knots.len(),
            ));
        }
        if !_check_knot_domain(&knots) {
            println!("{:#?}", &knots);
            return Err(SplineError::invalid_knot_vector());
        }

        Ok(BSpline {
            degree,
            knots,
            points,
        })
    }
    /// Returns the degree of the spline
    pub fn degree(&self) -> usize {
        self.degree
    }
    /// Returns an immutable reference to the knot vector
    pub const fn knots(&self) -> &Vec<T> {
        &self.knots
    }
    /// Returns a mutable reference to the knot vector
    pub fn knots_mut(&mut self) -> &mut Vec<T> {
        &mut self.knots
    }
    /// Returns an immutable reference to the control points
    pub const fn points(&self) -> &Vec<P> {
        &self.points
    }
    /// Returns a mutable reference to the control points
    pub fn points_mut(&mut self) -> &mut Vec<P> {
        &mut self.points
    }
    /// The piecewise polynomial basis function at index `i` and order `k` at time `t`
    pub fn basis(&self, i: usize, k: usize, t: T) -> T
    where
        T: Copy + PartialOrd,
    {
        match k {
            0 => self.zero_basis(i, t),
            _ => self.nonzero_basis(i, k, t),
        }
    }
    /// Computes the spline at multiple time points
    pub fn spline_iter(&self, t: Vec<T>) -> Vec<P>
    where
        P: Copy + NumOps<T, P>,
        T: Copy + NumOps<P, P> + PartialOrd,
    {
        t.iter().map(|t| self.spline(*t)).collect()
    }
    /// Computes the value of the spline at time `t`;
    ///
    /// This method is an implementation of the [de Boor's algorithm](https://en.wikipedia.org/wiki/De_Boor%27s_algorithm)
    pub fn spline(&self, t: T) -> P
    where
        P: Copy + NumOps<T, P>,
        T: Copy + NumOps<P, P> + PartialOrd,
    {
        self.points()
            .iter()
            .copied()
            .enumerate()
            .fold(P::zero(), |acc, (i, p)| {
                acc + p * self.basis(i, self.degree, t)
            })
    }
}

/*
 ************* Internal Implementations *************
*/
impl<P, T> BSpline<T, P>
where
    P: Num,
    T: Num,
{
    /// internal method implementing the basis function where `k = 0`
    pub(crate) fn zero_basis(&self, i: usize, t: T) -> T
    where
        T: Copy + PartialOrd,
    {
        if self.knots[i] <= t && t < self.knots[i + 1] {
            T::one()
        } else {
            T::zero()
        }
    }
    /// internal method implementing the basis function where `k > 0`
    pub(crate) fn nonzero_basis(&self, i: usize, k: usize, t: T) -> T
    where
        T: Copy + PartialOrd,
    {
        let mut result = T::zero();
        // left-side denominator
        let lsd = self.knots[i + k] - self.knots[i];
        if lsd != T::zero() {
            result = result + (t - self.knots[i]) / lsd * self.basis(i, k - 1, t);
        }
        // right-side denominator
        let rsd = self.knots[i + k + 1] - self.knots[i + 1];
        if rsd != T::zero() {
            result = result + (self.knots[i + k + 1] - t) / rsd * self.basis(i + 1, k - 1, t);
        }

        result
    }
}
