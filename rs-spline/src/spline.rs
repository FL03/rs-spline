/*
    Appellation: spline <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/



pub trait SplineData {
    type Point;
}



pub struct Spline<C, K> {
    pub(crate) degree: usize,
    pub(crate) points: Vec<C>,
    pub(crate) knots: Vec<K>,
}