use super::data3::Data3;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Data4<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

impl<T> Data4<T> {
    #[inline(always)]
    pub fn new(x: T, y: T, z: T, w: T) -> Self {
        Data4 { x, y, z, w }
    }

    #[inline(always)]
    pub fn from_value(value: T) -> Self
    where
        T: Copy,
    {
        Data4 {
            x: value,
            y: value,
            z: value,
            w: value,
        }
    }

    #[inline(always)]
    pub fn from_data3(data3: &Data3<T>, w: T) -> Self
    where
        T: Copy,
    {
        Data4 {
            x: data3.x,
            y: data3.y,
            z: data3.z,
            w,
        }
    }

    #[inline(always)]
    pub fn to_data3(&self) -> Data3<T>
    where
        T: Copy,
    {
        Data3 {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}

impl Data4<f32> {
    #[inline(always)]
    pub fn from_i32(data3: i32) -> Self {
        Data4 {
            x: data3 as f32,
            y: data3 as f32,
            z: data3 as f32,
            w: data3 as f32,
        }
    }

    #[inline(always)]
    pub fn to_i32_4(&self) -> Data4<i32> {
        Data4 {
            x: self.x as i32,
            y: self.y as i32,
            z: self.z as i32,
            w: self.w as i32,
        }
    }
}

impl<T: std::fmt::Display> Data4<T> {
    #[inline(always)]
    pub fn print(&self, lable: &str) {
        println!("{}: {} {} {} {}", lable, self.x, self.y, self.z, self.w);
    }
}

impl<T: std::fmt::Display> std::fmt::Display for Data4<T> {
    #[inline(always)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::any::type_name;
        write!(
            f,
            "{}({}, {}, {}, {})",
            type_name::<T>(),
            self.x,
            self.y,
            self.z,
            self.w
        )
    }
}

use std::ops::{Index, IndexMut};

impl<T> Index<usize> for Data4<T> {
    type Output = T;

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            3 => &self.w,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl<T> IndexMut<usize> for Data4<T> {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            3 => &mut self.w,
            _ => panic!("Index out of bounds"),
        }
    }
}
