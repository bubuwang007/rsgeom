#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Data2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Data2<T> {
    #[inline(always)]
    pub fn new(x: T, y: T) -> Self {
        Data2 { x, y }
    }
}

impl<T: std::fmt::Display> std::fmt::Display for Data2<T> {
    #[inline(always)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::any::type_name;
        write!(f, "{}({}, {})", type_name::<T>(), self.x, self.y)
    }
}

use std::ops::{Index, IndexMut};

impl<T> Index<usize> for Data2<T> {
    type Output = T;

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl<T> IndexMut<usize> for Data2<T> {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => panic!("Index out of bounds"),
        }
    }
}
