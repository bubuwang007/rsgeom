use super::data3::Data3;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Data4<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

impl <T> Data4<T> {
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
}