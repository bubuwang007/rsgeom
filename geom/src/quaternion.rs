#[derive(Debug, Clone, Copy)]
pub struct Quaternion<T = f64> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}
