use crate::vec::{Point3, Vec3};
use crate::ray::Ray;

#[derive(Copy, Clone)]
pub struct Camera {
    pub aspect_ratio: f64,
    pub viewport_height: f64,
    pub viewport_width: f64,
    pub focal_length: f64,
   
    origin: Point3,
    horizontal: Vec3<f64>,
    vertical: Vec3<f64>,
    lower_left_corner: Vec3<f64>,
}

impl Camera {
    pub fn new() -> Camera{
        let asp = 16.0 / 9.0;
        let vh = 2.0;
        let vw = asp * vh;
        let o = Point3::new(0.0, 0.0, 0.0);
        let h = Vec3::new(vw, 0.0, 0.0);
        let v = Vec3::new(0.0, vh, 0.0);

        Camera {
            aspect_ratio: asp,
            viewport_height: vh,
            viewport_width: vw,
            focal_length: 1.0,

            origin: o,
            horizontal: h,
            vertical: v,
            lower_left_corner: o - h/2.0 - v/2.0 - Vec3::new(0.0, 0.0, 1.0),
        }
    }

    pub fn get_ray(self, u: f64, v: f64) -> Ray<f64> {
        Ray::new(self.origin, self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin)
    }
}
