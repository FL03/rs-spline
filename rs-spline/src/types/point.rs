/*
    Appellation: point <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize,))]
pub struct Point<A, B = A>(A, B);

impl<A, B> Point<A, B> {
    pub fn new(x: A, y: B) -> Self {
        Self(x, y)
    }

    pub fn from_tuple((x, y): (A, B)) -> Self {
        Self(x, y)
    }

    pub const fn as_tuple(&self) -> (&A, &B) {
        (self.lhs(), self.rhs())
    }

    pub fn as_mut_tuple(&mut self) -> (&mut A, &mut B) {
        (&mut self.0, &mut self.1)
    }

    pub const fn lhs(&self) -> &A {
        &self.0
    }

    pub fn lhs_mut(&mut self) -> &mut A {
        &mut self.0
    }

    pub const fn rhs(&self) -> &B {
        &self.1
    }

    pub fn rhs_mut(&mut self) -> &mut B {
        &mut self.1
    }

    pub fn into_tuple(self) -> (A, B) {
        (self.0, self.1)
    }
}

/*
 ************* Implementations *************
*/

impl<A, B> From<(A, B)> for Point<A, B> {
    fn from(tuple: (A, B)) -> Self {
        Self::from_tuple(tuple)
    }
}

macro_rules! impl_ops {
    ($($($trait:ident)::*.$method:ident),* $(,)?) => {
        $(impl_ops!(@impl $($trait)::*.$method);)*
    };
    (@impl $($trait:ident)::*.$method:ident) => {
        impl<A, B, C, D> $($trait)::*<Point<C, D>> for Point<A, B>
        where
            A: $($trait)::*<C>,
            B: $($trait)::*<D>,
        {
            type Output = Point<A::Output, B::Output>;

            fn $method(self, rhs: Point<C, D>) -> Self::Output {
                Point(self.0.$method(rhs.0), self.1.$method(rhs.1))
            }
        }

        impl<'a, A, B, C, D> $($trait)::*<Point<C, D>> for &'a Point<A, B>
        where
            A: $($trait)::*<C>,
            B: $($trait)::*<D>,
            Point<A, B>: Clone,
        {
            type Output = Point<A::Output, B::Output>;

            fn $method(self, rhs: Point<C, D>) -> Self::Output {
                self.clone().$method(rhs)
            }
        }

        impl<'a, A, B, C, D> $($trait)::*<&'a Point<C, D>> for &'a Point<A, B>
        where
            A: $($trait)::*<C>,
            B: $($trait)::*<D>,
            Point<A, B>: Clone,
            Point<C, D>: Clone,
        {
            type Output = Point<A::Output, B::Output>;

            fn $method(self, rhs: &'a Point<C, D>) -> Self::Output {
                self.$method(rhs.clone())
            }
        }
        impl<'a, A, B, C, D> $($trait)::*<&'a Point<C, D>> for Point<A, B>
        where
            A: $($trait)::*<C>,
            B: $($trait)::*<D>,
            Point<C, D>: Clone,
        {
            type Output = Point<A::Output, B::Output>;

            fn $method(self, rhs: &'a Point<C, D>) -> Self::Output {
                self.$method(rhs.clone())
            }
        }
    };
}

impl_ops!(
    core::ops::Add.add,
    core::ops::Div.div,
    core::ops::Mul.mul,
    core::ops::Sub.sub
);
