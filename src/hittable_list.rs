use crate::hittable::{Hittable, HitRecord};
use crate::ray::Ray;

pub struct HittableList<'a> {
    pub objects: Vec<&'a dyn Hittable>,
}

impl<'a> HittableList<'a> {
    pub fn new() -> HittableList<'a> {
        HittableList {objects: Vec::new()}
    }

    pub fn clear(mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object:&'a dyn Hittable) {
        self.objects.push(object);
    }
}

impl<'a> Hittable for HittableList<'a> {
    fn hit(&self, r: Ray<f64>, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut temp_rec: HitRecord = HitRecord::new();
        let mut hit_anything: bool = false;
        let mut closest_so_far: f64 = t_max;

        for object in self.objects.iter() {
            if object.hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec;
            }
        }

        return hit_anything;
    }
}
