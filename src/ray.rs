use crate::vec;

#[derive(Copy, Clone, Debug)]
pub struct Ray<S> {
    pub origin: vec::Point3,
    pub direction: vec::Vec3<S>,
}

impl Ray<f64> {
    pub fn new(origin: vec::Point3, direction: vec::Vec3<f64>) -> Ray<f64> {
        Ray {origin, direction}
    }

    pub fn at(self, t: f64) -> vec::Point3 {
        self.origin + t * self.direction
    }
}
