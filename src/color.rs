use crate::vec;
use crate::utils;

pub fn write_color(color: vec::Color, samples: i64) -> String {
    let mut r = color.x;
    let mut g = color.y;
    let mut b = color.z;

    let scale = 1.0 / samples as f64;
    r *= scale;
    g *= scale;
    b *= scale;

    return format!("{} {} {}\n", 
                   (256.0 * utils::clamp(r, 0.0, 0.999)) as i64,
                   (256.0 * utils::clamp(g, 0.0, 0.999)) as i64,
                   (256.0 * utils::clamp(b, 0.0, 0.999)) as i64);
}
