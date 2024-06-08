/*
    Appellation: eval <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

pub trait Eval<T> {
    type Output;

    fn eval(&self, args: T) -> Self::Output;
}

pub trait BasisFn {
    type Output;

    fn basis(&self, loc: Scope, args: f64) -> Self::Output;
}

pub struct Scope<I = usize>(pub I, pub I);

impl<I> Scope<I> {
    pub fn new(start: I, end: I) -> Self {
        Scope(start, end)
    }
}
