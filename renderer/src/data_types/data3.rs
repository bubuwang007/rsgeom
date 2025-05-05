use super::data2::Data2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Data3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Data3<T> {
    #[inline(always)]
    pub fn new(x: T, y: T, z: T) -> Self {
        Data3 { x, y, z }
    }

    #[inline(always)]
    pub fn from_value(value: T) -> Self
    where
        T: Copy,
    {
        Data3 {
            x: value,
            y: value,
            z: value,
        }
    }

    #[inline(always)]
    pub fn from_data2(data2: &Data2<T>, z: T) -> Self
    where
        T: Copy,
    {
        Data3 {
            x: data2.x,
            y: data2.y,
            z,
        }
    }
}

impl<T: std::fmt::Display> std::fmt::Display for Data3<T> {
    #[inline(always)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::any::type_name;
        write!(
            f,
            "{}({}, {}, {})",
            type_name::<T>(),
            self.x,
            self.y,
            self.z
        )
    }
}

use std::ops::{Index, IndexMut};

impl<T> Index<usize> for Data3<T> {
    type Output = T;

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl<T> IndexMut<usize> for Data3<T> {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Index out of bounds"),
        }
    }
}
