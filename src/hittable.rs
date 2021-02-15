use crate::vec::{Point3, Vec3};
use crate::ray::Ray;

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3<f64>,
    pub t: f64,
}

trait Hittable {
    fn hit(self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}
