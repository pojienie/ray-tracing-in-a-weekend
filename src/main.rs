mod hittable;
mod ray;
mod vec3;

use hittable::{Hittable, Sphere};
use ray::Ray;
use std::io::{self, Write};
use vec3::{Color, Point3, Vec3};

fn ray_color(ray: &Ray, hittable: &impl Hittable) -> Color {
    let t: f64 = hittable.hit(ray);
    if t > 0.0 {
        let n: Vec3 = ray.at(t).sub(Vec3::new(0.0, 0.0, -1.0));
        return Color::new(n.v0 + 1.0, n.v1 + 1.0, n.v2 + 1.0).mul_value(0.5);
    }

    let unit_direction: Vec3 = ray.direction.unit();
    let t: f64 = 0.5 * (unit_direction.v1 + 1.0);

    let color1 = Color::new(1.0, 1.0, 1.0).mul_value(1.0 - t);
    let color2 = Color::new(0.5, 0.7, 1.0).mul_value(t);
    color1.add(color2)
}

fn main() {
    // image
    let aspect_ratio: f64 = 16.0 / 9.0;
    let width: i32 = 400;
    let width_f64: f64 = width.into();
    let height_f64: f64 = width_f64 / aspect_ratio;
    let height: i32 = height_f64 as i32;

    let center: Point3 = Point3::new(0.0, 0.0, -1.0);
    let radius: f64 = 0.5;
    let sphere: Sphere = Sphere {
        center: center,
        radius: radius,
    };

    // camera
    let viewport_height: f64 = 2.0;
    let viewport_width: f64 = viewport_height * aspect_ratio;
    let focal_length: f64 = 1.0;

    let origin: Point3 = Point3::new(0.0, 0.0, 0.0);
    let horizontal: Vec3 = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical: Vec3 = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin
        .sub(horizontal.div_value(2.0))
        .sub(vertical.div_value(2.0))
        .sub(Vec3::new(0.0, 0.0, focal_length));

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

            let u: f64 = x / (width_f64 - 1.0);
            let v: f64 = y / (height_f64 - 1.0);

            let p = lower_left_corner
                .add(horizontal.mul_value(u))
                .add(vertical.mul_value(v))
                .sub(origin);
            let ray: Ray = Ray::new(origin, p);
            let pixel: Color = ray_color(&ray, &sphere);

            let (r, g, b) = pixel.get_i32();

            println!("{} {} {}", r, g, b);
        }
    }

    eprintln!("");
    eprintln!("done!");
}
