use std::{f64::INFINITY, fs::File};
use std::path::Path;
use std::io::{self, Write, stdin, stdout, Read};

use ray::Ray;
use vec::{Color, Point3, Vec3};
use hittable::{HitRecord, Hittable};
use hittable_list::HittableList;
use sphere::Sphere;


mod vec;
mod color;
mod ray;
mod hittable;
mod hittable_list;
mod sphere;

fn ray_color(r: ray::Ray<f64>, world:&impl Hittable) -> vec::Color {
    let mut rec: HitRecord = HitRecord::new();

    if world.hit(r, 0.0, INFINITY, &mut rec) {
         let ret = 0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0));
         return ret;
    }

    let unit_direction: vec::Vec3<f64> = r.direction.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    return (1.0 - t) * vec::Color::new(1.0, 1.0, 1.0) + t * vec::Color::new(0.5, 0.7, 1.0);
}

fn pause() {
    let mut stdout = stdout();
    stdout.write(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: i64 = 400;
    const IMAGE_HEIGHT: i64 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i64;

    // World
    let mut world: HittableList = HittableList::new();
    let s1 = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5);
    let s2 = Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0);
    world.add(&s1);
    world.add(&s2);


    // Camera
    let viewport_height: f64 = 2.0;
    let viewport_wdith: f64 = ASPECT_RATIO * viewport_height;
    let focal_length: f64 = 1.0;

    let origin = vec::Point3::new(0.0, 0.0, 0.0);
    let horizontal = vec::Vec3::new(viewport_wdith, 0.0, 0.0);
    let vertical = vec::Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - vec::Vec3::new(0.0, 0.0, focal_length);

    // File Start
    let path = Path::new("output.ppm");
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("Could not create {}: {}", display, why),
        Ok(file) => file,
    };

    let mut outimg: String = String::from(format!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT));

    // Render
    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            print!("\rj: {}\ti: {}", j, i);
            io::stdout().flush().unwrap();
            let u = (i as f64) / ((IMAGE_WIDTH - 1) as f64);
            let v = (j as f64) / ((IMAGE_HEIGHT - 1) as f64);

            let r = ray::Ray::new(origin, lower_left_corner + u * horizontal + v * vertical - origin);
            let c = ray_color(r, &world);

            let cstring = color::write_color(c);
            outimg = format!("{}{}",outimg, cstring);
        }
    }

    print!("\nDone Rendering\n");

    // File End
    match file.write_all(outimg.as_bytes()) {
        Err(why) => panic!("Could not create {}: {}", display, why),
        Ok(_) => println!("Wrote image to {}", display),
    };
}
