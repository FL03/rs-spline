/*
    Appellation: shape <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::error::SplineError;

pub(crate) fn _is_shape_valid(degree: usize, knots: usize, points: usize) -> bool {
    points >= degree && knots == points + degree + 1
}

pub(crate) fn _check_shape(shape: &Shape) -> Result<Shape, SplineError> {
    if !shape.is_valid() {
        let Shape {
            degree,
            knots,
            points,
        } = *shape;
        let exp = super::degree(points, knots); // expected degree
        if points < degree {
            return Err(SplineError::NotEnoughPoints);
        } else if knots != exp {
            return Err(SplineError::not_enough_knots(exp, knots));
        } else {
            return Err(SplineError::unknown(""));
        }
    }
    Ok(*shape)
}

pub(crate) fn _validate_shape(shape: &Shape) -> Result<(), SplineError> {
    if !shape.is_valid() {
        let Shape {
            degree,
            knots,
            points,
        } = *shape;
        let exp = super::degree(points, knots); // expected degree
        if points < degree {
            return Err(SplineError::NotEnoughPoints);
        } else if knots != exp {
            return Err(SplineError::not_enough_knots(exp, knots));
        } else {
            return Err(SplineError::unknown(""));
        }
    }
    Ok(())
}
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Shape {
    pub(crate) degree: usize,
    pub(crate) knots: usize,
    pub(crate) points: usize,
}

impl Shape {
    pub fn new(points: usize, knots: usize) -> Self {
        let degree = super::degree(points, knots);

        Shape {
            degree,
            knots,
            points,
        }
    }

    pub fn from_degree_knots(degree: usize, knots: usize) -> Self {
        let points = knots - degree - 1;

        Shape {
            degree,
            knots,
            points,
        }
    }

    pub fn from_degree_points(degree: usize, points: usize) -> Self {
        let knots = points + degree + 1;

        Shape {
            degree,
            knots,
            points,
        }
    }
    ///
    pub fn check(self) -> Result<Self, SplineError> {
        self.validate().map(|_| self)
    }

    ///
    pub fn validate(&self) -> Result<(), SplineError> {
        _validate_shape(self)
    }

    pub fn degree(&self) -> usize {
        self.degree
    }

    pub fn knots(&self) -> usize {
        self.knots
    }

    pub fn points(&self) -> usize {
        self.points
    }
    /// Validate the shape of the spline by:
    /// - Ensuring that the number of points is greater than the degree
    /// - Ensuring that the number of knots is equal to the number of points plus the degree plus one
    pub fn is_valid(&self) -> bool {
        self.points() >= self.degree() && self.knots() == self.points() + self.degree() + 1
    }
}

impl core::fmt::Display for Shape {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f,
            "{{\"degree\": \"{}\", \"knots\": \"{}\", \"points\": \"{}\"}}",
            self.degree, self.knots, self.points
        )
    }
}
