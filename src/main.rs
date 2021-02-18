mod vec3;

use std::io::{self, Write};
use vec3::{Color};

fn main() {
    let width: i32 = 256;
    let height: i32 = 256;

    println!("P3");
    println!("{} {}", width, height);
    println!("255");

    let width_f64: f64 = width.into();
    let height_f64: f64 = height.into();
    for y in (0..height).rev() {
        eprint!("\rrow remaining {}    ", y);
        io::stderr().flush().unwrap();

        let y: f64 = y.into();

        for x in 0..width {
            let x: f64 = x.into();

            let pixel: Color = Color::new(x / width_f64, y / height_f64, 0.25);

            let (r, g, b) = pixel.get_i32();

            println!("{} {} {}", r, g, b);
        }
    }

    eprintln!("");
    eprintln!("done!");
}
