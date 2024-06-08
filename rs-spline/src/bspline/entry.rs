/*
    Appellation: entry <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use core::marker::PhantomData;

pub struct Entry<'a, K, V = K> {
    pub(crate) knot: &'a K,
    pub(crate) point: &'a V,
    _marker: PhantomData<&'a ()>,
}

impl<'a, K, V> Entry<'a, K, V> {
    pub fn new(knot: &'a K, point: &'a V) -> Self {
        Self {
            knot,
            point,
            _marker: PhantomData,
        }
    }

    pub const fn knot(&self) -> &K {
        &self.knot
    }

    pub const fn point(&self) -> &V {
        &self.point
    }
}
