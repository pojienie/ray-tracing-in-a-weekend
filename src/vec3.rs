#[derive(Clone, Copy)]
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

    pub fn add(&self, v: Vec3) -> Vec3 {
        Vec3 {
            v0: self.v0 + v.v0,
            v1: self.v1 + v.v1,
            v2: self.v2 + v.v2,
        }
    }

    pub fn sub(&self, v: Vec3) -> Vec3 {
        Vec3 {
            v0: self.v0 - v.v0,
            v1: self.v1 - v.v1,
            v2: self.v2 - v.v2,
        }
    }

    pub fn mul_value(&self, v: f64) -> Vec3 {
        Vec3 {
            v0: self.v0 * v,
            v1: self.v1 * v,
            v2: self.v2 * v,
        }
    }

    pub fn div_value(&self, v: f64) -> Vec3 {
        Vec3 {
            v0: self.v0 / v,
            v1: self.v1 / v,
            v2: self.v2 / v,
        }
    }

    pub fn length(&self) -> f64 {
        let v0 = self.v0 * self.v0;
        let v1 = self.v1 * self.v1;
        let v2 = self.v2 * self.v2;
        (v0 + v1 + v2).sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        let v0 = self.v0 * self.v0;
        let v1 = self.v1 * self.v1;
        let v2 = self.v2 * self.v2;
        v0 + v1 + v2
    }

    pub fn unit(&self) -> Vec3 {
        self.div_value(self.length())
    }

    pub fn dot(v: Vec3, u: Vec3) -> f64 {
        let v0 = v.v0 * u.v0;
        let v1 = v.v1 * u.v1;
        let v2 = v.v2 * u.v2;
        v0 + v1 + v2
    }
}

pub type Color = Vec3;
pub type Point3 = Vec3;
