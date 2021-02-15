use crate::vec::{Point3, Vec3};
use crate::ray::Ray;
use crate::hittable::{Hittable, HitRecord};

#[derive(Debug, Copy, Clone)]
pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Sphere {
        Sphere {center, radius}
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: Ray<f64>, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc: Vec3<f64> = r.origin - self.center;
        let a = r.direction.length_squared();
        let half_b = oc.dot(r.direction);
        let c = oc.length_squared() - self.radius * self.radius;

        let discrim = half_b * half_b -  a * c;
        if discrim < 0.0 {
            return false;
        }

        let sqrtd = discrim.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return false
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);

        return true;
    }
}
