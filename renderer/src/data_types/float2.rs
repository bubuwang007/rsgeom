pub struct Float2 {
    pub x: f32,
    pub y: f32,
}

impl Float2 {
    #[inline(always)]
    pub fn new(x: f32, y: f32) -> Self {
        Float2 { x, y }
    }

    #[inline(always)]
    pub fn to_string(&self) -> String {
        format!("Float2({}, {})", self.x, self.y)
    }

}

use std::ops::{Index, IndexMut};

impl Index<usize> for Float2 {
    type Output = f32;

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl IndexMut<usize> for Float2 {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => panic!("Index out of bounds"),
        }
    }
}
