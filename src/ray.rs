use crate::vec;

pub struct Ray<S> {
    pub origin: vec::Point3<S>,
    pub direction: vec::Vec3<S>,
}

impl Ray<f64> {
    pub fn new(origin: vec::Point3<f64>, direction: vec::Vec3<f64>) -> Ray<f64> {
        Ray {origin, direction}
    }

    pub fn at(self, t: f64) -> vec::Point3<f64> {
        self.origin + t * dir;
    }
}
