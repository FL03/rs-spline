/*
    Appellation: knot <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use core::slice;

#[repr(transparent)]
pub struct Knots<T>(pub(crate) Vec<T>);

impl<T> Knots<T> {
    pub fn new(p: usize, points: usize) -> Self
    where
        T: PartialEq,
    {
        assert!(p > 0, "degree must be greater than 0");
        let len = p + points + 1;
        Self(Vec::with_capacity(len))
    }

    pub fn is_sorted(&self) -> bool
    where
        T: PartialOrd,
    {
        self.windows(2).all(|w| w[0] <= w[1])
    }

    pub fn iter(&self) -> slice::Iter<'_, T> {
        self.0.iter()
    }

    pub fn iter_mut(&mut self) -> slice::IterMut<'_, T> {
        self.0.iter_mut()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn push(&mut self, knot: T) {
        self.0.push(knot);
    }

    pub fn remove(&mut self, index: usize) -> T {
        self.0.remove(index)
    }

    pub fn windows(&self, size: usize) -> slice::Windows<'_, T> {
        self.0.windows(size)
    }
}

pub trait KnotVector<T> {
    fn check_knot_domain(&self) -> bool;
    fn degree(&self, points: usize) -> usize;
}

/*
 ************* Implementations *************
*/
impl<T> AsRef<[T]> for Knots<T> {
    fn as_ref(&self) -> &[T] {
        &self.0
    }
}

impl<T> AsMut<[T]> for Knots<T> {
    fn as_mut(&mut self) -> &mut [T] {
        &mut self.0
    }
}

impl<T> AsRef<Vec<T>> for Knots<T> {
    fn as_ref(&self) -> &Vec<T> {
        &self.0
    }
}

impl<T> AsMut<Vec<T>> for Knots<T> {
    fn as_mut(&mut self) -> &mut Vec<T> {
        &mut self.0
    }
}
impl<T> core::borrow::Borrow<Vec<T>> for Knots<T> {
    fn borrow(&self) -> &Vec<T> {
        &self.0
    }
}

impl<T> core::borrow::BorrowMut<Vec<T>> for Knots<T> {
    fn borrow_mut(&mut self) -> &mut Vec<T> {
        &mut self.0
    }
}

impl<T> core::ops::Deref for Knots<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Vec<T> {
        &self.0
    }
}

impl<T> core::ops::DerefMut for Knots<T> {
    fn deref_mut(&mut self) -> &mut Vec<T> {
        &mut self.0
    }
}
