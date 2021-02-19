use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub trait Hittable {
    fn hit(&self, ray: &Ray) -> f64;
}

#[derive(Clone, Copy)]
pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray) -> f64 {
        let oc: Vec3 = ray.origin.sub(self.center);

        let a = ray.direction.length_squared();
        let half_b = Vec3::dot(oc, ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return -1.0;
        }

        (-half_b - discriminant.sqrt()) / a
    }
}
