#[derive(Clone, Copy, Debug, Default)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    /// Update the position and return the velocity
    pub fn update(&mut self, x: f32, y: f32, z: f32, delta_t: f32) -> Self {
        let velocity = Vector3::new(
            (x - self.x) / delta_t,
            (y - self.y) / delta_t,
            (z - self.z) / delta_t,
        );
        self.x = x;
        self.y = y;
        self.z = z;
        velocity
    }
}

impl From<(f32, f32, f32)> for Vector3 {
    fn from(tuple: (f32, f32, f32)) -> Self {
        Self {
            x: tuple.0,
            y: tuple.1,
            z: tuple.2,
        }
    }
}
