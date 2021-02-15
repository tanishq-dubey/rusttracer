use crate::vec::{Point3, Vec3};
use crate::ray::Ray;

#[derive(Debug, Copy, Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3<f64>,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new() -> HitRecord {
       HitRecord{p: Point3::new(0.0, 0.0, 0.0),
                 normal: Vec3::new(0.0, 0.0, 0.0),
                 t: 0.0,
                 front_face: false} 
    }

    pub fn set_face_normal(&mut self, r: Ray<f64>, outward_normal: Vec3<f64>) {
        self.front_face = r.direction.dot(outward_normal) < 0.0;
        if self.front_face {
            self.normal = outward_normal;
        } else {
            self.normal = outward_normal * -1.0;
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: Ray<f64>, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}
