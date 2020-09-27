#[derive(Debug, Copy)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vector3 {
        Vector3 { x, y, z }
    }
}
impl Clone for Vector3 {
    fn clone(&self) -> Vector3 {
        *self
    }
}
