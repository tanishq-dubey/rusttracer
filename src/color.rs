use crate::vec;

pub fn write_color(color: vec::Color) -> String {
    let ir = (255.999 * color.x) as i64;
    let ig = (255.999 * color.y) as i64;
    let ib = (255.999 * color.z) as i64;

    return format!("{} {} {}\n", ir, ig, ib);
}
