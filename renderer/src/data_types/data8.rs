use super::data4::Data4;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Data8<T> {
    pub a: T,
    pub b: T,
    pub c: T,
    pub d: T,
    pub e: T,
    pub f: T,
    pub g: T,
    pub h: T,
}

impl<T> Data8<T> {
    #[inline(always)]
    pub fn new(a: T, b: T, c: T, d: T, e: T, f: T, g: T, h: T) -> Self {
        Data8 {
            a,
            b,
            c,
            d,
            e,
            f,
            g,
            h,
        }
    }

    #[inline(always)]
    pub fn from_value(value: T) -> Self
    where
        T: Copy,
    {
        Data8 {
            a: value,
            b: value,
            c: value,
            d: value,
            e: value,
            f: value,
            g: value,
            h: value,
        }
    }

    #[inline(always)]
    pub fn from_2data4(&self, data4_1: &Data4<T>, data4_2: &Data4<T>) -> Self
    where
        T: Copy,
    {
        Data8 {
            a: data4_1.x,
            b: data4_1.y,
            c: data4_1.z,
            d: data4_1.w,
            e: data4_2.x,
            f: data4_2.y,
            g: data4_2.z,
            h: data4_2.w,
        }
    }
}

impl<T: std::fmt::Display> Data8<T> {
    #[inline(always)]
    pub fn print(&self, lable: &str) {
        println!(
            "{}: {} {} {} {} {} {} {} {}",
            lable, self.a, self.b, self.c, self.d, self.e, self.f, self.g, self.h
        );
    }
}

impl<T: std::fmt::Display> std::fmt::Display for Data8<T> {
    #[inline(always)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::any::type_name;
        write!(
            f,
            "{}({}, {}, {}, {}, {}, {}, {}, {})",
            type_name::<T>(),
            self.a,
            self.b,
            self.c,
            self.d,
            self.e,
            self.f,
            self.g,
            self.h
        )
    }
}

impl Data8<f32> {
    #[inline(always)]
    pub fn to_i32_8(&self) -> Data8<i32> {
        Data8 {
            a: self.a as i32,
            b: self.b as i32,
            c: self.c as i32,
            d: self.d as i32,
            e: self.e as i32,
            f: self.f as i32,
            g: self.g as i32,
            h: self.h as i32,
        }
    }
}

use std::ops::{Index, IndexMut};

impl<T> Index<usize> for Data8<T> {
    type Output = T;

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.a,
            1 => &self.b,
            2 => &self.c,
            3 => &self.d,
            4 => &self.e,
            5 => &self.f,
            6 => &self.g,
            7 => &self.h,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl<T> IndexMut<usize> for Data8<T> {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.a,
            1 => &mut self.b,
            2 => &mut self.c,
            3 => &mut self.d,
            4 => &mut self.e,
            5 => &mut self.f,
            6 => &mut self.g,
            7 => &mut self.h,
            _ => panic!("Index out of bounds"),
        }
    }
}
