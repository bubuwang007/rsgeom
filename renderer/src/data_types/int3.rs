use crate::data_types::int2::Int2;

pub struct Int3 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Int3 {
    #[inline(always)]
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Int3 { x, y, z }
    }

    #[inline(always)]
    pub fn from_value(value: i32) -> Self {
        Int3 {
            x: value,
            y: value,
            z: value,
        }
    }

    #[inline(always)]
    pub fn from_int2(int2: &Int2, z: i32) -> Self {
        Int3 {
            x: int2.x,
            y: int2.y,
            z,
        }
    }

    #[inline(always)]
    pub fn to_string(&self) -> String {
        format!("Int3({}, {}, {})", self.x, self.y, self.z)
    }
}
