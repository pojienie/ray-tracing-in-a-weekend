use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::{Color, Point3, Vec3};

pub trait Material {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> (Color, Ray);
}

pub struct EmptyMaterial {}

impl Material for EmptyMaterial {
    fn scatter(&self, _ray: &Ray, _record: &HitRecord) -> (Color, Ray) {
        let origin: Point3 = Vec3::new(0.0, 0.0, 0.0);
        let direction: Vec3 = Vec3::new(0.0, 0.0, 0.0);
        (Color::new(0.0, 0.0, 0.0), Ray::new(origin, direction))
    }
}

pub static EMPTY_MATERIAL: EmptyMaterial = EmptyMaterial {};

pub struct Lambertian {
    pub albedo: Color,
}

impl Lambertian {
    pub fn new(color: Color) -> Lambertian {
        Lambertian { albedo: color }
    }
}

fn random_in_hemisphere(normal: Vec3) -> Vec3 {
    let random_unit_vector: Vec3 = Vec3::random_unit_vector();
    if Vec3::dot(normal, random_unit_vector) > 0.0 {
        random_unit_vector
    } else {
        random_unit_vector.neg()
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, record: &HitRecord) -> (Color, Ray) {
        let r = random_in_hemisphere(record.normal);

        let target: Point3 = record.p.add(record.normal).add(r);
        let ray2: Ray = Ray::new(record.p, target.sub(record.p));
        (self.albedo, ray2)
    }
}
