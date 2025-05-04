pub struct Int2 {
    pub x: i32,
    pub y: i32,
}

impl Int2 {
    #[inline(always)]
    pub fn new(x: i32, y: i32) -> Self {
        Int2 { x, y }
    }

    #[inline(always)]
    pub fn to_string(&self) -> String {
        format!("Int2({}, {})", self.x, self.y)
    }
}

use std::ops::{Index, IndexMut};

impl Index<usize> for Int2 {
    type Output = i32;

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl IndexMut<usize> for Int2 {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => panic!("Index out of bounds"),
        }
    }
}