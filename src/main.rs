use std::fs::File;
use std::path::Path;
use std::io::{self, Write};


mod vec;

fn main() {
    // Image
    
    const IMAGE_WIDTH: i64 = 512;
    const IMAGE_HEIGHT: i64 = 512;

    // Render
    let path = Path::new("output.ppm");
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("Could not create {}: {}", display, why),
        Ok(file) => file,
    };

    let mut outimg: String = String::from(format!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT));

    for j in (0..IMAGE_HEIGHT).rev() {
        print!("\rScanlines Remaining: {}", j);
        io::stdout().flush().unwrap();
        for i in 0..IMAGE_WIDTH {
            let r = (i as f64) / ((IMAGE_WIDTH - 1) as f64);
            let g = (j as f64) / ((IMAGE_HEIGHT - 1) as f64);
            let b = 0.25;

            let ir = (255.999 * r) as i64;
            let ig = (255.999 * g) as i64;
            let ib = (255.999 * b) as i64;

            outimg = format!("{}{} {} {}\n",outimg, ir, ig, ib);
        }
    }

    print!("\nDone Rendering\n");

    match file.write_all(outimg.as_bytes()) {
        Err(why) => panic!("Could not create {}: {}", display, why),
        Ok(_) => println!("Wrote image to {}", display),
    };
}
