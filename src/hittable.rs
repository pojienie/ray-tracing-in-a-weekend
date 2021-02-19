use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub trait Hittable {
    fn hit(&self, ray: &Ray) -> bool;
}

#[derive(Clone, Copy)]
pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray) -> bool {
        let oc: Vec3 = ray.origin.sub(self.center);

        let a = Vec3::dot(ray.direction, ray.direction);
        let b = Vec3::dot(oc, ray.direction) * 2.0;
        let c = Vec3::dot(oc, oc) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;
        discriminant > 0.0
    }
}
