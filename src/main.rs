fn main() {
    let width: i32 = 256;
    let height: i32 = 256;

    println!("P3");
    println!("{} {}", width, height);
    println!("255");

    let width_f64: f64 = width.into();
    let height_f64: f64 = height.into();
    for y in (0..height).rev() {
        let y: f64 = y.into();

        for x in 0..width {
            let x: f64 = x.into();

            let r: f64 = x / width_f64;
            let g: f64 = y / height_f64;
            let b: f64 = 0.25;

            let r: i32 = (r * 255.999) as i32;
            let g: i32 = (g * 255.999) as i32;
            let b: i32 = (b * 255.999) as i32;

            println!("{} {} {}", r, g, b);
        }
    }
}
