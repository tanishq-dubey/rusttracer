use std::fs::File;
use std::path::Path;
use std::io::{self, Write, stdin, stdout, Read};

use vec::{Color, Point3, Vec3};
use ray::Ray;
use hittable::{HitRecord, Hittable};
use hittable_list::HittableList;
use sphere::Sphere;
use camera::Camera;

mod vec;
mod color;
mod ray;
mod hittable;
mod hittable_list;
mod sphere;
mod utils;
mod camera;

fn ray_color(r: ray::Ray<f64>, world:&impl Hittable) -> vec::Color {
    let mut rec: HitRecord = HitRecord::new();

    if world.hit(r, 0.0, utils::INFINITY, &mut rec) {
        let target: Point3 = rec.p + rec.normal + Vec3::new_random_in_unit_sphere();
        return 0.5 * ray_color(Ray::new(rec.p, target - rec.p), world);
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
    const SAMPLES_PER_PIXEL: i64 = 100;

    // World
    let mut world: HittableList = HittableList::new();
    let s1 = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5);
    let s2 = Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0);
    world.add(&s1);
    world.add(&s2);


    // Camera
    let cam: Camera = Camera::new();

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

            let mut color: Color = Color::new(0.0, 0.0, 0.0);
            for s in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + utils::random_f64()) / ((IMAGE_WIDTH - 1) as f64);
                let v = (j as f64 + utils::random_f64()) / ((IMAGE_HEIGHT - 1) as f64);
                let r = cam.get_ray(u, v);
                color = color + ray_color(r, &world);
            }

            let cstring = color::write_color(color, SAMPLES_PER_PIXEL);
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
