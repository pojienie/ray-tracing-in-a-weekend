mod camera;
mod hittable;
mod ray;
mod vec3;

use camera::Camera;
use hittable::{HitRecord, Hittable, Sphere};
use rand::prelude::*;
use ray::Ray;
use std::io::{self, Write};
use std::vec::Vec;
use vec3::{Color, Point3, Vec3};

fn random_in_hemisphere(normal: Vec3) -> Vec3 {
    let random_unit_vector: Vec3 = Vec3::random_unit_vector();
    if Vec3::dot(normal, random_unit_vector) > 0.0 {
        random_unit_vector
    } else {
        random_unit_vector.neg()
    }
}

fn ray_color(ray: &Ray, hittables: &Vec<impl Hittable>, depth: i32) -> Color {
    if depth == 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    let mut hit_record: HitRecord = HitRecord::new();
    let mut closest_t: f64 = std::f64::INFINITY;
    let mut hit_anything: bool = false;

    for hittable in hittables {
        let mut temp_hit_record: HitRecord = HitRecord::new();
        let hit: bool = hittable.hit(ray, 0.001, closest_t, &mut temp_hit_record);
        if hit {
            hit_anything = true;
            closest_t = hit_record.t;
            hit_record = temp_hit_record;
        }
    }

    if hit_anything {
        let r = random_in_hemisphere(hit_record.normal);

        let target: Point3 = hit_record.p.add(hit_record.normal).add(r);
        let ray2: Ray = Ray::new(hit_record.p, target.sub(hit_record.p));
        let color: Color = ray_color(&ray2, hittables, depth - 1);
        return color.mul_value(0.5);
    }

    let unit_direction: Vec3 = ray.direction.unit();
    let t: f64 = 0.5 * (unit_direction.v1 + 1.0);

    let color1 = Color::new(1.0, 1.0, 1.0).mul_value(1.0 - t);
    let color2 = Color::new(0.5, 0.7, 1.0).mul_value(t);
    color1.add(color2)
}

fn main() {
    let max_depth: i32 = 50;

    // image
    let aspect_ratio: f64 = 16.0 / 9.0;
    let width: i32 = 400;
    let width_f64: f64 = width.into();
    let height_f64: f64 = width_f64 / aspect_ratio;
    let height: i32 = height_f64 as i32;
    let samples_per_pixel = 100;

    // world
    let center: Point3 = Point3::new(0.0, 0.0, -1.0);
    let radius: f64 = 0.5;
    let sphere1: Sphere = Sphere {
        center: center,
        radius: radius,
    };

    let center: Point3 = Point3::new(0.0, -100.5, -1.0);
    let radius: f64 = 100.0;
    let sphere2: Sphere = Sphere {
        center: center,
        radius: radius,
    };

    let spheres = vec![sphere1, sphere2];

    // camera
    let camera = Camera::new();

    // random
    let mut rng = rand::thread_rng();

    // render
    println!("P3");
    println!("{} {}", width, height);
    println!("255");
    for y in (0..height).rev() {
        eprint!("\rrow remaining {}    ", y);
        io::stderr().flush().unwrap();

        let y: f64 = y.into();

        for x in 0..width {
            let x: f64 = x.into();

            let mut pixel: Color = Color::new(0.0, 0.0, 0.0);
            for _i in 0..samples_per_pixel {
                let du: f64 = rng.gen();
                let dv: f64 = rng.gen();

                let u: f64 = (x + du) / (width_f64 - 1.0);
                let v: f64 = (y + dv) / (height_f64 - 1.0);

                let ray: Ray = camera.get_ray(u, v);
                pixel = pixel.add(ray_color(&ray, &spheres, max_depth));
            }
            pixel = pixel.div_value(samples_per_pixel.into());

            let (r, g, b) = pixel.sqrt().clamp(0.0, 0.999).get_i32();

            println!("{} {} {}", r, g, b);
        }
    }

    eprintln!("");
    eprintln!("done!");
}
