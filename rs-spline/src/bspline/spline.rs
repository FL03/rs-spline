/*
    Appellation: spline <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::{Shape, _check_knot_domain};
use crate::error::*;
use crate::Eval;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

use num::traits::{Num, NumOps};

/// A [B-Spline](https://mathworld.wolfram.com/B-Spline.html) is a generalization of the [Bezier curve](https://mathworld.wolfram.com/BezierCurve.html),
///
///
/// ### Resources
///
/// - [Wolfram (B-Spline)](https://mathworld.wolfram.com/B-Spline.html)
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct BSpline<T, P> {
    pub(crate) knots: Vec<T>,
    pub(crate) points: Vec<P>,
    pub(crate) shape: Shape,
}

impl<T, P> BSpline<T, P> {
    pub fn new(
        points: impl IntoIterator<Item = P>,
        knots: impl IntoIterator<Item = T>,
    ) -> Result<Self>
    where
        T: PartialOrd,
    {
        // collect the points and knots from the provided collections
        let points = Vec::from_iter(points);
        let knots = Vec::from_iter(knots);
        // check if the knot vector is valid
        if !_check_knot_domain(&knots) {
            return Err(SplineError::invalid_knot_vector());
        }
        // create and validate the shape
        let shape = Shape::new(points.len(), knots.len()).check()?;
        // return a new instance of the spline
        Ok(BSpline {
            knots,
            points,
            shape,
        })
    }
    /// Returns the degree of the spline
    pub fn degree(&self) -> usize {
        self.shape.degree()
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
    pub fn shape(&self) -> Shape {
        self.shape
    }
}

impl<P, T> BSpline<T, P>
where
    P: Copy + Num,
    T: Copy + Num + PartialOrd,
{
    /// The piecewise polynomial basis function at index `i` and order `k` at time `t`
    pub fn basis(&self, i: usize, deg: usize, time: T) -> T {
        match deg {
            0 => self.zero_basis(i, time),
            _ => self.nonzero_basis(i, deg, time),
        }
    }
    /// Evaluate the spline at a point `t`;
    /// this implementation
    ///
    ///
    /// This method is an implementation of the [de Boor's algorithm](https://en.wikipedia.org/wiki/De_Boor%27s_algorithm);
    ///
    pub fn eval(&self, t: T) -> P
    where
        P: NumOps<T, P>,
        T: NumOps<P, P>,
    {
        self.points()
            .iter()
            .copied()
            .enumerate()
            .fold(P::zero(), |acc, (i, p)| {
                acc + p * self.basis(i, self.degree(), t)
            })
    }
    /// Evaluates the spline at a point `t` within the range of the knot vector;
    /// Panics if `t` is out of range
    ///
    /// see [eval](BSpline::eval) for more information.
    pub fn eval_checked(&self, t: &T) -> P
    where
        P: NumOps<T, P>,
        T: NumOps<P, P>,
    {
        assert!(
            &self.knots[0] <= t && t <= &self.knots[self.knots.len() - 1],
            "Provided value `t` is out of range!"
        );
        self.points()
            .iter()
            .copied()
            .enumerate()
            .fold(P::zero(), |acc, (i, p)| {
                acc + p * self.basis(i, self.degree(), *t)
            })
    }
    /// Computes the spline at multiple time points
    pub fn eval_iter<I>(&self, iter: I) -> Vec<(T, P)>
    where
        I: IntoIterator<Item = T>,
        P: NumOps<T, P>,
        T: NumOps<P, P>,
    {
        iter.into_iter().map(|t| (t, self.eval(t))).collect()
    }
}

/*
 ************* Implementations *************
*/

impl<P, T> Eval<T> for BSpline<T, P>
where
    P: Copy + Num + NumOps<T, P>,
    T: Copy + Num + NumOps<P, P> + PartialOrd,
{
    type Output = P;

    fn eval(&self, t: T) -> Self::Output {
        self.eval(t)
    }
}

/*
 ************* Internal Implementations *************
*/
impl<P, T> BSpline<T, P>
where
    P: Copy + Num,
    T: Copy + Num + PartialOrd,
{
    /// internal method implementing the basis function where `k = 0`
    pub(crate) fn zero_basis(&self, i: usize, t: T) -> T {
        if self.knots[i] <= t && t < self.knots[i + 1] {
            return T::one();
        }
        T::zero()
    }
    /// internal method implementing the basis function where `k > 0`
    pub(crate) fn nonzero_basis(&self, i: usize, k: usize, t: T) -> T {
        // left-side denominator
        let lsd = if self.knots[i + k] == self.knots[i] {
            T::zero()
        } else {
            let c = (t - self.knots[i]) / (self.knots[i + k] - self.knots[i]);
            c * self.basis(i, k - 1, t)
        };
        // right-side denominator
        let rsd = if self.knots[i + k + 1] == self.knots[i + 1] {
            T::zero()
        } else {
            let c = (self.knots[i + k + 1] - t) / (self.knots[i + k + 1] - self.knots[i + 1]);
            c * self.basis(i + 1, k - 1, t)
        };

        lsd + rsd
    }
}
