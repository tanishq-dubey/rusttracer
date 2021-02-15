use rand::Rng;

// Faster alias
pub static INFINITY: f64 = f64::INFINITY;
pub static PI: f64 = 3.1415926535897932385;

#[inline]
pub fn degrees_to_radians(degrees: f64) -> f64 {
    return degrees * PI / 180.0;
}

#[inline]
pub fn random_f64() -> f64 {
    let mut rng = rand::thread_rng();
    return rng.gen::<f64>();
}

pub fn random_f64_range(min: f64, max: f64) -> f64 {
    return min + (max-min)*random_f64();
}

