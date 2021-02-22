use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new() -> HitRecord {
        HitRecord {
            p: Point3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            t: 0.0,
            front_face: true,
        }
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.front_face = Vec3::dot(ray.direction, outward_normal) < 0.0;

        self.normal = outward_normal;
        if !self.front_face {
            self.normal = outward_normal.neg();
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}

#[derive(Clone, Copy)]
pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc: Vec3 = ray.origin.sub(self.center);

        let a = ray.direction.length_squared();
        let half_b = Vec3::dot(oc, ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return false;
        }

        let sqrtd: f64 = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root: f64 = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }

        rec.t = root;
        rec.p = ray.at(rec.t);
        let outward_normal: Vec3 = rec.p.sub(self.center).div_value(self.radius);
        rec.set_face_normal(ray, outward_normal);

        true
    }
}
