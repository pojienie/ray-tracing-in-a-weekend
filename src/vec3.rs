pub struct Vec3 {
    pub v0: f64, // R or X
    pub v1: f64, // G or Y
    pub v2: f64, // B or Z
}

impl Vec3 {
    pub fn new(v0: f64, v1: f64, v2: f64) -> Vec3 {
        Vec3 {
            v0: v0,
            v1: v1,
            v2: v2,
        }
    }

    pub fn get_i32(&self) -> (i32, i32, i32) {
        (
            (255.999 * self.v0) as i32,
            (255.999 * self.v1) as i32,
            (255.999 * self.v2) as i32,
        )
    }
}

pub type Color = Vec3;
